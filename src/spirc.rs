use futures::future;
use futures::sink::BoxSink;
use futures::stream::BoxStream;
use futures::sync::{oneshot, mpsc};
use futures::{Future, Stream, Sink, Async, Poll, BoxFuture};
use protobuf::{self, Message};

use mercury::MercuryError;
use player::Player;
use mixer::Mixer;
use session::Session;
use util::{now_ms, SpotifyId, SeqGenerator};
use version;

use protocol;
use protocol::spirc::{PlayStatus, State, MessageType, Frame, DeviceState};

pub struct SpircTask {
    player: Player,
    mixer: Box<Mixer + Send>,

    sequence: SeqGenerator<u32>,

    ident: String,
    device: DeviceState,
    state: State,

    subscription: BoxStream<Frame, MercuryError>,
    sender: BoxSink<Frame, MercuryError>,
    commands: mpsc::UnboundedReceiver<SpircCommand>,
    end_of_track: BoxFuture<(), oneshot::Canceled>,

    shutdown: bool,
}

pub enum SpircCommand {
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

fn initial_device_state(name: String, volume: u16) -> DeviceState {
    protobuf_init!(DeviceState::new(), {
        sw_version: version::version_string(),
        is_active: false,
        can_play: true,
        volume: volume as u32,
        name: name,
        capabilities => [
            @{
                typ: protocol::spirc::CapabilityType::kCanBePlayer,
                intValue => [1]
            },
            @{
                typ: protocol::spirc::CapabilityType::kDeviceType,
                intValue => [5]
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
    pub fn new(session: Session, player: Player, mixer: Box<Mixer + Send>)
        -> (Spirc, SpircTask)
    {
        let ident = session.device_id().to_owned();
        let name = session.config().name.clone();

        let uri = format!("hm://remote/user/{}", session.username());

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
        let device = initial_device_state(name, volume);
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
        };

        let spirc = Spirc {
            commands: cmd_tx,
        };

        task.hello();

        (spirc, task)
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
        match cmd {
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
                if self.state.get_status() == PlayStatus::kPlayStatusPause {
                    self.mixer.start();
                    self.player.play();
                    self.state.set_status(PlayStatus::kPlayStatusPlay);
                    self.state.set_position_measured_at(now_ms() as u64);
                }

                self.notify(None);
            }

            MessageType::kMessageTypePause => {
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

                self.notify(None);
            }

            MessageType::kMessageTypeNext => {
                let current_index = self.state.get_playing_track_index();
                let new_index = (current_index + 1) % (self.state.get_track().len() as u32);

                self.state.set_playing_track_index(new_index);
                self.state.set_position_ms(0);
                self.state.set_position_measured_at(now_ms() as u64);

                self.load_track(true);
                self.notify(None);
            }

            MessageType::kMessageTypePrev => {
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
