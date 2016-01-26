use libc::{c_int, c_char};
use std::ffi::{CStr, CString};
use std::mem;
use std::slice::from_raw_parts;

use librespot::session::{Session, Config, Bitrate};

use types::sp_error;
use types::sp_error::*;
use types::sp_session_config;

pub static mut global_session: Option<*mut Session> = None;

#[allow(non_camel_case_types)]
pub type sp_session = Session;

#[no_mangle]
pub unsafe extern "C" fn sp_session_create(c_config: *const sp_session_config,
                                           c_session: *mut *mut sp_session) -> sp_error {
    assert_eq!(global_session, None);

    let c_config = &*c_config;

    let application_key = from_raw_parts::<u8>(c_config.application_key as *const u8,
                                               c_config.application_key_size);

    let user_agent = CStr::from_ptr(c_config.user_agent).to_string_lossy().into_owned();
    let device_name = CStr::from_ptr(c_config.device_id).to_string_lossy().into_owned();
    let cache_location = CStr::from_ptr(c_config.cache_location).to_string_lossy().into_owned();

    let config = Config {
        application_key: application_key.to_owned(),
        user_agent: user_agent,
        device_name: device_name,
        cache_location: cache_location.into(),
        bitrate: Bitrate::Bitrate160,
    };

    let session = Box::new(Session::new(config));
    let session = Box::into_raw(session);

    global_session = Some(session);
    *c_session = session;

    SP_ERROR_OK
}

#[no_mangle]
pub unsafe extern "C" fn sp_session_release(c_session: *mut sp_session) -> sp_error {
    assert_eq!(global_session, Some(c_session));

    global_session = None;
    drop(Box::from_raw(c_session));

    SP_ERROR_OK
}

#[no_mangle]
pub unsafe extern "C" fn sp_session_login(c_session: *mut sp_session,
                                          c_username: *const c_char,
                                          c_password: *const c_char,
                                          _remember_me: bool,
                                          _blob: *const c_char) -> sp_error {
    assert_eq!(global_session, Some(c_session));

    let session = &*c_session;

    let username = CStr::from_ptr(c_username).to_string_lossy().into_owned();
    let password = CStr::from_ptr(c_password).to_string_lossy().into_owned();

    session.login_password(username, password).unwrap();

    {
        let session = session.clone();
        ::std::thread::spawn(move || {
            loop {
                session.poll();
            }
        });
    }

    SP_ERROR_OK
}

#[no_mangle]
pub unsafe extern "C" fn sp_session_user_name(c_session: *mut sp_session) -> *const c_char {
    assert_eq!(global_session, Some(c_session));

    let session = &*c_session;

    let username = CString::new(session.username()).unwrap();
    let c_username = username.as_ptr();

    // FIXME
    mem::forget(username);

    c_username
}

#[no_mangle]
pub unsafe extern "C" fn sp_session_user_country(c_session: *mut sp_session) -> c_int {
    assert_eq!(global_session, Some(c_session));

    let session = &*c_session;

    let country = session.username();
    country.chars().fold(0, |acc, x| {
        acc << 8 | (x as u32)
    }) as c_int
}
