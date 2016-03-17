use eventual::Async;
use protobuf::{self, Message, RepeatedField};

use util;
use session::Session;
use util::SpotifyId;
use util::version::version_string;
use mercury::{MercuryRequest, MercuryMethod};
use player::{Player, PlayerState};

use std::borrow::Cow;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;

use protocol;
pub use protocol::spirc::{PlayStatus, MessageType};

#[derive(Clone)]
pub struct SpircManager(Arc<Mutex<SpircInternal>>);

struct SpircInternal {
    player: Player,
    session: Session,

    seq_nr: u32,

    name: String,
    ident: String,
    device_type: u8,
    can_play: bool,

    repeat: bool,
    shuffle: bool,

    is_active: bool,
    became_active_at: i64,

    last_command_ident: String,
    last_command_msgid: u32,

    tracks: Vec<SpotifyId>,
    index: u32,

    devices: HashMap<String, String>,
}

impl SpircManager {
    pub fn new(session: Session, player: Player) -> SpircManager {
        let ident = session.device_id().to_owned();
        let name = session.config().device_name.clone();

        SpircManager(Arc::new(Mutex::new(SpircInternal {
            player: player,
            session: session,

            seq_nr: 0,

            name: name,
            ident: ident,
            device_type: 5,
            can_play: true,

            repeat: false,
            shuffle: false,

            is_active: false,
            became_active_at: 0,

            last_command_ident: String::new(),
            last_command_msgid: 0,

            tracks: Vec::new(),
            index: 0,

            devices: HashMap::new(),
        })))
    }

    pub fn run(&self) {
        let rx = {
            let mut internal = self.0.lock().unwrap();

            let rx = internal.session.mercury_sub(internal.uri());

            internal.notify(true, None);

            // Use a weak pointer to avoid creating an Rc cycle between the player and the
            // SpircManager
            let _self = Arc::downgrade(&self.0);
            internal.player.add_observer(Box::new(move |state| {
                if let Some(_self) = _self.upgrade() {
                    let mut internal = _self.lock().unwrap();
                    internal.on_update(state);
                }
            }));

            rx
        };

        for pkt in rx {
            let data = pkt.payload.first().unwrap();
            let frame = protobuf::parse_from_bytes::<protocol::spirc::Frame>(data).unwrap();

            println!("{:?} {} {} {} {}",
                     frame.get_typ(),
                     frame.get_device_state().get_name(),
                     frame.get_ident(),
                     frame.get_seq_nr(),
                     frame.get_state_update_id());

            self.0.lock().unwrap().handle(frame);
        }
    }

    pub fn devices(&self) -> HashMap<String, String> {
        self.0.lock().unwrap().devices.clone()
    }

    pub fn send_play(&self, recipient: &str) {
        let mut internal = self.0.lock().unwrap();
        CommandSender::new(&mut *internal, MessageType::kMessageTypePlay)
            .recipient(recipient)
            .send();
    }

    pub fn send_pause(&self, recipient: &str) {
        let mut internal = self.0.lock().unwrap();
        CommandSender::new(&mut *internal, MessageType::kMessageTypePause)
            .recipient(recipient)
            .send();
    }

    pub fn send_prev(&self, recipient: &str) {
        let mut internal = self.0.lock().unwrap();
        CommandSender::new(&mut *internal, MessageType::kMessageTypePrev)
            .recipient(recipient)
            .send();
    }

    pub fn send_next(&self, recipient: &str) {
        let mut internal = self.0.lock().unwrap();
        CommandSender::new(&mut *internal, MessageType::kMessageTypeNext)
            .recipient(recipient)
            .send();
    }

    pub fn send_replace_tracks<I: Iterator<Item = SpotifyId>>(&mut self,
                                                              recipient: &str,
                                                              track_ids: I) {
        let state = track_ids_to_state(track_ids);
        let mut internal = self.0.lock().unwrap();
        CommandSender::new(&mut *internal, MessageType::kMessageTypeReplace)
            .recipient(recipient)
            .state(state)
            .send();
    }

    pub fn send_load_tracks<I: Iterator<Item = SpotifyId>>(&mut self,
                                                           recipient: &str,
                                                           track_ids: I) {
        let state = track_ids_to_state(track_ids);
        let mut internal = self.0.lock().unwrap();
        CommandSender::new(&mut *internal, MessageType::kMessageTypeLoad)
            .recipient(recipient)
            .state(state)
            .send();
    }

    pub fn get_queue(&self) -> Vec<SpotifyId> {
        self.0.lock().unwrap().tracks.clone()
    }
}

impl SpircInternal {
    fn on_update(&mut self, player_state: &PlayerState) {
        let end_of_track = player_state.end_of_track();
        if end_of_track {
            self.index = (self.index + 1) % self.tracks.len() as u32;
            let track = self.tracks[self.index as usize];
            self.player.load(track, true, 0);
        } else {
            self.notify_with_player_state(false, None, player_state);
        }
    }

    fn handle(&mut self, frame: protocol::spirc::Frame) {
        if frame.get_ident() == self.ident ||
           (frame.get_recipient().len() > 0 && !frame.get_recipient().contains(&self.ident)) {
            return;
        }

        if frame.get_recipient().len() > 0 {
            self.last_command_ident = frame.get_ident().to_owned();
            self.last_command_msgid = frame.get_seq_nr();
        }

        if frame.has_ident() && !frame.has_goodbye() && frame.has_device_state() {
            self.devices.insert(frame.get_ident().into(),
                                frame.get_device_state().get_name().into());
        }

        match frame.get_typ() {
            MessageType::kMessageTypeHello => {
                self.notify(false, Some(frame.get_ident()));
            }
            MessageType::kMessageTypeLoad => {
                if !self.is_active {
                    self.is_active = true;
                    self.became_active_at = util::now_ms();
                }

                self.reload_tracks(&frame);

                let play = frame.get_state().get_status() == PlayStatus::kPlayStatusPlay;
                let track = self.tracks[self.index as usize];
                let position = frame.get_state().get_position_ms();
                self.player.load(track, play, position);
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
                if self.is_active && frame.get_device_state().get_is_active() {
                    self.is_active = false;
                    self.player.stop();
                }
            }
            MessageType::kMessageTypeVolume => {
                self.player.volume(frame.get_volume() as u16);
            }
            MessageType::kMessageTypeGoodbye => {
                if frame.has_ident() {
                    self.devices.remove(frame.get_ident());
                }
            }
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

    fn device_state(&self, player_state: &PlayerState) -> protocol::spirc::DeviceState {
        protobuf_init!(protocol::spirc::DeviceState::new(), {
            sw_version: version_string(),
            is_active: self.is_active,
            can_play: self.can_play,
            volume: player_state.volume() as u32,
            name: self.name.clone(),
            error_code: 0,
            became_active_at: if self.is_active { self.became_active_at as i64 } else { 0 },
            capabilities => [
                @{
                    typ: protocol::spirc::CapabilityType::kCanBePlayer,
                    intValue => [0]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kDeviceType,
                    intValue => [ self.device_type as i64 ]
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

    fn uri(&self) -> String {
        format!("hm://remote/user/{}", self.session.username())
    }
}

struct CommandSender<'a> {
    spirc_internal: &'a mut SpircInternal,
    cmd: MessageType,
    recipient: Option<&'a str>,
    player_state: Option<&'a PlayerState>,
    state: Option<protocol::spirc::State>,
}

impl<'a> CommandSender<'a> {
    fn new(spirc_internal: &'a mut SpircInternal, cmd: MessageType) -> CommandSender {
        CommandSender {
            spirc_internal: spirc_internal,
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

    fn state(mut self, s: protocol::spirc::State) -> CommandSender<'a> {
        self.state = Some(s);
        self
    }

    fn send(self) {
        let state = self.player_state.map_or_else(|| {
            Cow::Owned(self.spirc_internal.player.state())
        }, |s| {
            Cow::Borrowed(s)
        });

        let mut pkt = protobuf_init!(protocol::spirc::Frame::new(), {
            version: 1,
            ident: self.spirc_internal.ident.clone(),
            protocol_version: "2.0.0",
            seq_nr: { self.spirc_internal.seq_nr += 1; self.spirc_internal.seq_nr  },
            typ: self.cmd,
            recipient: RepeatedField::from_vec(
                self.recipient.map(|r| vec![r.to_owned()] ).unwrap_or(vec![])
                ),
            device_state: self.spirc_internal.device_state(&state),
            state_update_id: state.update_time()
        });

        if self.spirc_internal.is_active {
            pkt.set_state(self.spirc_internal.spirc_state(&state));
        }

        self.spirc_internal
            .session
            .mercury(MercuryRequest {
                method: MercuryMethod::SEND,
                uri: self.spirc_internal.uri(),
                content_type: None,
                payload: vec![pkt.write_to_bytes().unwrap()],
            })
            .fire();
    }
}

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
