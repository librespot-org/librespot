use libc::{c_int, c_char};
use std::ffi::CStr;
use std::slice::from_raw_parts;
use std::sync::mpsc;
use std::boxed::FnBox;
use std::sync::Mutex;

use librespot::session::{Session, Config, Bitrate};
use eventual::{Async, AsyncResult, Future};

use cstring_cache::CStringCache;
use types::sp_error;
use types::sp_error::*;
use types::sp_session_config;
use types::sp_session_callbacks;

static mut global_session: Option<(*const sp_session, *const Mutex<mpsc::Sender<SpSessionEvent>>)> = None;

pub type SpSessionEvent = Box<FnBox(&mut SpSession) -> ()>;

pub struct SpSession {
    pub session: Session,
    cache: CStringCache,
    rx: mpsc::Receiver<SpSessionEvent>,

    pub callbacks: &'static sp_session_callbacks,
}

impl SpSession {
    pub unsafe fn global() -> &'static SpSession {
        &*global_session.unwrap().0
    }

    pub fn run<F: FnOnce(&mut SpSession) -> () + 'static>(event: F) {
        let tx = unsafe {
            &*global_session.unwrap().1
        };
        
        tx.lock().unwrap().send(Box::new(event)).unwrap();
    }

    pub fn receive<T, E, F>(future: Future<T, E>, handler: F)
        where T : Send, E: Send,
              F : FnOnce(&mut SpSession, AsyncResult<T, E>) -> () + Send + 'static {

        future.receive(move |result| {
            SpSession::run(move |session| {
                handler(session, result);
            })
        })
    }
}

#[allow(non_camel_case_types)]
pub type sp_session = SpSession;

#[no_mangle]
pub unsafe extern "C" fn sp_session_create(c_config: *const sp_session_config,
                                           c_session: *mut *mut sp_session) -> sp_error {
    assert!(global_session.is_none());

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

    let (tx, rx) = mpsc::channel();

    let session = SpSession {
        session: Session::new(config),
        cache: CStringCache::new(),
        rx: rx,
        callbacks: &*c_config.callbacks,
    };

    let session = Box::into_raw(Box::new(session));
    let tx = Box::into_raw(Box::new(Mutex::new(tx)));

    global_session = Some((session, tx));

    *c_session = session;

    SP_ERROR_OK
}

#[no_mangle]
pub unsafe extern "C" fn sp_session_release(c_session: *mut sp_session) -> sp_error {
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
    let session = &*c_session;

    let username = CStr::from_ptr(c_username).to_string_lossy().into_owned();
    let password = CStr::from_ptr(c_password).to_string_lossy().into_owned();

    {
        let session = session.session.clone();
        SpSession::receive(Future::spawn(move || {
            session.login_password(username, password)
        }), |session, result| {
            result.unwrap();

            {
                let session = session.session.clone();
                ::std::thread::spawn(move || {
                    loop {
                        session.poll();
                    }
                });
            }
        });
    }

    SP_ERROR_OK
}

#[no_mangle]
pub unsafe extern "C" fn sp_session_user_name(c_session: *mut sp_session) -> *const c_char {
    let session = &mut *c_session;

    let username = session.session.username();
    session.cache.intern(&username).as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn sp_session_user_country(c_session: *mut sp_session) -> c_int {
    let session = &*c_session;

    let country = session.session.country();
    country.chars().fold(0, |acc, x| {
        acc << 8 | (x as u32)
    }) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn sp_session_process_events(c_session: *mut sp_session, next_timeout: *mut c_int) -> sp_error {
    let session = &mut *c_session;

    if !next_timeout.is_null() {
        *next_timeout = 10;
    }

    let event = session.rx.recv().unwrap();
    event.call_box((session,));

    SP_ERROR_OK
}
