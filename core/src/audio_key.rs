use std::{collections::HashMap, io::Write, time::Duration};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use bytes::Bytes;
use thiserror::Error;
use tokio::sync::oneshot;

use crate::{Error, FileId, SpotifyId, packet::PacketType, util::SeqGenerator};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct AudioKey(pub [u8; 16]);

#[derive(Debug, Error)]
pub enum AudioKeyError {
    #[error("audio key error {0:#06x}")]
    AesKey(u16),
    #[error("other end of channel disconnected")]
    Channel,
    #[error("unexpected packet type {0}")]
    Packet(u8),
    #[error("sequence {0} not pending")]
    Sequence(u32),
    #[error("audio key response timeout")]
    Timeout,
}

impl From<AudioKeyError> for Error {
    fn from(err: AudioKeyError) -> Self {
        match err {
            AudioKeyError::AesKey(_) => Error::unavailable(err),
            AudioKeyError::Channel => Error::aborted(err),
            AudioKeyError::Sequence(_) => Error::aborted(err),
            AudioKeyError::Packet(_) => Error::unimplemented(err),
            AudioKeyError::Timeout => Error::aborted(err),
        }
    }
}

component! {
    AudioKeyManager : AudioKeyManagerInner {
        sequence: SeqGenerator<u32> = SeqGenerator::new(0),
        pending: HashMap<u32, oneshot::Sender<Result<AudioKey, AudioKeyError>>> = HashMap::new(),
    }
}

// Server error code for transient denials that succeed when the same request
// is retried (issue #1649); all other codes are returned to the caller as-is.
const AUDIO_KEY_ERROR_TRANSIENT: u16 = 0x0002;

// Removes its sequence from the pending map when the attempt ends or the
// request future is dropped.
struct PendingGuard {
    manager: AudioKeyManager,
    seq: u32,
}

impl Drop for PendingGuard {
    fn drop(&mut self) {
        self.manager.lock(|inner| {
            inner.pending.remove(&self.seq);
        });
    }
}

impl AudioKeyManager {
    pub(crate) fn dispatch(&self, cmd: PacketType, mut data: Bytes) -> Result<(), Error> {
        let seq = BigEndian::read_u32(data.split_to(4).as_ref());

        let sender = self
            .lock(|inner| inner.pending.remove(&seq))
            .ok_or(AudioKeyError::Sequence(seq))?;

        match cmd {
            PacketType::AesKey => {
                let mut key = [0u8; 16];
                key.copy_from_slice(data.as_ref());
                sender
                    .send(Ok(AudioKey(key)))
                    .map_err(|_| AudioKeyError::Channel)?
            }
            PacketType::AesKeyError => {
                let code = BigEndian::read_u16(data.as_ref());
                error!(
                    "error audio key {:x} {:x}",
                    data.as_ref()[0],
                    data.as_ref()[1]
                );
                sender
                    .send(Err(AudioKeyError::AesKey(code)))
                    .map_err(|_| AudioKeyError::Channel)?
            }
            _ => {
                trace!("Did not expect {cmd:?} AES key packet with data {data:#?}");
                return Err(AudioKeyError::Packet(cmd as u8).into());
            }
        }

        Ok(())
    }

    pub async fn request(&self, track: SpotifyId, file: FileId) -> Result<AudioKey, Error> {
        const KEY_RESPONSE_TIMEOUT: Duration = Duration::from_millis(1500);
        const KEY_REQUEST_RETRIES: u32 = 3;
        const RETRY_DELAY: Duration = Duration::from_secs(1);

        let mut last_err: Option<AudioKeyError> = None;

        for attempt in 0..KEY_REQUEST_RETRIES {
            let (tx, rx) = oneshot::channel();

            let seq = self.lock(move |inner| {
                let seq = inner.sequence.get();
                inner.pending.insert(seq, tx);
                seq
            });

            let _guard = PendingGuard {
                manager: self.clone(),
                seq,
            };

            self.send_key_request(seq, track, file)?;

            match tokio::time::timeout(KEY_RESPONSE_TIMEOUT, rx).await {
                Ok(Ok(Ok(key))) => return Ok(key),
                Ok(Ok(Err(AudioKeyError::AesKey(code)))) => {
                    // Only the transient code is retried; permanent denials
                    // and no-key responses for unencrypted files return
                    // immediately so the normal playback path stays fast.
                    if code != AUDIO_KEY_ERROR_TRANSIENT {
                        return Err(AudioKeyError::AesKey(code).into());
                    }
                    last_err = Some(AudioKeyError::AesKey(code));
                }
                Ok(Ok(Err(err))) => return Err(err.into()),
                Ok(Err(_)) => last_err = Some(AudioKeyError::Channel),
                Err(_) => {
                    error!("Audio key response timeout");
                    last_err = Some(AudioKeyError::Timeout);
                }
            }

            if attempt + 1 < KEY_REQUEST_RETRIES {
                warn!(
                    "audio key request failed, retrying in {RETRY_DELAY:?} (attempt {}/{KEY_REQUEST_RETRIES})",
                    attempt + 1
                );
                tokio::time::sleep(RETRY_DELAY).await;
            }
        }

        Err(last_err.unwrap_or(AudioKeyError::Timeout).into())
    }

    fn send_key_request(&self, seq: u32, track: SpotifyId, file: FileId) -> Result<(), Error> {
        let mut data: Vec<u8> = Vec::new();
        data.write_all(&file.0)?;
        data.write_all(&track.to_raw())?;
        data.write_u32::<BigEndian>(seq)?;
        data.write_u16::<BigEndian>(0x0000)?;

        self.session().send_packet(PacketType::RequestKey, data)
    }
}
