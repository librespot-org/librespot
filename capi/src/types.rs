#![allow(non_camel_case_types, dead_code)]

use libc::{size_t, c_int, c_char, c_void};
use session::sp_session;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum sp_error {
    SP_ERROR_OK = 0,
    SP_ERROR_BAD_API_VERSION = 1,
    SP_ERROR_API_INITIALIZATION_FAILED = 2,
    SP_ERROR_TRACK_NOT_PLAYABLE = 3,
    SP_ERROR_BAD_APPLICATION_KEY = 5,
    SP_ERROR_BAD_USERNAME_OR_PASSWORD = 6,
    SP_ERROR_USER_BANNED = 7,
    SP_ERROR_UNABLE_TO_CONTACT_SERVER = 8,
    SP_ERROR_CLIENT_TOO_OLD = 9,
    SP_ERROR_OTHER_PERMANENT = 10,
    SP_ERROR_BAD_USER_AGENT = 11,
    SP_ERROR_MISSING_CALLBACK = 12,
    SP_ERROR_INVALID_INDATA = 13,
    SP_ERROR_INDEX_OUT_OF_RANGE = 14,
    SP_ERROR_USER_NEEDS_PREMIUM = 15,
    SP_ERROR_OTHER_TRANSIENT = 16,
    SP_ERROR_IS_LOADING = 17,
    SP_ERROR_NO_STREAM_AVAILABLE = 18,
    SP_ERROR_PERMISSION_DENIED = 19,
    SP_ERROR_INBOX_IS_FULL = 20,
    SP_ERROR_NO_CACHE = 21,
    SP_ERROR_NO_SUCH_USER = 22,
    SP_ERROR_NO_CREDENTIALS = 23,
    SP_ERROR_NETWORK_DISABLED = 24,
    SP_ERROR_INVALID_DEVICE_ID = 25,
    SP_ERROR_CANT_OPEN_TRACE_FILE = 26,
    SP_ERROR_APPLICATION_BANNED = 27,
    SP_ERROR_OFFLINE_TOO_MANY_TRACKS = 31,
    SP_ERROR_OFFLINE_DISK_CACHE = 32,
    SP_ERROR_OFFLINE_EXPIRED = 33,
    SP_ERROR_OFFLINE_NOT_ALLOWED = 34,
    SP_ERROR_OFFLINE_LICENSE_LOST = 35,
    SP_ERROR_OFFLINE_LICENSE_ERROR = 36,
    SP_ERROR_LASTFM_AUTH_ERROR = 39,
    SP_ERROR_INVALID_ARGUMENT = 40,
    SP_ERROR_SYSTEM_FAILURE = 41,
}

#[repr(C)]
#[derive(Copy,Clone)]
pub struct sp_session_config {
    pub api_version: c_int,
    pub cache_location: *const c_char,
    pub settings_location: *const c_char,
    pub application_key: *const c_void,
    pub application_key_size: size_t,
    pub user_agent: *const c_char,
    pub callbacks: *const sp_session_callbacks,
    pub userdata: *mut c_void,
    pub compress_playlists: bool,
    pub dont_save_metadata_for_playlists: bool,
    pub initially_unload_playlists: bool,
    pub device_id: *const c_char,
    pub proxy: *const c_char,
    pub proxy_username: *const c_char,
    pub proxy_password: *const c_char,
    pub tracefile: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sp_session_callbacks {
    pub logged_in: Option<unsafe extern "C" fn(session: *mut sp_session,
                                               error: sp_error)>,

    pub logged_out: Option<unsafe extern "C" fn(session: *mut sp_session)>,

    pub metadata_updated: Option<unsafe extern "C" fn(session: *mut sp_session)>,

    pub connection_error: Option<unsafe extern "C" fn(session: *mut sp_session,
                                                      error: sp_error)>,

    pub message_to_user: Option<unsafe extern "C" fn(session: *mut sp_session,
                                                     message: *const c_char)>,

    pub notify_main_thread: Option<unsafe extern "C" fn(session: *mut sp_session)>,

    pub music_delivery: Option<unsafe extern "C" fn(session: *mut sp_session,
                                                                   format: *const sp_audioformat,
                                                                   frames: *const c_void,
                                                                   num_frames: c_int)
                                                                   -> c_int>,

    pub play_token_lost: Option<unsafe extern "C" fn(session: *mut sp_session)>,

    pub log_message: Option<unsafe extern "C" fn(session: *mut sp_session,
                                                                data: *const c_char)>,

    pub end_of_track: Option<unsafe extern "C" fn(session: *mut sp_session)>,

    pub streaming_error: Option<unsafe extern "C" fn(session: *mut sp_session,
                                                                    error: sp_error)>,

    pub userinfo_updated: Option<unsafe extern "C" fn(session: *mut sp_session)>,

    pub start_playback: Option<unsafe extern "C" fn(session: *mut sp_session)>,

    pub stop_playback: Option<unsafe extern "C" fn(session: *mut sp_session)>,

    pub get_audio_buffer_stats: Option<unsafe extern "C" fn(session: *mut sp_session,
                                                            stats: *mut sp_audio_buffer_stats)>,

    pub offline_status_updated: Option<unsafe extern "C" fn(session: *mut sp_session)>,

    pub offline_error: Option<unsafe extern "C" fn(session: *mut sp_session,
                                                   error: sp_error)>,

    pub credentials_blob_updated: Option<unsafe extern "C" fn(session: *mut sp_session,
                                                              blob: *const c_char)>,

    pub connectionstate_updated: Option<unsafe extern "C" fn(session: *mut sp_session)>,

    pub scrobble_error: Option<unsafe extern "C" fn(session: *mut sp_session,
                                                    error: sp_error)>,

    pub private_session_mode_changed: Option<unsafe extern "C" fn(session: *mut sp_session,
                                                                  is_private: bool)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sp_audioformat {
    pub sample_type: sp_sampletype,
    pub sample_rate: c_int,
    pub channels: c_int,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum sp_sampletype {
    SP_SAMPLETYPE_INT16_NATIVE_ENDIAN = 0,
    _Dummy // rust #10292
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sp_audio_buffer_stats {
    pub samples: c_int,
    pub stutter: c_int,
}
