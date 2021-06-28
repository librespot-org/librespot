use std::collections::HashMap;
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use futures_util::FutureExt;
use protobuf::Message;
use tokio::sync::{mpsc, oneshot};

use crate::protocol;
use crate::util::SeqGenerator;

mod types;
pub use self::types::*;

mod sender;
pub use self::sender::MercurySender;

component! {
    MercuryManager : MercuryManagerInner {
        sequence: SeqGenerator<u64> = SeqGenerator::new(0),
        pending: HashMap<Vec<u8>, MercuryPending> = HashMap::new(),
        subscriptions: Vec<(String, mpsc::UnboundedSender<MercuryResponse>)> = Vec::new(),
        invalid: bool = false,
    }
}

pub struct MercuryPending {
    parts: Vec<Vec<u8>>,
    partial: Option<Vec<u8>>,
    callback: Option<oneshot::Sender<Result<MercuryResponse, MercuryError>>>,
}

pub struct MercuryFuture<T> {
    receiver: oneshot::Receiver<Result<T, MercuryError>>,
}

impl<T> Future for MercuryFuture<T> {
    type Output = Result<T, MercuryError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.receiver.poll_unpin(cx).map_err(|_| MercuryError)?
    }
}

impl MercuryManager {
    fn next_seq(&self) -> Vec<u8> {
        let mut seq = vec![0u8; 8];
        BigEndian::write_u64(&mut seq, self.lock(|inner| inner.sequence.get()));
        seq
    }

    fn request(&self, req: MercuryRequest) -> MercuryFuture<MercuryResponse> {
        let (tx, rx) = oneshot::channel();

        let pending = MercuryPending {
            parts: Vec::new(),
            partial: None,
            callback: Some(tx),
        };

        let seq = self.next_seq();
        self.lock(|inner| {
            if !inner.invalid {
                inner.pending.insert(seq.clone(), pending);
            }
        });

        let cmd = req.method.command();
        let data = req.encode(&seq);

        self.session().send_packet(cmd, data);
        MercuryFuture { receiver: rx }
    }

    pub fn get<T: Into<String>>(&self, uri: T) -> MercuryFuture<MercuryResponse> {
        self.request(MercuryRequest {
            method: MercuryMethod::Get,
            uri: uri.into(),
            content_type: None,
            payload: Vec::new(),
        })
    }

    pub fn send<T: Into<String>>(&self, uri: T, data: Vec<u8>) -> MercuryFuture<MercuryResponse> {
        self.request(MercuryRequest {
            method: MercuryMethod::Send,
            uri: uri.into(),
            content_type: None,
            payload: vec![data],
        })
    }

    pub fn sender<T: Into<String>>(&self, uri: T) -> MercurySender {
        MercurySender::new(self.clone(), uri.into())
    }

    pub fn subscribe<T: Into<String>>(
        &self,
        uri: T,
    ) -> impl Future<Output = Result<mpsc::UnboundedReceiver<MercuryResponse>, MercuryError>> + 'static
    {
        let uri = uri.into();
        let request = self.request(MercuryRequest {
            method: MercuryMethod::Sub,
            uri: uri.clone(),
            content_type: None,
            payload: Vec::new(),
        });

        let manager = self.clone();
        async move {
            let response = request.await?;

            let (tx, rx) = mpsc::unbounded_channel();

            manager.lock(move |inner| {
                if !inner.invalid {
                    debug!("subscribed uri={} count={}", uri, response.payload.len());
                    if !response.payload.is_empty() {
                        // Old subscription protocol, watch the provided list of URIs
                        for sub in response.payload {
                            let mut sub =
                                protocol::pubsub::Subscription::parse_from_bytes(&sub).unwrap();
                            let sub_uri = sub.take_uri();

                            debug!("subscribed sub_uri={}", sub_uri);

                            inner.subscriptions.push((sub_uri, tx.clone()));
                        }
                    } else {
                        // New subscription protocol, watch the requested URI
                        inner.subscriptions.push((uri, tx));
                    }
                }
            });

            Ok(rx)
        }
    }

    pub(crate) fn dispatch(&self, cmd: u8, mut data: Bytes) {
        let seq_len = BigEndian::read_u16(data.split_to(2).as_ref()) as usize;
        let seq = data.split_to(seq_len).as_ref().to_owned();

        let flags = data.split_to(1).as_ref()[0];
        let count = BigEndian::read_u16(data.split_to(2).as_ref()) as usize;

        let pending = self.lock(|inner| inner.pending.remove(&seq));

        let mut pending = match pending {
            Some(pending) => pending,
            None if cmd == 0xb5 => MercuryPending {
                parts: Vec::new(),
                partial: None,
                callback: None,
            },
            None => {
                warn!("Ignore seq {:?} cmd {:x}", seq, cmd);
                return;
            }
        };

        for i in 0..count {
            let mut part = Self::parse_part(&mut data);
            if let Some(mut partial) = mem::replace(&mut pending.partial, None) {
                partial.extend_from_slice(&part);
                part = partial;
            }

            if i == count - 1 && (flags == 2) {
                pending.partial = Some(part)
            } else {
                pending.parts.push(part);
            }
        }

        if flags == 0x1 {
            self.complete_request(cmd, pending);
        } else {
            self.lock(move |inner| inner.pending.insert(seq, pending));
        }
    }

    fn parse_part(data: &mut Bytes) -> Vec<u8> {
        let size = BigEndian::read_u16(data.split_to(2).as_ref()) as usize;
        data.split_to(size).as_ref().to_owned()
    }

    fn complete_request(&self, cmd: u8, mut pending: MercuryPending) {
        let header_data = pending.parts.remove(0);
        let header = protocol::mercury::Header::parse_from_bytes(&header_data).unwrap();

        let response = MercuryResponse {
            uri: header.get_uri().to_string(),
            status_code: header.get_status_code(),
            payload: pending.parts,
        };

        if response.status_code >= 500 {
            panic!("Spotify servers returned an error. Restart librespot.");
        } else if response.status_code >= 400 {
            warn!("error {} for uri {}", response.status_code, &response.uri);
            if let Some(cb) = pending.callback {
                let _ = cb.send(Err(MercuryError));
            }
        } else if cmd == 0xb5 {
            self.lock(|inner| {
                let mut found = false;

                // TODO: This is just a workaround to make utf-8 encoded usernames work.
                // A better solution would be to use an uri struct and urlencode it directly
                // before sending while saving the subscription under its unencoded form.
                let mut uri_split = response.uri.split('/');

                let encoded_uri = std::iter::once(uri_split.next().unwrap().to_string())
                    .chain(uri_split.map(|component| {
                        form_urlencoded::byte_serialize(component.as_bytes()).collect::<String>()
                    }))
                    .collect::<Vec<String>>()
                    .join("/");

                inner.subscriptions.retain(|&(ref prefix, ref sub)| {
                    if encoded_uri.starts_with(prefix) {
                        found = true;

                        // if send fails, remove from list of subs
                        // TODO: send unsub message
                        sub.send(response.clone()).is_ok()
                    } else {
                        // URI doesn't match
                        true
                    }
                });

                if !found {
                    debug!("unknown subscription uri={}", response.uri);
                }
            })
        } else if let Some(cb) = pending.callback {
            let _ = cb.send(Ok(response));
        }
    }

    pub(crate) fn shutdown(&self) {
        self.lock(|inner| {
            inner.invalid = true;
            // destroy the sending halves of the channels to signal everyone who is waiting for something.
            inner.pending.clear();
            inner.subscriptions.clear();
        });
    }
}
