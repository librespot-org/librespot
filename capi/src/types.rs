#![allow(non_camel_case_types)]

use libc::size_t;

pub enum sp_session_callbacks {}

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
    pub api_version: ::std::os::raw::c_int,
    pub cache_location: *const ::std::os::raw::c_char,
    pub settings_location: *const ::std::os::raw::c_char,
    pub application_key: *const ::std::os::raw::c_void,
    pub application_key_size: size_t,
    pub user_agent: *const ::std::os::raw::c_char,
    pub callbacks: *const sp_session_callbacks,
    pub userdata: *mut ::std::os::raw::c_void,
    pub compress_playlists: bool,
    pub dont_save_metadata_for_playlists: bool,
    pub initially_unload_playlists: bool,
    pub device_id: *const ::std::os::raw::c_char,
    pub proxy: *const ::std::os::raw::c_char,
    pub proxy_username: *const ::std::os::raw::c_char,
    pub proxy_password: *const ::std::os::raw::c_char,
    pub tracefile: *const ::std::os::raw::c_char,
}

