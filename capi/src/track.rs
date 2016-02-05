use libc::{c_int, c_char};
use std::ptr::null_mut;

use artist::sp_artist;
use metadata::SpMetadata;
use session::SpSession;

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
    let track = &mut *c_track;

    let name = track.get()
                    .map(|metadata| &metadata.name as &str)
                    .unwrap_or("");

    track.intern(name).as_ptr()
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
    let session = SpSession::global();

    track.get()
         .and_then(|metadata| metadata.artists.get(index as usize).map(|x| *x))
         .map(|artist_id| session.session.metadata::<Artist>(artist_id))
         .map(|artist| Box::into_raw(Box::new(SpMetadata::from_future(artist))))
         .unwrap_or(null_mut())
}

