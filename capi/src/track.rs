use libc::{c_int, c_char};
use std::ffi::CString;
use std::mem;
use std::ptr::null_mut;

use artist::sp_artist;
use metadata::SpMetadata;
use session::global_session;

use librespot::metadata::{Track, Artist};

#[allow(non_camel_case_types)]
pub type sp_track = SpMetadata<Track>;

#[no_mangle]
pub unsafe extern "C" fn sp_track_is_loaded(c_track: *mut sp_track) -> bool {
    let track = &*c_track;
    track.is_loaded()
}

#[no_mangle]
pub unsafe extern "C" fn sp_track_name(c_track: *mut sp_track) -> *const c_char {
    let track = &*c_track;

    let name = track.get()
                    .map(|metadata| metadata.name.clone())
                    .unwrap_or("".to_owned());

    let name = CString::new(name).unwrap();
    let c_name = name.as_ptr();

    // FIXME
    mem::forget(name);

    c_name
}

#[no_mangle]
pub unsafe extern "C" fn sp_track_num_artists(c_track: *mut sp_track) -> c_int {
    let track = &*c_track;

    track.get()
         .map(|metadata| metadata.artists.len() as c_int)
         .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn sp_track_artist(c_track: *mut sp_track, index: c_int) -> *mut sp_artist {
    let track = &*c_track;
    let session = &*global_session.unwrap();

    track.get()
         .and_then(|metadata| metadata.artists.get(index as usize).map(|x| *x))
         .map(|artist_id| {
             let artist = SpMetadata::from_future(session.metadata::<Artist>(artist_id));
             Box::into_raw(Box::new(artist))
         })
         .unwrap_or(null_mut())
}

