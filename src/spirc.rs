use protobuf::{self, Message, RepeatedField};
use std::borrow::Cow;
use futures::{Future, Stream, Sink, Async, Poll};
use futures::stream::BoxStream;
use futures::sink::BoxSink;
use futures::sync::mpsc;

use mercury::MercuryError;
use player::{Player, PlayerState};
use session::Session;
use util::{now_ms, SpotifyId, SeqGenerator};
use version;

use protocol;
pub use protocol::spirc::PlayStatus;
use protocol::spirc::{MessageType, Frame, DeviceState};

pub struct SpircTask {
    player: Player,

    sequence: SeqGenerator<u32>,

    ident: String,
    device: DeviceState,

    repeat: bool,
    shuffle: bool,

    last_command_ident: String,
    last_command_msgid: u32,

    tracks: Vec<SpotifyId>,
    index: u32,

    subscription: BoxStream<Frame, MercuryError>,
    sender: BoxSink<Frame, MercuryError>,

    updates: mpsc::UnboundedReceiver<PlayerState>,
    commands: mpsc::UnboundedReceiver<SpircCommand>,

    shutdown: bool,
}

pub enum SpircCommand {
    Shutdown
}

pub struct Spirc {
    commands: mpsc::UnboundedSender<SpircCommand>,
}

fn initial_device_state(name: String, volume: u16) -> DeviceState {
    protobuf_init!(DeviceState::new(), {
        sw_version: version::version_string(),
        is_active: false,
        can_play: true,
        volume: volume as u32,
        name: name,
        error_code: 0,
        became_active_at: 0,
        capabilities => [
            @{
                typ: protocol::spirc::CapabilityType::kCanBePlayer,
                intValue => [0]
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
                intValue => [10]
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
    pub fn new(session: Session, player: Player) -> (Spirc, SpircTask) {
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

        let updates = player.observe();

        let (cmd_tx, cmd_rx) = mpsc::unbounded();

        let volume = 0xFFFF;
        let device = initial_device_state(name, volume);
        player.volume(volume);

        let mut task = SpircTask {
            player: player,

            sequence: SeqGenerator::new(1),

            ident: ident,
            device: device,

            repeat: false,
            shuffle: false,

            last_command_ident: String::new(),
            last_command_msgid: 0,

            tracks: Vec::new(),
            index: 0,

            subscription: subscription,
            sender: sender,
            updates: updates,
            commands: cmd_rx,

            shutdown: false,
        };

        let spirc = Spirc {
            commands: cmd_tx,
        };

        task.notify(true, None);

        (spirc, task)
    }

    pub fn shutdown(&mut self) {
        mpsc::UnboundedSender::send(&mut self.commands, SpircCommand::Shutdown).unwrap();
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

                match self.updates.poll().unwrap() {
                    Async::Ready(Some(state)) => {
                        progress = true;
                        self.handle_update(state);
                    }
                    Async::Ready(None) => panic!("player terminated"),
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
    fn handle_update(&mut self, player_state: PlayerState) {
        let end_of_track = player_state.end_of_track();
        if end_of_track {
            self.index = (self.index + 1) % self.tracks.len() as u32;
            let track = self.tracks[self.index as usize];
            self.player.load(track, true, 0);
        } else {
            self.notify_with_player_state(false, None, &player_state);
        }
    }

    fn handle_command(&mut self, cmd: SpircCommand) {
        match cmd {
            SpircCommand::Shutdown => {
                CommandSender::new(self, MessageType::kMessageTypeGoodbye).send();
                self.shutdown = true;
                self.commands.close();
                self.updates.close();
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

        if frame.get_recipient().len() > 0 {
            self.last_command_ident = frame.get_ident().to_owned();
            self.last_command_msgid = frame.get_seq_nr();
        }

        match frame.get_typ() {
            MessageType::kMessageTypeHello => {
                self.notify(false, Some(frame.get_ident()));
            }
            MessageType::kMessageTypeLoad => {
                if !self.device.get_is_active() {
                    self.device.set_is_active(true);
                    self.device.set_became_active_at(now_ms());
                }

                self.reload_tracks(&frame);
                if self.tracks.len() > 0 {
                    let play = frame.get_state().get_status() == PlayStatus::kPlayStatusPlay;
                    let track = self.tracks[self.index as usize];
                    let position = frame.get_state().get_position_ms();
                    self.player.load(track, play, position);
                } else {
                    self.notify(false, Some(frame.get_ident()));
                }
            }
            MessageType::kMessageTypePlay => {
                self.player.play();
            }
            MessageType::kMessageTypePause => {
                self.player.pause();
            }
            MessageType::kMessageTypeNext => {
                self.index = (self.index + 1) % self.tracks.len() as u32;
                let track = self.tracks[self.index as usize];
                self.player.load(track, true, 0);
            }
            MessageType::kMessageTypePrev => {
                self.index = (self.index - 1) % self.tracks.len() as u32;
                let track = self.tracks[self.index as usize];
                self.player.load(track, true, 0);
            }
            MessageType::kMessageTypeSeek => {
                self.player.seek(frame.get_position());
            }
            MessageType::kMessageTypeReplace => {
                self.reload_tracks(&frame);
            }
            MessageType::kMessageTypeNotify => {
                if self.device.get_is_active() && frame.get_device_state().get_is_active() {
                    self.device.set_is_active(false);
                    self.player.stop();
                }
            }
            MessageType::kMessageTypeVolume => {
                let volume = frame.get_volume();
                self.player.volume(volume as u16);
                self.device.set_volume(volume);
                self.notify(false, None);
            }
            MessageType::kMessageTypeGoodbye => (),
            _ => (),
        }
    }

    fn reload_tracks(&mut self, ref frame: &protocol::spirc::Frame) {
        self.index = frame.get_state().get_playing_track_index();
        self.tracks = frame.get_state()
                           .get_track()
                           .iter()
                           .filter(|track| track.has_gid())
                           .map(|track| SpotifyId::from_raw(track.get_gid()))
                           .collect();
    }

    fn notify(&mut self, hello: bool, recipient: Option<&str>) {
        let mut cs = CommandSender::new(self,
                                        if hello {
                                            MessageType::kMessageTypeHello
                                        } else {
                                            MessageType::kMessageTypeNotify
                                        });
        if let Some(s) = recipient {
            cs = cs.recipient(&s);
        }
        cs.send();
    }

    fn notify_with_player_state(&mut self,
                                hello: bool,
                                recipient: Option<&str>,
                                player_state: &PlayerState) {
        let mut cs = CommandSender::new(self,
                                        if hello {
                                            MessageType::kMessageTypeHello
                                        } else {
                                            MessageType::kMessageTypeNotify
                                        })
                         .player_state(player_state);
        if let Some(s) = recipient {
            cs = cs.recipient(&s);
        }
        cs.send();
    }

    fn spirc_state(&self, player_state: &PlayerState) -> protocol::spirc::State {
        let (position_ms, position_measured_at) = player_state.position();

        protobuf_init!(protocol::spirc::State::new(), {
            status: player_state.status(),
            position_ms: position_ms,
            position_measured_at: position_measured_at as u64,

            playing_track_index: self.index,
            track: self.tracks.iter().map(|track| {
                protobuf_init!(protocol::spirc::TrackRef::new(), {
                    gid: track.to_raw().to_vec()
                })
            }).collect(),

            shuffle: self.shuffle,
            repeat: self.repeat,

            playing_from_fallback: true,

            last_command_ident: self.last_command_ident.clone(),
            last_command_msgid: self.last_command_msgid
        })
    }
}

struct CommandSender<'a> {
    spirc: &'a mut SpircTask,
    cmd: MessageType,
    recipient: Option<&'a str>,
    player_state: Option<&'a PlayerState>,
    state: Option<protocol::spirc::State>,
}

impl<'a> CommandSender<'a> {
    fn new(spirc: &'a mut SpircTask, cmd: MessageType) -> CommandSender {
        CommandSender {
            spirc: spirc,
            cmd: cmd,
            recipient: None,
            player_state: None,
            state: None,
        }
    }

    fn recipient(mut self, r: &'a str) -> CommandSender {
        self.recipient = Some(r);
        self
    }

    fn player_state(mut self, s: &'a PlayerState) -> CommandSender {
        self.player_state = Some(s);
        self
    }

    #[allow(dead_code)]
    fn state(mut self, s: protocol::spirc::State) -> CommandSender<'a> {
        self.state = Some(s);
        self
    }

    fn send(self) {
        let state = self.player_state.map_or_else(|| {
            Cow::Owned(self.spirc.player.state())
        }, |s| {
            Cow::Borrowed(s)
        });

        let mut frame = protobuf_init!(Frame::new(), {
            version: 1,
            ident: self.spirc.ident.clone(),
            protocol_version: "2.0.0",
            seq_nr: self.spirc.sequence.get(),
            typ: self.cmd,
            recipient: RepeatedField::from_vec(
                self.recipient.map(|r| vec![r.to_owned()] ).unwrap_or(vec![])
                ),
            device_state: self.spirc.device.clone(),
            state_update_id: state.update_time()
        });

        if self.spirc.device.get_is_active() {
            frame.set_state(self.spirc.spirc_state(&state));
        }

        let ready = self.spirc.sender.start_send(frame).unwrap().is_ready();
        assert!(ready);
    }
}

#[allow(dead_code)]
fn track_ids_to_state<I: Iterator<Item = SpotifyId>>(track_ids: I) -> protocol::spirc::State {
    let tracks: Vec<protocol::spirc::TrackRef> =
        track_ids.map(|i| {
                     protobuf_init!(protocol::spirc::TrackRef::new(), { gid: i.to_raw().to_vec()})
                 })
                 .collect();
    protobuf_init!(protocol::spirc::State::new(), {
                    track: RepeatedField::from_vec(tracks)
                })
}
