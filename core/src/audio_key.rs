use std::{collections::HashMap, future::Future, io::Write, sync::Arc, time::Duration};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use bytes::Bytes;
use thiserror::Error;
use tokio::{
    sync::{Mutex, oneshot},
    time::Instant,
};

use crate::{Error, FileId, SpotifyId, packet::PacketType, util::SeqGenerator};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct AudioKey(pub [u8; 16]);

#[derive(Debug, Error)]
pub enum AudioKeyError {
    /// Spotify refused the key, with this code.
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
    #[error("audio key requests are backing off after a refusal")]
    BackingOff,
}

impl From<AudioKeyError> for Error {
    fn from(err: AudioKeyError) -> Self {
        match err {
            AudioKeyError::AesKey(_) => Error::unavailable(err),
            AudioKeyError::Channel => Error::aborted(err),
            AudioKeyError::Sequence(_) => Error::aborted(err),
            AudioKeyError::Packet(_) => Error::unimplemented(err),
            AudioKeyError::Timeout => Error::aborted(err),
            AudioKeyError::BackingOff => Error::resource_exhausted(err),
        }
    }
}

component! {
    AudioKeyManager : AudioKeyManagerInner {
        sequence: SeqGenerator<u32> = SeqGenerator::new(0),
        pending: HashMap<u32, oneshot::Sender<Result<AudioKey, Error>>> = HashMap::new(),
        pacer: Arc<KeyPacer> = Arc::default(),
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
                error!(
                    "error audio key {:x} {:x}",
                    data.as_ref()[0],
                    data.as_ref()[1]
                );
                let code = BigEndian::read_u16(data.as_ref());
                sender
                    .send(Err(AudioKeyError::AesKey(code).into()))
                    .map_err(|_| AudioKeyError::Channel)?
            }
            _ => {
                trace!("Did not expect {cmd:?} AES key packet with data {data:#?}");
                return Err(AudioKeyError::Packet(cmd as u8).into());
            }
        }

        Ok(())
    }

    /// Requests the audio key of `file`.
    ///
    /// Requests go out one at a time, at least 200 ms apart. When Spotify
    /// refuses a key with code 2, the session waits 3, then 5, then 10 seconds
    /// before it asks for any key again, and this key is asked for again after
    /// each wait, up to seven times, about a minute, before the refusal is
    /// returned. A key that arrives ends the wait. Any other refusal is
    /// returned at once.
    pub async fn request(&self, track: SpotifyId, file: FileId) -> Result<AudioKey, Error> {
        let pacer = self.pacer();
        pacer.request(|| self.request_now(track, file)).await
    }

    /// Requests the audio key of `file` once, for a caller that can do
    /// without it, such as one fetching ahead of time. It waits its turn
    /// like [`request`](Self::request), but while the session is waiting
    /// after a refusal it fails at once with [`AudioKeyError::BackingOff`]
    /// instead of asking, and a refusal is returned without asking again.
    pub async fn try_request(&self, track: SpotifyId, file: FileId) -> Result<AudioKey, Error> {
        let pacer = self.pacer();
        pacer.ask(false, || self.request_now(track, file)).await
    }

    /// Whether the session is waiting after a refused key before it asks
    /// for another.
    pub fn is_backing_off(&self) -> bool {
        self.pacer().is_backing_off()
    }

    fn pacer(&self) -> Arc<KeyPacer> {
        self.lock(|inner| inner.pacer.clone())
    }

    async fn request_now(&self, track: SpotifyId, file: FileId) -> Result<AudioKey, Error> {
        let (tx, rx) = oneshot::channel();

        let seq = self.lock(move |inner| {
            let seq = inner.sequence.get();
            inner.pending.insert(seq, tx);
            seq
        });

        self.send_key_request(seq, track, file)?;
        const KEY_RESPONSE_TIMEOUT: Duration = Duration::from_millis(1500);
        match tokio::time::timeout(KEY_RESPONSE_TIMEOUT, rx).await {
            Err(_) => {
                error!("Audio key response timeout");
                Err(AudioKeyError::Timeout.into())
            }
            Ok(k) => k?,
        }
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

/// The shortest time between two audio key requests.
const MIN_REQUEST_GAP: Duration = Duration::from_millis(200);

/// The refusal code worth waiting out. Spotify does not document these codes.
/// In the reports on librespot-org/librespot#1649, code 2 comes in bursts after
/// many keys are asked for at once and asking again later gets the key, while
/// code 1 goes on for every track of an account however long it waits. Only
/// code 2 starts a back-off or is asked again, so any other refusal fails as
/// quickly as it did before.
const TEMPORARY_REFUSAL: u16 = 2;

/// How long the session waits before asking for a key after one, two, and
/// three or more temporary refusals in a row. Spotify does not say that it
/// limits how fast keys may be asked for, but refusals come in bursts that
/// asking again at once prolongs, and that stop after some quiet (see
/// librespot-org/librespot#1649).
const REFUSAL_BACKOFF: [Duration; 3] = [
    Duration::from_secs(3),
    Duration::from_secs(5),
    Duration::from_secs(10),
];

/// How many times [`AudioKeyManager::request`] asks again for a temporarily
/// refused key, each time after the back-off. With the back-offs above, it
/// asks at about 0, 3, 8, 18, 28, 38, 48 and 58 seconds. A burst has been seen
/// to keep refusing until 28 seconds after its first refusal, and once for
/// about 40.
const REFUSAL_RETRIES: usize = 7;

/// Paces the audio key requests of a session.
#[derive(Default)]
struct KeyPacer {
    /// Held for the whole of a request, so only one is in flight. It keeps
    /// when the last one was sent.
    turn: Mutex<Option<Instant>>,
    backoff: std::sync::Mutex<Backoff>,
}

#[derive(Default)]
struct Backoff {
    /// Temporary refusals in a row, since the last key that arrived.
    refusals: usize,
    /// No key is asked for before this.
    until: Option<Instant>,
}

fn is_temporary_refusal(error: &Error) -> bool {
    matches!(
        error.error.downcast_ref::<AudioKeyError>(),
        Some(AudioKeyError::AesKey(TEMPORARY_REFUSAL))
    )
}

impl KeyPacer {
    fn backoff(&self) -> std::sync::MutexGuard<'_, Backoff> {
        self.backoff
            .lock()
            .expect("audio key back-off mutex should not be poisoned")
    }

    fn backoff_until(&self) -> Option<Instant> {
        self.backoff().until.filter(|until| *until > Instant::now())
    }

    fn is_backing_off(&self) -> bool {
        self.backoff_until().is_some()
    }

    /// Asks with `ask` until a temporary refusal has been asked again
    /// [`REFUSAL_RETRIES`] times.
    async fn request<F, Fut>(&self, mut ask: F) -> Result<AudioKey, Error>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<AudioKey, Error>>,
    {
        let mut retries = 0;
        loop {
            match self.ask(true, &mut ask).await {
                Err(e) if is_temporary_refusal(&e) && retries < REFUSAL_RETRIES => {
                    retries += 1;
                    debug!("Asking again for a refused audio key ({retries}/{REFUSAL_RETRIES})");
                }
                result => return result,
            }
        }
    }

    /// Asks once with `ask` when it is this request's turn. While the
    /// session backs off, it waits for the end if `wait`, and otherwise
    /// fails without asking.
    async fn ask<F, Fut>(&self, wait: bool, ask: F) -> Result<AudioKey, Error>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<AudioKey, Error>>,
    {
        // A request waiting out the back-off holds the turn for all of it, so
        // this has to answer before queueing for the turn.
        if !wait && self.is_backing_off() {
            return Err(AudioKeyError::BackingOff.into());
        }

        let mut last_sent = self.turn.lock().await;

        // The request that held the turn may have started a back-off.
        if let Some(until) = self.backoff_until() {
            if !wait {
                return Err(AudioKeyError::BackingOff.into());
            }
            tokio::time::sleep_until(until).await;
        }
        if let Some(last_sent) = *last_sent {
            tokio::time::sleep_until(last_sent + MIN_REQUEST_GAP).await;
        }

        *last_sent = Some(Instant::now());
        let result = ask().await;

        let mut backoff = self.backoff();
        match &result {
            Ok(_) => *backoff = Backoff::default(),
            Err(e) if is_temporary_refusal(e) => {
                backoff.refusals += 1;
                let delay = REFUSAL_BACKOFF[backoff.refusals.min(REFUSAL_BACKOFF.len()) - 1];
                warn!("Spotify refused an audio key, waiting {delay:?} before asking for another");
                backoff.until = Some(Instant::now() + delay);
            }
            // Another refusal code will not pass by waiting, and a timeout or
            // a lost connection says nothing about whether Spotify would give
            // the key.
            Err(_) => {}
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Mutex as StdMutex,
        atomic::{AtomicUsize, Ordering},
    };

    use super::*;
    use crate::{SessionConfig, session::Session};

    const KEY: AudioKey = AudioKey([7; 16]);

    /// A key server that answers each request in turn from `answers`,
    /// refusing temporarily once they run out, and notes when each request
    /// came.
    struct KeyServer {
        answers: StdMutex<Vec<bool>>,
        asked_at: StdMutex<Vec<Instant>>,
        start: Instant,
        in_flight: AtomicUsize,
        most_in_flight: AtomicUsize,
    }

    impl KeyServer {
        fn new(answers: &[bool]) -> Arc<Self> {
            Arc::new(Self {
                answers: StdMutex::new(answers.iter().rev().copied().collect()),
                asked_at: StdMutex::default(),
                start: Instant::now(),
                in_flight: AtomicUsize::new(0),
                most_in_flight: AtomicUsize::new(0),
            })
        }

        async fn ask(self: Arc<Self>) -> Result<AudioKey, Error> {
            self.asked_at.lock().unwrap().push(Instant::now());
            let in_flight = self.in_flight.fetch_add(1, Ordering::SeqCst) + 1;
            self.most_in_flight.fetch_max(in_flight, Ordering::SeqCst);
            // The answer takes a moment to arrive.
            tokio::time::sleep(Duration::from_millis(50)).await;
            self.in_flight.fetch_sub(1, Ordering::SeqCst);
            if self.answers.lock().unwrap().pop().unwrap_or(false) {
                Ok(KEY)
            } else {
                Err(AudioKeyError::AesKey(TEMPORARY_REFUSAL).into())
            }
        }

        /// When each request came, in milliseconds since the server started.
        fn asked_at_ms(&self) -> Vec<u128> {
            self.asked_at
                .lock()
                .unwrap()
                .iter()
                .map(|at| (*at - self.start).as_millis())
                .collect()
        }
    }

    fn is_backing_off_error(result: &Result<AudioKey, Error>) -> bool {
        matches!(
            result
                .as_ref()
                .map_err(|e| e.error.downcast_ref::<AudioKeyError>()),
            Err(Some(AudioKeyError::BackingOff))
        )
    }

    #[tokio::test(start_paused = true)]
    async fn requests_go_out_one_at_a_time_and_spaced() {
        let pacer = Arc::new(KeyPacer::default());
        let server = KeyServer::new(&[true; 4]);

        let requests = (0..4).map(|_| {
            let pacer = pacer.clone();
            let server = server.clone();
            async move { pacer.request(|| server.clone().ask()).await }
        });
        let results = futures_util::future::join_all(requests).await;

        assert!(results.iter().all(|key| matches!(key, Ok(KEY))));
        assert_eq!(server.most_in_flight.load(Ordering::SeqCst), 1);
        // Each waits for the previous answer (50 ms) and then for the gap
        // since it was sent.
        assert_eq!(server.asked_at_ms(), vec![0, 200, 400, 600]);
    }

    #[tokio::test(start_paused = true)]
    async fn a_refused_key_is_asked_again_after_a_growing_back_off() {
        let pacer = KeyPacer::default();
        let server = KeyServer::new(&[false, false, false, true]);

        let key = pacer.request(|| server.clone().ask()).await;

        assert!(matches!(key, Ok(KEY)));
        // Refused at 50, 3050 + 50 and 8100 + 50 ms: 3, 5 and 10 s back-offs.
        assert_eq!(server.asked_at_ms(), vec![0, 3050, 8100, 18150]);
        assert!(!pacer.is_backing_off());
    }

    #[tokio::test(start_paused = true)]
    async fn a_key_refused_for_longer_than_three_retries_still_arrives() {
        let pacer = KeyPacer::default();
        // Refused four times over 18 s, and given at the next ask, at about
        // 28 s.
        let server = KeyServer::new(&[false, false, false, false, true]);

        let key = pacer.request(|| server.clone().ask()).await;

        assert!(matches!(key, Ok(KEY)));
        assert_eq!(server.asked_at_ms(), vec![0, 3050, 8100, 18150, 28200]);
        assert!(!pacer.is_backing_off());
    }

    #[tokio::test(start_paused = true)]
    async fn another_refusal_code_returns_at_once_and_starts_no_back_off() {
        let pacer = KeyPacer::default();
        let start = Instant::now();
        let asked = AtomicUsize::new(0);

        let key = pacer
            .request(|| async {
                asked.fetch_add(1, Ordering::SeqCst);
                Err(AudioKeyError::AesKey(1).into())
            })
            .await;

        assert!(matches!(
            key.as_ref()
                .map_err(|e| e.error.downcast_ref::<AudioKeyError>()),
            Err(Some(AudioKeyError::AesKey(1)))
        ));
        assert_eq!(asked.load(Ordering::SeqCst), 1);
        assert_eq!(start.elapsed(), Duration::ZERO);
        assert!(!pacer.is_backing_off());
    }

    #[tokio::test]
    async fn dispatch_passes_on_the_refusal_code() {
        let session = Session::new(SessionConfig::default(), None);
        let manager = session.audio_key();
        let (tx, rx) = oneshot::channel();
        manager.lock(|inner| inner.pending.insert(9, tx));

        manager
            .dispatch(
                PacketType::AesKeyError,
                Bytes::from_static(&[0, 0, 0, 9, 0, 2]),
            )
            .unwrap();

        assert!(rx.await.unwrap().as_ref().is_err_and(is_temporary_refusal));
    }

    #[tokio::test(start_paused = true)]
    async fn a_key_refused_through_every_retry_is_reported() {
        let pacer = KeyPacer::default();
        let server = KeyServer::new(&[]);

        let key = pacer.request(|| server.clone().ask()).await;

        assert!(key.as_ref().is_err_and(is_temporary_refusal));
        assert_eq!(server.asked_at_ms().len(), REFUSAL_RETRIES + 1);
        // About a minute, with the 50 ms that each answer takes.
        assert_eq!(
            server.asked_at_ms(),
            vec![0, 3050, 8100, 18150, 28200, 38250, 48300, 58350]
        );
        assert!(pacer.is_backing_off());

        // The back-off stays at its longest until a key arrives.
        let start = Instant::now();
        let _ = pacer.ask(true, || server.clone().ask()).await;
        assert_eq!(
            server.asked_at.lock().unwrap().last().unwrap(),
            &(start + REFUSAL_BACKOFF[2])
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_refusal_holds_back_every_request_of_the_session() {
        let pacer = Arc::new(KeyPacer::default());
        let server = KeyServer::new(&[false, true, true]);

        let first = pacer.ask(true, || server.clone().ask()).await;
        assert!(first.as_ref().is_err_and(is_temporary_refusal));
        assert!(pacer.is_backing_off());

        // Another track's key waits for the back-off too.
        let second = pacer.request(|| server.clone().ask()).await;
        assert!(matches!(second, Ok(KEY)));
        assert_eq!(server.asked_at_ms(), vec![0, 3050]);

        // A key that arrives ends the back-off, so the next request only
        // keeps the gap.
        assert!(!pacer.is_backing_off());
        let third = pacer.request(|| server.clone().ask()).await;
        assert!(matches!(third, Ok(KEY)));
        assert_eq!(server.asked_at_ms(), vec![0, 3050, 3250]);
    }

    #[tokio::test(start_paused = true)]
    async fn an_optional_request_is_not_sent_during_a_back_off() {
        let pacer = KeyPacer::default();
        let server = KeyServer::new(&[false, true]);

        let _ = pacer.ask(true, || server.clone().ask()).await;
        let optional = pacer.ask(false, || server.clone().ask()).await;

        assert!(is_backing_off_error(&optional));
        assert_eq!(server.asked_at_ms(), vec![0]);

        tokio::time::advance(REFUSAL_BACKOFF[0]).await;
        let optional = pacer.ask(false, || server.clone().ask()).await;
        assert!(matches!(optional, Ok(KEY)));
    }

    #[tokio::test(start_paused = true)]
    async fn an_optional_request_does_not_wait_for_a_request_holding_the_turn() {
        let pacer = Arc::new(KeyPacer::default());
        let server = KeyServer::new(&[false, true]);

        let _ = pacer.ask(true, || server.clone().ask()).await;

        // This request holds the turn through the back-off.
        let waiting = tokio::spawn({
            let pacer = pacer.clone();
            let server = server.clone();
            async move { pacer.request(|| server.clone().ask()).await }
        });
        tokio::task::yield_now().await;

        let start = Instant::now();
        let optional = pacer.ask(false, || server.clone().ask()).await;

        assert!(is_backing_off_error(&optional));
        assert_eq!(start.elapsed(), Duration::ZERO);
        assert_eq!(server.asked_at_ms(), vec![0]);

        assert!(matches!(waiting.await.unwrap(), Ok(KEY)));
        assert_eq!(server.asked_at_ms(), vec![0, 3050]);
    }

    #[tokio::test(start_paused = true)]
    async fn an_optional_request_queued_behind_a_refusal_is_not_sent() {
        let pacer = KeyPacer::default();
        let server = KeyServer::new(&[false]);

        // No back-off is on when the optional request starts waiting its
        // turn, but the request ahead of it starts one.
        let (first, optional) = futures_util::future::join(
            pacer.ask(true, || server.clone().ask()),
            pacer.ask(false, || server.clone().ask()),
        )
        .await;

        assert!(first.as_ref().is_err_and(is_temporary_refusal));
        assert!(is_backing_off_error(&optional));
        assert_eq!(server.asked_at_ms(), vec![0]);
    }

    #[tokio::test(start_paused = true)]
    async fn a_dropped_request_gives_up_its_turn_and_leaves_the_back_off() {
        let pacer = KeyPacer::default();
        let server = KeyServer::new(&[false, true]);

        let _ = pacer.ask(true, || server.clone().ask()).await;

        // A load the player has moved on from is dropped mid-wait.
        let dropped = tokio::time::timeout(
            Duration::from_secs(1),
            pacer.request(|| server.clone().ask()),
        )
        .await;
        assert!(dropped.is_err());
        assert!(pacer.is_backing_off());

        let key = pacer.request(|| server.clone().ask()).await;
        assert!(matches!(key, Ok(KEY)));
        assert_eq!(server.asked_at_ms(), vec![0, 3050]);
    }

    #[tokio::test(start_paused = true)]
    async fn a_timeout_does_not_start_a_back_off() {
        let pacer = KeyPacer::default();

        let timeout = pacer
            .ask(true, || async { Err(AudioKeyError::Timeout.into()) })
            .await;

        assert!(timeout.is_err());
        assert!(!pacer.is_backing_off());
    }
}
