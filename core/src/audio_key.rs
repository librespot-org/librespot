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
    #[error("audio key error")]
    AesKey,
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
            AudioKeyError::AesKey => Error::unavailable(err),
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
        pending: HashMap<u32, PendingKey> = HashMap::new(),
        // Counts the connections to the server, so a request can tell that it was sent
        // again on a new one.
        connection: u32 = 0,
        reconnecting: bool = false,
    }
}

struct PendingKey {
    track: SpotifyId,
    file: FileId,
    sender: oneshot::Sender<Result<AudioKey, Error>>,
}

impl AudioKeyManager {
    pub(crate) fn dispatch(&self, cmd: PacketType, mut data: Bytes) -> Result<(), Error> {
        let seq = BigEndian::read_u32(data.split_to(4).as_ref());

        let sender = self
            .lock(|inner| inner.pending.remove(&seq))
            .ok_or(AudioKeyError::Sequence(seq))?
            .sender;

        match cmd {
            PacketType::AesKey => {
                let mut key = [0u8; 16];
                key.copy_from_slice(data.as_ref());
                sender
                    .send(Ok(AudioKey(key)))
                    .map_err(|_| AudioKeyError::Channel)?
            }
            PacketType::AesKeyError => {
                error!(
                    "error audio key {:x} {:x}",
                    data.as_ref()[0],
                    data.as_ref()[1]
                );
                sender
                    .send(Err(AudioKeyError::AesKey.into()))
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
        let (tx, mut rx) = oneshot::channel();

        let (seq, mut connection) = self.lock(move |inner| {
            let seq = inner.sequence.get();
            let key = PendingKey {
                track,
                file,
                sender: tx,
            };
            inner.pending.insert(seq, key);
            (seq, inner.connection)
        });

        self.send_key_request(seq, track, file)?;
        const KEY_RESPONSE_TIMEOUT: Duration = Duration::from_millis(1500);
        // Only time on a connection counts: a request waits while the session
        // reconnects, and gets its full time again once it is sent on the new connection.
        loop {
            if let Ok(response) = tokio::time::timeout(KEY_RESPONSE_TIMEOUT, &mut rx).await {
                return response?;
            }
            let timed_out = self.lock(|inner| {
                if inner.reconnecting || inner.connection != connection {
                    connection = inner.connection;
                    return false;
                }
                inner.pending.remove(&seq);
                true
            });
            if timed_out {
                error!("Audio key response timeout");
                return Err(AudioKeyError::Timeout.into());
            }
        }
    }

    /// Holds the timeouts of pending requests while the session reconnects.
    pub(crate) fn connection_lost(&self) {
        self.lock(|inner| inner.reconnecting = true);
    }

    /// Requests the pending keys again on the new connection, since the closed one
    /// may have lost their requests or replies. A request still queued goes out
    /// twice; the second reply finds nothing pending and is dropped.
    pub(crate) fn reconnected(&self) {
        let requests: Vec<_> = self.lock(|inner| {
            inner.connection = inner.connection.wrapping_add(1);
            inner.reconnecting = false;
            inner
                .pending
                .iter()
                .map(|(&seq, key)| (seq, key.track, key.file))
                .collect()
        });
        for (seq, track, file) in requests {
            if let Err(e) = self.send_key_request(seq, track, file) {
                warn!("Unable to request audio key again: {e}");
            }
        }
    }

    /// Ends the pending requests of a session that is shut down, for example after
    /// it could not reconnect, and lets later ones time out instead of waiting for
    /// a reconnect that the session no longer makes.
    pub(crate) fn shutdown(&self) {
        self.lock(|inner| {
            inner.reconnecting = false;
            inner.pending.clear();
        });
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

#[cfg(test)]
mod tests {
    use tokio::{sync::mpsc, task::JoinHandle, time::Instant};

    use super::*;
    use crate::Session;

    const KEY: [u8; 16] = [7; 16];

    fn track() -> SpotifyId {
        SpotifyId::from_raw(&[1; 16]).expect("16 bytes are a track id")
    }

    fn file() -> FileId {
        FileId([2; 20])
    }

    async fn sent_request(packets: &mut mpsc::UnboundedReceiver<(u8, Vec<u8>)>) -> Vec<u8> {
        let (cmd, data) = packets.recv().await.expect("a key request");
        assert_eq!(cmd, PacketType::RequestKey as u8);
        data
    }

    fn spawn_request(keys: &AudioKeyManager) -> JoinHandle<Result<AudioKey, Error>> {
        let keys = keys.clone();
        tokio::spawn(async move { keys.request(track(), file()).await })
    }

    fn key_reply(request: &[u8]) -> Bytes {
        // the request is the file id (20 bytes), track id (16), seq (4) and 2 zero bytes
        let mut reply = request[36..40].to_vec();
        reply.extend_from_slice(&KEY);
        reply.into()
    }

    fn is_timeout(error: &Error) -> bool {
        matches!(
            error.error.downcast_ref::<AudioKeyError>(),
            Some(AudioKeyError::Timeout)
        )
    }

    #[tokio::test(start_paused = true)]
    async fn a_request_times_out_on_its_connection() {
        let (session, _packets) = Session::with_test_connection();
        let started = Instant::now();

        let result = session.audio_key().request(track(), file()).await;

        assert!(is_timeout(&result.expect_err("no reply came")));
        assert_eq!(started.elapsed(), Duration::from_millis(1500));
    }

    #[tokio::test(start_paused = true)]
    async fn a_request_waits_while_the_session_reconnects() {
        let (session, mut packets) = Session::with_test_connection();
        let keys = session.audio_key();
        let request = spawn_request(keys);
        let sent = sent_request(&mut packets).await;

        keys.connection_lost();
        tokio::time::sleep(Duration::from_secs(10)).await;
        assert!(!request.is_finished());

        keys.reconnected();
        let resent = sent_request(&mut packets).await;
        assert_eq!(resent, sent);
        // the request sent again gets its full time on the new connection
        tokio::time::sleep(Duration::from_millis(1400)).await;
        keys.dispatch(PacketType::AesKey, key_reply(&resent))
            .expect("the request is pending");

        let key = request.await.expect("the request ran");
        assert_eq!(key.expect("the reply came").0, KEY);
    }

    #[tokio::test(start_paused = true)]
    async fn a_shutdown_ends_a_request_waiting_for_a_reconnect() {
        let (session, mut packets) = Session::with_test_connection();
        let keys = session.audio_key();
        let request = spawn_request(keys);
        sent_request(&mut packets).await;

        keys.connection_lost();
        tokio::time::sleep(Duration::from_secs(10)).await;
        let shut_down = Instant::now();
        session.shutdown();

        let result = tokio::time::timeout(Duration::from_secs(60), request)
            .await
            .expect("the request ended")
            .expect("the request ran");
        assert!(!is_timeout(&result.expect_err("no reply came")));
        assert_eq!(shut_down.elapsed(), Duration::ZERO);
    }

    #[tokio::test(start_paused = true)]
    async fn a_request_after_a_shutdown_does_not_wait_for_a_reconnect() {
        let (session, _packets) = Session::with_test_connection();
        session.audio_key().connection_lost();
        session.shutdown();

        let result = tokio::time::timeout(
            Duration::from_secs(60),
            session.audio_key().request(track(), file()),
        )
        .await
        .expect("the request ended");

        assert!(is_timeout(&result.expect_err("no reply came")));
    }
}
