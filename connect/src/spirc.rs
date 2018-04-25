use futures::future;
use futures::sync::{mpsc, oneshot};
use futures::{Async, Future, Poll, Sink, Stream};
use protobuf::{self, Message};

use core::config::ConnectConfig;
use core::mercury::MercuryError;
use core::session::Session;
use core::spotify_id::SpotifyId;
use core::util::SeqGenerator;
use core::version;

use protocol;
use protocol::spirc::{DeviceState, Frame, MessageType, PlayStatus, State};

use playback::mixer::Mixer;
use playback::player::Player;

use rand;
use rand::Rng;
use std;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SpircTask {
    player: Player,
    mixer: Box<Mixer>,
    linear_volume: bool,

    sequence: SeqGenerator<u32>,

    ident: String,
    device: DeviceState,
    state: State,

    subscription: Box<Stream<Item = Frame, Error = MercuryError>>,
    sender: Box<Sink<SinkItem = Frame, SinkError = MercuryError>>,
    commands: mpsc::UnboundedReceiver<SpircCommand>,
    end_of_track: Box<Future<Item = (), Error = oneshot::Canceled>>,

    shutdown: bool,
    session: Session,
}

pub enum SpircCommand {
    Play,
    PlayPause,
    Pause,
    Prev,
    Next,
    VolumeUp,
    VolumeDown,
    Shutdown,
}

pub struct Spirc {
    commands: mpsc::UnboundedSender<SpircCommand>,
}

fn now_ms() -> i64 {
    let dur = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => dur,
        Err(err) => err.duration(),
    };
    (dur.as_secs() * 1000 + (dur.subsec_nanos() / 1000_000) as u64) as i64
}

fn initial_state() -> State {
    let mut frame = protocol::spirc::State::new();
    frame.set_repeat(false);
    frame.set_shuffle(false);
    frame.set_status(PlayStatus::kPlayStatusStop);
    frame.set_position_ms(0);
    frame.set_position_measured_at(0);
    frame
}

fn initial_device_state(config: ConnectConfig, volume: u16) -> DeviceState {
    {
        let mut msg = DeviceState::new();
        msg.set_sw_version(version::version_string());
        msg.set_is_active(false);
        msg.set_can_play(true);
        msg.set_volume(volume as u32);
        msg.set_name(config.name);
        {
            let repeated = msg.mut_capabilities();
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kCanBePlayer);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(1)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kDeviceType);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(config.device_type as i64)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kGaiaEqConnectId);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(1)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kSupportsLogout);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(0)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kIsObservable);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(1)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kVolumeSteps);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(64)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kSupportedContexts);
                {
                    let repeated = msg.mut_stringValue();
                    repeated.push(::std::convert::Into::into("album"));
                    repeated.push(::std::convert::Into::into("playlist"));
                    repeated.push(::std::convert::Into::into("search"));
                    repeated.push(::std::convert::Into::into("inbox"));
                    repeated.push(::std::convert::Into::into("toplist"));
                    repeated.push(::std::convert::Into::into("starred"));
                    repeated.push(::std::convert::Into::into("publishedstarred"));
                    repeated.push(::std::convert::Into::into("track"))
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kSupportedTypes);
                {
                    let repeated = msg.mut_stringValue();
                    repeated.push(::std::convert::Into::into("audio/local"));
                    repeated.push(::std::convert::Into::into("audio/track"));
                    repeated.push(::std::convert::Into::into("local"));
                    repeated.push(::std::convert::Into::into("track"))
                };
                msg
            };
        };
        msg
    }
}

fn calc_logarithmic_volume(volume: u16) -> u16 {
    // Volume conversion taken from https://www.dr-lex.be/info-stuff/volumecontrols.html#ideal2
    // Convert the given volume [0..0xffff] to a dB gain
    // We assume a dB range of 60dB.
    // Use the equatation: a * exp(b * x)
    // in which a = IDEAL_FACTOR, b = 1/1000
    const IDEAL_FACTOR: f64 = 6.908;
    let normalized_volume = volume as f64 / std::u16::MAX as f64; // To get a value between 0 and 1

    let mut val = std::u16::MAX;
    // Prevent val > std::u16::MAX due to rounding errors
    if normalized_volume < 0.999 {
        let new_volume = (normalized_volume * IDEAL_FACTOR).exp() / 1000.0;
        val = (new_volume * std::u16::MAX as f64) as u16;
    }

    debug!("input volume:{} to mixer: {}", volume, val);

    // return the scale factor (0..0xffff) (equivalent to a voltage multiplier).
    val
}

fn volume_to_mixer(volume: u16, linear_volume: bool) -> u16 {
    if linear_volume {
        debug!("linear volume: {}", volume);
        volume
    } else {
        calc_logarithmic_volume(volume)
    }
}

impl Spirc {
    pub fn new(
        config: ConnectConfig,
        session: Session,
        player: Player,
        mixer: Box<Mixer>,
    ) -> (Spirc, SpircTask) {
        debug!("new Spirc[{}]", session.session_id());

        let ident = session.device_id().to_owned();

        let uri = format!("hm://remote/3/user/{}/", session.username());

        let subscription = session.mercury().subscribe(&uri as &str);
        let subscription = subscription
            .map(|stream| stream.map_err(|_| MercuryError))
            .flatten_stream();
        let subscription = Box::new(subscription.map(|response| -> Frame {
            let data = response.payload.first().unwrap();
            protobuf::parse_from_bytes(data).unwrap()
        }));

        let sender = Box::new(
            session
                .mercury()
                .sender(uri)
                .with(|frame: Frame| Ok(frame.write_to_bytes().unwrap())),
        );

        let (cmd_tx, cmd_rx) = mpsc::unbounded();

        let volume = config.volume as u16;
        let linear_volume = config.linear_volume;

        let device = initial_device_state(config, volume);
        mixer.set_volume(volume_to_mixer(volume as u16, linear_volume));

        let mut task = SpircTask {
            player: player,
            mixer: mixer,
            linear_volume: linear_volume,

            sequence: SeqGenerator::new(1),

            ident: ident,

            device: device,
            state: initial_state(),

            subscription: subscription,
            sender: sender,
            commands: cmd_rx,
            end_of_track: Box::new(future::empty()),

            shutdown: false,
            session: session.clone(),
        };

        let spirc = Spirc { commands: cmd_tx };

        task.hello();

        (spirc, task)
    }

    pub fn play(&self) {
        let _ = self.commands.unbounded_send(SpircCommand::Play);
    }
    pub fn play_pause(&self) {
        let _ = self.commands.unbounded_send(SpircCommand::PlayPause);
    }
    pub fn pause(&self) {
        let _ = self.commands.unbounded_send(SpircCommand::Pause);
    }
    pub fn prev(&self) {
        let _ = self.commands.unbounded_send(SpircCommand::Prev);
    }
    pub fn next(&self) {
        let _ = self.commands.unbounded_send(SpircCommand::Next);
    }
    pub fn volume_up(&self) {
        let _ = self.commands.unbounded_send(SpircCommand::VolumeUp);
    }
    pub fn volume_down(&self) {
        let _ = self.commands.unbounded_send(SpircCommand::VolumeDown);
    }
    pub fn shutdown(&self) {
        let _ = self.commands.unbounded_send(SpircCommand::Shutdown);
    }
}

impl Future for SpircTask {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            let mut progress = false;

            if self.session.is_invalid() {
                return Ok(Async::Ready(()));
            }

            if !self.shutdown {
                match self.subscription.poll().unwrap() {
                    Async::Ready(Some(frame)) => {
                        progress = true;
                        self.handle_frame(frame);
                    }
                    Async::Ready(None) => panic!("subscription terminated"),
                    Async::NotReady => (),
                }

                match self.commands.poll().unwrap() {
                    Async::Ready(Some(command)) => {
                        progress = true;
                        self.handle_command(command);
                    }
                    Async::Ready(None) => (),
                    Async::NotReady => (),
                }

                match self.end_of_track.poll() {
                    Ok(Async::Ready(())) => {
                        progress = true;
                        self.handle_end_of_track();
                    }
                    Ok(Async::NotReady) => (),
                    Err(oneshot::Canceled) => self.end_of_track = Box::new(future::empty()),
                }
            }

            let poll_sender = self.sender.poll_complete().unwrap();

            // Only shutdown once we've flushed out all our messages
            if self.shutdown && poll_sender.is_ready() {
                return Ok(Async::Ready(()));
            }

            if !progress {
                return Ok(Async::NotReady);
            }
        }
    }
}

impl SpircTask {
    fn handle_command(&mut self, cmd: SpircCommand) {
        let active = self.device.get_is_active();
        match cmd {
            SpircCommand::Play => {
                if active {
                    self.handle_play();
                    self.notify(None);
                } else {
                    CommandSender::new(self, MessageType::kMessageTypePlay).send();
                }
            }
            SpircCommand::PlayPause => {
                if active {
                    self.handle_play_pause();
                    self.notify(None);
                } else {
                    CommandSender::new(self, MessageType::kMessageTypePlayPause).send();
                }
            }
            SpircCommand::Pause => {
                if active {
                    self.handle_pause();
                    self.notify(None);
                } else {
                    CommandSender::new(self, MessageType::kMessageTypePause).send();
                }
            }
            SpircCommand::Prev => {
                if active {
                    self.handle_prev();
                    self.notify(None);
                } else {
                    CommandSender::new(self, MessageType::kMessageTypePrev).send();
                }
            }
            SpircCommand::Next => {
                if active {
                    self.handle_next();
                    self.notify(None);
                } else {
                    CommandSender::new(self, MessageType::kMessageTypeNext).send();
                }
            }
            SpircCommand::VolumeUp => {
                if active {
                    self.handle_volume_up();
                    self.notify(None);
                } else {
                    CommandSender::new(self, MessageType::kMessageTypeVolumeUp).send();
                }
            }
            SpircCommand::VolumeDown => {
                if active {
                    self.handle_volume_down();
                    self.notify(None);
                } else {
                    CommandSender::new(self, MessageType::kMessageTypeVolumeDown).send();
                }
            }
            SpircCommand::Shutdown => {
                CommandSender::new(self, MessageType::kMessageTypeGoodbye).send();
                self.shutdown = true;
                self.commands.close();
            }
        }
    }

    fn handle_frame(&mut self, frame: Frame) {
        debug!(
            "{:?} {:?} {} {} {}",
            frame.get_typ(),
            frame.get_device_state().get_name(),
            frame.get_ident(),
            frame.get_seq_nr(),
            frame.get_state_update_id()
        );

        if frame.get_ident() == self.ident
            || (frame.get_recipient().len() > 0 && !frame.get_recipient().contains(&self.ident))
        {
            return;
        }

        match frame.get_typ() {
            MessageType::kMessageTypeHello => {
                self.notify(Some(frame.get_ident()));
            }

            MessageType::kMessageTypeLoad => {
                if !self.device.get_is_active() {
                    self.device.set_is_active(true);
                    self.device.set_became_active_at(now_ms());
                }

                self.update_tracks(&frame);

                if self.state.get_track().len() > 0 {
                    self.state.set_position_ms(frame.get_state().get_position_ms());
                    self.state.set_position_measured_at(now_ms() as u64);

                    let play = frame.get_state().get_status() == PlayStatus::kPlayStatusPlay;
                    self.load_track(play);
                } else {
                    self.state.set_status(PlayStatus::kPlayStatusStop);
                }

                self.notify(None);
            }

            MessageType::kMessageTypePlay => {
                self.handle_play();
                self.notify(None);
            }

            MessageType::kMessageTypePlayPause => {
                self.handle_play_pause();
                self.notify(None);
            }

            MessageType::kMessageTypePause => {
                self.handle_pause();
                self.notify(None);
            }

            MessageType::kMessageTypeNext => {
                self.handle_next();
                self.notify(None);
            }

            MessageType::kMessageTypePrev => {
                self.handle_prev();
                self.notify(None);
            }

            MessageType::kMessageTypeVolumeUp => {
                self.handle_volume_up();
                self.notify(None);
            }

            MessageType::kMessageTypeVolumeDown => {
                self.handle_volume_down();
                self.notify(None);
            }

            MessageType::kMessageTypeRepeat => {
                self.state.set_repeat(frame.get_state().get_repeat());
                self.notify(None);
            }

            MessageType::kMessageTypeShuffle => {
                self.state.set_shuffle(frame.get_state().get_shuffle());
                if self.state.get_shuffle() {
                    let current_index = self.state.get_playing_track_index();
                    {
                        let tracks = self.state.mut_track();
                        tracks.swap(0, current_index as usize);
                        if let Some((_, rest)) = tracks.split_first_mut() {
                            rand::thread_rng().shuffle(rest);
                        }
                    }
                    self.state.set_playing_track_index(0);
                } else {
                    let context = self.state.get_context_uri();
                    debug!("{:?}", context);
                }
                self.notify(None);
            }

            MessageType::kMessageTypeSeek => {
                let position = frame.get_position();

                self.state.set_position_ms(position);
                self.state.set_position_measured_at(now_ms() as u64);
                self.player.seek(position);
                self.notify(None);
            }

            MessageType::kMessageTypeReplace => {
                self.update_tracks(&frame);
                self.notify(None);
            }

            MessageType::kMessageTypeVolume => {
                self.device.set_volume(frame.get_volume());
                self.mixer
                    .set_volume(volume_to_mixer(frame.get_volume() as u16, self.linear_volume));
                self.notify(None);
            }

            MessageType::kMessageTypeNotify => {
                if self.device.get_is_active() && frame.get_device_state().get_is_active() {
                    self.device.set_is_active(false);
                    self.state.set_status(PlayStatus::kPlayStatusStop);
                    self.player.stop();
                    self.mixer.stop();
                }
            }

            _ => (),
        }
    }

    fn handle_play(&mut self) {
        if self.state.get_status() == PlayStatus::kPlayStatusPause {
            self.mixer.start();
            self.player.play();
            self.state.set_status(PlayStatus::kPlayStatusPlay);
            self.state.set_position_measured_at(now_ms() as u64);
        }
    }

    fn handle_play_pause(&mut self) {
        match self.state.get_status() {
            PlayStatus::kPlayStatusPlay => self.handle_pause(),
            PlayStatus::kPlayStatusPause => self.handle_play(),
            _ => (),
        }
    }

    fn handle_pause(&mut self) {
        if self.state.get_status() == PlayStatus::kPlayStatusPlay {
            self.player.pause();
            self.mixer.stop();
            self.state.set_status(PlayStatus::kPlayStatusPause);

            let now = now_ms() as u64;
            let position = self.state.get_position_ms();

            let diff = now - self.state.get_position_measured_at();

            self.state.set_position_ms(position + diff as u32);
            self.state.set_position_measured_at(now);
        }
    }

    fn consume_queued_track(&mut self) -> usize {
        // Removes current track if it is queued
        // Returns the index of the next track
        let current_index = self.state.get_playing_track_index() as usize;
        if self.state.get_track()[current_index].get_queued() {
            self.state.mut_track().remove(current_index);
            return current_index;
        }
        current_index + 1
    }

    fn handle_next(&mut self) {
        let mut new_index = self.consume_queued_track() as u32;
        let mut continue_playing = true;
        if new_index >= self.state.get_track().len() as u32 {
            new_index = 0; // Loop around back to start
            continue_playing = self.state.get_repeat();
        }
        self.state.set_playing_track_index(new_index);
        self.state.set_position_ms(0);
        self.state.set_position_measured_at(now_ms() as u64);

        self.load_track(continue_playing);
    }

    fn handle_prev(&mut self) {
        // Previous behaves differently based on the position
        // Under 3s it goes to the previous song (starts playing)
        // Over 3s it seeks to zero (retains previous play status)
        if self.position() < 3000 {
            // Queued tracks always follow the currently playing track.
            // They should not be considered when calculating the previous
            // track so extract them beforehand and reinsert them after it.
            let mut queue_tracks = Vec::new();
            {
                let queue_index = self.consume_queued_track();
                let tracks = self.state.mut_track();
                while queue_index < tracks.len() && tracks[queue_index].get_queued() {
                    queue_tracks.push(tracks.remove(queue_index));
                }
            }
            let current_index = self.state.get_playing_track_index();
            let new_index = if current_index > 0 {
                current_index - 1
            } else if self.state.get_repeat() {
                self.state.get_track().len() as u32 - 1
            } else {
                0
            };
            // Reinsert queued tracks after the new playing track.
            let mut pos = (new_index + 1) as usize;
            for track in queue_tracks.into_iter() {
                self.state.mut_track().insert(pos, track);
                pos += 1;
            }

            self.state.set_playing_track_index(new_index);
            self.state.set_position_ms(0);
            self.state.set_position_measured_at(now_ms() as u64);

            self.load_track(true);
        } else {
            self.state.set_position_ms(0);
            self.state.set_position_measured_at(now_ms() as u64);
            self.player.seek(0);
        }
    }

    fn handle_volume_up(&mut self) {
        let mut volume: u32 = self.device.get_volume() as u32 + 4096;
        if volume > 0xFFFF {
            volume = 0xFFFF;
        }
        self.device.set_volume(volume);
        self.mixer
            .set_volume(volume_to_mixer(volume as u16, self.linear_volume));
    }

    fn handle_volume_down(&mut self) {
        let mut volume: i32 = self.device.get_volume() as i32 - 4096;
        if volume < 0 {
            volume = 0;
        }
        self.device.set_volume(volume as u32);
        self.mixer
            .set_volume(volume_to_mixer(volume as u16, self.linear_volume));
    }

    fn handle_end_of_track(&mut self) {
        self.handle_next();
        self.notify(None);
    }

    fn position(&mut self) -> u32 {
        let diff = now_ms() as u64 - self.state.get_position_measured_at();
        self.state.get_position_ms() + diff as u32
    }

    fn update_tracks(&mut self, frame: &protocol::spirc::Frame) {
        let index = frame.get_state().get_playing_track_index();
        let tracks = frame.get_state().get_track();
        let context_uri = frame.get_state().get_context_uri().to_owned();

        self.state.set_playing_track_index(index);
        self.state.set_track(tracks.into_iter().cloned().collect());
        self.state.set_context_uri(context_uri);
        self.state.set_repeat(frame.get_state().get_repeat());
        self.state.set_shuffle(frame.get_state().get_shuffle());
    }

    fn load_track(&mut self, play: bool) {
        let index = self.state.get_playing_track_index();
        let track = {
            let gid = self.state.get_track()[index as usize].get_gid();
            SpotifyId::from_raw(gid).unwrap()
        };
        let position = self.state.get_position_ms();

        let end_of_track = self.player.load(track, play, position);

        if play {
            self.state.set_status(PlayStatus::kPlayStatusPlay);
        } else {
            self.state.set_status(PlayStatus::kPlayStatusPause);
        }

        self.end_of_track = Box::new(end_of_track);
    }

    fn hello(&mut self) {
        CommandSender::new(self, MessageType::kMessageTypeHello).send();
    }

    fn notify(&mut self, recipient: Option<&str>) {
        let mut cs = CommandSender::new(self, MessageType::kMessageTypeNotify);
        if let Some(s) = recipient {
            cs = cs.recipient(&s);
        }
        cs.send();
    }
}

impl Drop for SpircTask {
    fn drop(&mut self) {
        debug!("drop Spirc[{}]", self.session.session_id());
    }
}

struct CommandSender<'a> {
    spirc: &'a mut SpircTask,
    frame: protocol::spirc::Frame,
}

impl<'a> CommandSender<'a> {
    fn new(spirc: &'a mut SpircTask, cmd: MessageType) -> CommandSender {
        let mut frame = protocol::spirc::Frame::new();
        frame.set_version(1);
        frame.set_protocol_version(::std::convert::Into::into("2.0.0"));
        frame.set_ident(spirc.ident.clone());
        frame.set_seq_nr(spirc.sequence.get());
        frame.set_typ(cmd);
        frame.set_device_state(spirc.device.clone());
        frame.set_state_update_id(now_ms());
        CommandSender {
            spirc: spirc,
            frame: frame,
        }
    }

    fn recipient(mut self, recipient: &'a str) -> CommandSender {
        self.frame.mut_recipient().push(recipient.to_owned());
        self
    }

    #[allow(dead_code)]
    fn state(mut self, state: protocol::spirc::State) -> CommandSender<'a> {
        self.frame.set_state(state);
        self
    }

    fn send(mut self) {
        if !self.frame.has_state() && self.spirc.device.get_is_active() {
            self.frame.set_state(self.spirc.state.clone());
        }

        let send = self.spirc.sender.start_send(self.frame).unwrap();
        assert!(send.is_ready());
    }
}
