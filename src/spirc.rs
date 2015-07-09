use protobuf::{self, Message};
use std::sync::{mpsc, MutexGuard};

use util;
use session::Session;
use util::SpotifyId;
use util::version::version_string;
use mercury::{MercuryRequest, MercuryMethod};

use librespot_protocol as protocol;
pub use librespot_protocol::spirc::PlayStatus;

pub struct SpircManager<'s, D: SpircDelegate> {
    delegate: D,
    session: &'s Session,

    username: String,
    state_update_id: i64,
    seq_nr: u32,

    name: String,
    ident: String,
    device_type: u8,
    can_play: bool,

    repeat: bool,
    shuffle: bool,
    volume: u16,

    is_active: bool,
    became_active_at: i64,

    last_command_ident: String,
    last_command_msgid: u32,

    track: Option<SpotifyId>
}

pub trait SpircDelegate {
    type State : SpircState;

    fn load(&self, track: SpotifyId,
            start_playing: bool, position_ms: u32);
    fn play(&self);
    fn pause(&self);
    fn seek(&self, position_ms: u32);
    fn stop(&self);

    fn state(&self) -> MutexGuard<Self::State>;
    fn updates(&self) -> mpsc::Receiver<i64>;
}

pub trait SpircState {
    fn status(&self) -> PlayStatus;
    fn position(&self) -> (u32, i64);
    fn update_time(&self) -> i64;
}

impl <'s, D: SpircDelegate> SpircManager<'s, D> {
    pub fn new(session: &'s Session, delegate: D,
               username: String, name: String) -> SpircManager<'s, D> {
        SpircManager {
            delegate: delegate,
            session: &session,

            username: username.clone(),
            state_update_id: 0,
            seq_nr: 0,

            name: name,
            ident: session.config.device_id.clone(),
            device_type: 5,
            can_play: true,

            repeat: false,
            shuffle: false,
            volume: 0x8000,

            is_active: false,
            became_active_at: 0,

            last_command_ident: String::new(),
            last_command_msgid: 0,

            track: None
        }
    }

    pub fn run(&mut self) {
        let rx = self.session.mercury_sub(format!("hm://remote/user/{}/v23", self.username));
        let updates = self.delegate.updates();

        loop {
            select! {
                pkt = rx.recv() => {
                    let frame = protobuf::parse_from_bytes::<protocol::spirc::Frame>(
                        pkt.unwrap().payload.front().unwrap()).unwrap();
                    println!("{:?} {} {} {} {}",
                             frame.get_typ(),
                             frame.get_device_state().get_name(),
                             frame.get_ident(),
                             frame.get_seq_nr(),
                             frame.get_state_update_id());
                    if frame.get_ident() != self.ident &&
                        (frame.get_recipient().len() == 0 ||
                         frame.get_recipient().contains(&self.ident)) {
                            self.handle(frame);
                        }
                },
                update_time = updates.recv() => {
                    self.state_update_id = update_time.unwrap();
                    self.notify(None);
                }
            }
        }
    }

    fn handle(&mut self, frame: protocol::spirc::Frame) {
        if frame.get_recipient().len() > 0 {
            self.last_command_ident = frame.get_ident().to_string();
            self.last_command_msgid = frame.get_seq_nr();
        }
        match frame.get_typ() {
            protocol::spirc::MessageType::kMessageTypeHello => {
                self.notify(Some(frame.get_ident()));
            }
            protocol::spirc::MessageType::kMessageTypeLoad => {
                if !self.is_active {
                    self.is_active = true;
                    self.became_active_at = util::now_ms();
                }

                let index = frame.get_state().get_playing_track_index() as usize;
                let track = SpotifyId::from_raw(frame.get_state().get_track()[index].get_gid());
                let play = frame.get_state().get_status() == PlayStatus::kPlayStatusPlay;
                self.track = Some(track);
                self.delegate.load(track, play, frame.get_state().get_position_ms());
            }
            protocol::spirc::MessageType::kMessageTypePlay => {
                self.delegate.play();
            }
            protocol::spirc::MessageType::kMessageTypePause => {
                self.delegate.pause();
            }
            protocol::spirc::MessageType::kMessageTypeSeek => {
                self.delegate.seek(frame.get_position());
            }
            protocol::spirc::MessageType::kMessageTypeNotify => {
                if self.is_active && frame.get_device_state().get_is_active() {
                    self.is_active = false;
                    self.delegate.stop();
                }
            }
            _ => ()
        }
    }

    fn notify(&mut self, recipient: Option<&str>) {
        let mut pkt = protobuf_init!(protocol::spirc::Frame::new(), {
            version: 1,
            ident: self.ident.clone(),
            protocol_version: "2.0.0".to_string(),
            seq_nr: { self.seq_nr += 1; self.seq_nr  },
            typ: protocol::spirc::MessageType::kMessageTypeNotify,
            device_state: self.device_state(),
            recipient: protobuf::RepeatedField::from_vec(
                recipient.map(|r| vec![r.to_string()] ).unwrap_or(vec![])
            ),
            state_update_id: self.state_update_id as i64
        });

        if self.is_active {
            pkt.set_state(self.spirc_state());
        }

        self.session.mercury(MercuryRequest{
            method: MercuryMethod::SEND,
            uri: format!("hm://remote/user/{}", self.username),
            content_type: None,
            payload: vec![ pkt.write_to_bytes().unwrap() ]
        });
    }

    fn spirc_state(&self) -> protocol::spirc::State {
        let state = self.delegate.state();
        let (position_ms, position_measured_at) = state.position();

        protobuf_init!(protocol::spirc::State::new(), {
            status: state.status(),
            position_ms: position_ms,
            position_measured_at: position_measured_at as u64,

            playing_track_index: 0,
            track => [
                @{
                    gid: self.track.unwrap().to_raw().to_vec()
                }
            ],

            shuffle: self.shuffle,
            repeat: self.repeat,

            playing_from_fallback: true,

            last_command_ident: self.last_command_ident.clone(),
            last_command_msgid: self.last_command_msgid
        })
    }

    fn device_state(&self) -> protocol::spirc::DeviceState {
        protobuf_init!(protocol::spirc::DeviceState::new(), {
            sw_version: version_string(),
            is_active: self.is_active,
            can_play: self.can_play,
            volume: self.volume as u32,
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
                        "album".to_string(),
                        "playlist".to_string(),
                        "search".to_string(),
                        "inbox".to_string(),
                        "toplist".to_string(),
                        "starred".to_string(),
                        "publishedstarred".to_string(),
                        "track".to_string(),
                    ]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportedTypes,
                    stringValue => [
                        "audio/local".to_string(),
                        "audio/track".to_string(),
                        "local".to_string(),
                        "track".to_string(),
                    ]
                }
            ],
        })
    }
}
