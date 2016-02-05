use metadata::SpMetadata;
use session::SpSession;
use track::sp_track;
use types::sp_error;
use types::sp_error::*;
use std::ffi::CStr;
use std::rc::Rc;
use libc::c_char;
use librespot::link::Link;

#[allow(non_camel_case_types)]
pub type sp_link = Rc<Link>;

#[no_mangle]
pub unsafe extern "C" fn sp_link_create_from_string(uri: *const c_char) -> *mut sp_link {
    let uri = CStr::from_ptr(uri).to_string_lossy();
    let link = Link::from_str(&uri).unwrap();

    Box::into_raw(Box::new(Rc::new(link)))
}

#[no_mangle]
pub unsafe extern "C" fn sp_link_release(c_link: *mut sp_link) -> sp_error {
    drop(Box::from_raw(c_link));

    SP_ERROR_OK
}

#[no_mangle]
pub unsafe extern "C" fn sp_link_as_track(c_link: *mut sp_link) -> *mut sp_track {
    let link = &*c_link;
    let session = SpSession::global();

    let track = SpMetadata::from_future(link.as_track(&session.session).unwrap());
    Box::into_raw(Box::new(track))
}
