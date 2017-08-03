use futures::future;
use futures::sink::BoxSink;
use futures::stream::BoxStream;
use futures::sync::{oneshot, mpsc};
use futures::{Future, Stream, Sink, Async, Poll, BoxFuture};
use protobuf::{self, Message};

use core::config::ConnectConfig;
use core::mercury::MercuryError;
use core::session::Session;
use core::util::{now_ms, SpotifyId, SeqGenerator};
use core::version;

use protocol;
use protocol::spirc::{PlayStatus, State, MessageType, Frame, DeviceState};

use mixer::Mixer;
use player::Player;

pub struct SpircTask {
    player: Player,
    mixer: Box<Mixer>,

    sequence: SeqGenerator<u32>,

    ident: String,
    device: DeviceState,
    state: State,

    subscription: BoxStream<Frame, MercuryError>,
    sender: BoxSink<Frame, MercuryError>,
    commands: mpsc::UnboundedReceiver<SpircCommand>,
    end_of_track: BoxFuture<(), oneshot::Canceled>,

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
    Shutdown
}

pub struct Spirc {
    commands: mpsc::UnboundedSender<SpircCommand>,
}

fn initial_state() -> State {
    protobuf_init!(protocol::spirc::State::new(), {
        repeat: false,
        shuffle: false,
        status: PlayStatus::kPlayStatusStop,
        position_ms: 0,
        position_measured_at: 0,
    })
}

fn initial_device_state(config: ConnectConfig, volume: u16) -> DeviceState {
    protobuf_init!(DeviceState::new(), {
        sw_version: version::version_string(),
        is_active: false,
        can_play: true,
        volume: volume as u32,
        name: config.name,
        capabilities => [
            @{
                typ: protocol::spirc::CapabilityType::kCanBePlayer,
                intValue => [1]
            },
            @{
                typ: protocol::spirc::CapabilityType::kDeviceType,
                intValue => [config.device_type as i64]
            },
            @{
                typ: protocol::spirc::CapabilityType::kGaiaEqConnectId,
                intValue => [1]
            },
            @{
                typ: protocol::spirc::CapabilityType::kSupportsLogout,
                intValue => [0]
            },
            @{
                typ: protocol::spirc::CapabilityType::kIsObservable,
                intValue => [1]
            },
            @{
                typ: protocol::spirc::CapabilityType::kVolumeSteps,
                intValue => [64]
            },
            @{
                typ: protocol::spirc::CapabilityType::kSupportedContexts,
                stringValue => [
                    "album",
                    "playlist",
                    "search",
                    "inbox",
                    "toplist",
                    "starred",
                    "publishedstarred",
                    "track",
                ]
            },
            @{
                typ: protocol::spirc::CapabilityType::kSupportedTypes,
                stringValue => [
                    "audio/local",
                    "audio/track",
                    "local",
                    "track",
                ]
            }
        ],
    })
}

impl Spirc {
    pub fn new(config: ConnectConfig, session: Session, player: Player, mixer: Box<Mixer>)
        -> (Spirc, SpircTask)
    {
        debug!("new Spirc[{}]", session.session_id());

        let ident = session.device_id().to_owned();

        let uri = format!("hm://remote/3/user/{}/", session.username());

        let subscription = session.mercury().subscribe(&uri as &str);
        let subscription = subscription.map(|stream| stream.map_err(|_| MercuryError)).flatten_stream();
        let subscription = subscription.map(|response| -> Frame {
            let data = response.payload.first().unwrap();
            protobuf::parse_from_bytes(data).unwrap()
        }).boxed();

        let sender = Box::new(session.mercury().sender(uri).with(|frame: Frame| {
            Ok(frame.write_to_bytes().unwrap())
        }));

        let (cmd_tx, cmd_rx) = mpsc::unbounded();

        let volume = 0xFFFF;
        let device = initial_device_state(config, volume);
        mixer.set_volume(volume);

        let mut task = SpircTask {
            player: player,
            mixer: mixer,

            sequence: SeqGenerator::new(1),

            ident: ident,

            device: device,
            state: initial_state(),

            subscription: subscription,
            sender: sender,
            commands: cmd_rx,
            end_of_track: future::empty().boxed(),

            shutdown: false,
            session: session.clone(),
        };

        let spirc = Spirc {
            commands: cmd_tx,
        };

        task.hello();

        (spirc, task)
    }

    pub fn play(&self) {
        let _ = mpsc::UnboundedSender::send(&self.commands, SpircCommand::Play);
    }
    pub fn play_pause(&self) {
        let _ = mpsc::UnboundedSender::send(&self.commands, SpircCommand::PlayPause);
    }
    pub fn pause(&self) {
        let _ = mpsc::UnboundedSender::send(&self.commands, SpircCommand::Pause);
    }
    pub fn prev(&self) {
        let _ = mpsc::UnboundedSender::send(&self.commands, SpircCommand::Prev);
    }
    pub fn next(&self) {
        let _ = mpsc::UnboundedSender::send(&self.commands, SpircCommand::Next);
    }
    pub fn volume_up(&self) {
        let _ = mpsc::UnboundedSender::send(&self.commands, SpircCommand::VolumeUp);
    }
    pub fn volume_down(&self) {
        let _ = mpsc::UnboundedSender::send(&self.commands, SpircCommand::VolumeDown);
    }
    pub fn shutdown(&self) {
        let _ = mpsc::UnboundedSender::send(&self.commands, SpircCommand::Shutdown);
    }
}

impl Future for SpircTask {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            let mut progress = false;

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
                    Err(oneshot::Canceled) => {
                        self.end_of_track = future::empty().boxed()
                    }
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
        debug!("{:?} {:?} {} {} {}",
               frame.get_typ(),
               frame.get_device_state().get_name(),
               frame.get_ident(),
               frame.get_seq_nr(),
               frame.get_state_update_id());

        if frame.get_ident() == self.ident ||
           (frame.get_recipient().len() > 0 && !frame.get_recipient().contains(&self.ident)) {
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
                let volume = frame.get_volume();
                self.device.set_volume(volume);
                self.mixer.set_volume(frame.get_volume() as u16);
                self.notify(None);
            }

            MessageType::kMessageTypeNotify => {
                if self.device.get_is_active() &&
                    frame.get_device_state().get_is_active()
                {
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

    fn handle_next(&mut self) {
        let current_index = self.state.get_playing_track_index();
        let new_index = (current_index + 1) % (self.state.get_track().len() as u32);

        self.state.set_playing_track_index(new_index);
        self.state.set_position_ms(0);
        self.state.set_position_measured_at(now_ms() as u64);

        self.load_track(true);
    }

    fn handle_prev(&mut self) {
        // Previous behaves differently based on the position
        // Under 3s it goes to the previous song
        // Over 3s it seeks to zero
        if self.position() < 3000 {
            let current_index = self.state.get_playing_track_index();

            let new_index = if current_index == 0 {
                self.state.get_track().len() as u32 - 1
            } else {
                current_index - 1
            };

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
        let mut volume: u32 = self.mixer.volume() as u32 + 4096;
        if volume > 0xFFFF {
            volume = 0xFFFF;
        }
        self.device.set_volume(volume);
        self.mixer.set_volume(volume as u16);
    }

    fn handle_volume_down(&mut self) {
        let mut volume: i32 = self.mixer.volume() as i32 - 4096;
        if volume < 0 {
            volume = 0;
        }
        self.device.set_volume(volume as u32);
        self.mixer.set_volume(volume as u16);
    }

    fn handle_end_of_track(&mut self) {
        let current_index = self.state.get_playing_track_index();
        let new_index = (current_index + 1) % (self.state.get_track().len() as u32);

        self.state.set_playing_track_index(new_index);
        self.state.set_position_ms(0);
        self.state.set_position_measured_at(now_ms() as u64);

        self.load_track(true);
        self.notify(None);
    }

    fn position(&mut self) -> u32 {
        let diff = now_ms() as u64 - self.state.get_position_measured_at();
        self.state.get_position_ms() + diff as u32
    }

    fn update_tracks(&mut self, frame: &protocol::spirc::Frame) {
        let index = frame.get_state().get_playing_track_index();
        let tracks = frame.get_state().get_track();

        self.state.set_playing_track_index(index);
        self.state.set_track(tracks.into_iter().cloned().collect());
    }

    fn load_track(&mut self, play: bool) {
        let index = self.state.get_playing_track_index();
        let track = {
            let gid = self.state.get_track()[index as usize].get_gid();
            SpotifyId::from_raw(gid)
        };
        let position = self.state.get_position_ms();

        let end_of_track = self.player.load(track, play, position);

        if play {
            self.state.set_status(PlayStatus::kPlayStatusPlay);
        } else {
            self.state.set_status(PlayStatus::kPlayStatusPause);
        }

        self.end_of_track = end_of_track.boxed();
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
        let frame = protobuf_init!(protocol::spirc::Frame::new(), {
            version: 1,
            protocol_version: "2.0.0",
            ident: spirc.ident.clone(),
            seq_nr: spirc.sequence.get(),
            typ: cmd,

            device_state: spirc.device.clone(),
            state_update_id: now_ms(),
        });

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
