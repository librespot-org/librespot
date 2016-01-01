use eventual::Async;
use protobuf::{self, Message};
use std::sync::{mpsc, MutexGuard};

use util;
use session::Session;
use util::SpotifyId;
use util::version::version_string;
use mercury::{MercuryRequest, MercuryMethod};

use librespot_protocol as protocol;
pub use librespot_protocol::spirc::PlayStatus;

pub struct SpircManager<D: SpircDelegate> {
    delegate: D,
    session: Session,

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

    tracks: Vec<SpotifyId>,
    index: u32
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
    fn end_of_track(&self) -> bool;
}

impl <D: SpircDelegate> SpircManager<D> {
    pub fn new(session: Session, delegate: D)
            -> SpircManager<D> {

        let ident = session.0.data.read().unwrap().device_id.clone();
        let name = session.0.config.device_name.clone();

        SpircManager {
            delegate: delegate,
            session: session,

            state_update_id: 0,
            seq_nr: 0,

            name: name,
            ident: ident,
            device_type: 5,
            can_play: true,

            repeat: false,
            shuffle: false,
            volume: 0x8000,

            is_active: false,
            became_active_at: 0,

            last_command_ident: String::new(),
            last_command_msgid: 0,

            tracks: Vec::new(),
            index: 0
        }
    }

    pub fn run(&mut self) {
        let rx = self.session.mercury_sub(format!("hm://remote/user/{}/",
                    self.session.0.data.read().unwrap().canonical_username.clone()));
        let updates = self.delegate.updates();

        self.notify(true, None);

        loop {
            select! {
                pkt = rx.recv() => {
                    let frame = protobuf::parse_from_bytes::<protocol::spirc::Frame>(
                        pkt.unwrap().payload.first().unwrap()).unwrap();

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
                    let end_of_track = self.delegate.state().end_of_track();
                    if end_of_track {
                        self.index = (self.index + 1) % self.tracks.len() as u32;
                        let track = self.tracks[self.index as usize];
                        self.delegate.load(track, true, 0);
                    } else {
                        self.state_update_id = update_time.unwrap();
                        self.notify(false, None);
                    }
                }
            }
        }
    }

    fn handle(&mut self, frame: protocol::spirc::Frame) {
        if frame.get_recipient().len() > 0 {
            self.last_command_ident = frame.get_ident().to_owned();
            self.last_command_msgid = frame.get_seq_nr();
        }
        match frame.get_typ() {
            protocol::spirc::MessageType::kMessageTypeHello => {
                self.notify(false, Some(frame.get_ident()));
            }
            protocol::spirc::MessageType::kMessageTypeLoad => {
                if !self.is_active {
                    self.is_active = true;
                    self.became_active_at = util::now_ms();
                }


                self.index = frame.get_state().get_playing_track_index();
                self.tracks = frame.get_state().get_track().iter()
                    .map(|track| SpotifyId::from_raw(track.get_gid()))
                    .collect();

                let play = frame.get_state().get_status() == PlayStatus::kPlayStatusPlay;
                let track = self.tracks[self.index as usize];
                let position = frame.get_state().get_position_ms();
                self.delegate.load(track, play, position);
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

    fn notify(&mut self, hello: bool, recipient: Option<&str>) {
        let mut pkt = protobuf_init!(protocol::spirc::Frame::new(), {
            version: 1,
            ident: self.ident.clone(),
            protocol_version: "2.0.0".to_owned(),
            seq_nr: { self.seq_nr += 1; self.seq_nr  },
            typ: if hello {
                protocol::spirc::MessageType::kMessageTypeHello
            } else {
                protocol::spirc::MessageType::kMessageTypeNotify
            },

            device_state: self.device_state(),
            recipient: protobuf::RepeatedField::from_vec(
                recipient.map(|r| vec![r.to_owned()] ).unwrap_or(vec![])
            ),
            state_update_id: self.state_update_id as i64
        });

        if self.is_active {
            pkt.set_state(self.spirc_state());
        }

        self.session.mercury(MercuryRequest{
            method: MercuryMethod::SEND,
            uri: format!("hm://remote/user/{}", 
                         self.session.0.data.read().unwrap().canonical_username.clone()),
            content_type: None,
            payload: vec![ pkt.write_to_bytes().unwrap() ]
        }).await().unwrap();
    }

    fn spirc_state(&self) -> protocol::spirc::State {
        let state = self.delegate.state();
        let (position_ms, position_measured_at) = state.position();

        protobuf_init!(protocol::spirc::State::new(), {
            status: state.status(),
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
                        "album".to_owned(),
                        "playlist".to_owned(),
                        "search".to_owned(),
                        "inbox".to_owned(),
                        "toplist".to_owned(),
                        "starred".to_owned(),
                        "publishedstarred".to_owned(),
                        "track".to_owned(),
                    ]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportedTypes,
                    stringValue => [
                        "audio/local".to_owned(),
                        "audio/track".to_owned(),
                        "local".to_owned(),
                        "track".to_owned(),
                    ]
                }
            ],
        })
    }
}
