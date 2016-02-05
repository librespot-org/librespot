use libc::c_char;

use librespot::metadata::Artist;

use metadata::SpMetadata;

#[allow(non_camel_case_types)]
pub type sp_artist = SpMetadata<Artist>;

#[no_mangle]
pub unsafe extern "C" fn sp_artist_is_loaded(c_artist: *mut sp_artist) -> bool {
    let artist = &*c_artist;
    artist.is_loaded()
}

#[no_mangle]
pub unsafe extern "C" fn sp_artist_name(c_artist: *mut sp_artist) -> *const c_char {
    let artist = &mut *c_artist;

    let name = artist.get()
                     .map(|metadata| &metadata.name as &str)
                     .unwrap_or("");

    artist.intern(name).as_ptr()
}
