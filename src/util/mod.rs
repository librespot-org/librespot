use rand::{Rng,Rand};

mod int128;
mod spotify_id;
mod arcvec;

pub use util::int128::u128;
pub use util::spotify_id::{SpotifyId, FileId};
pub use util::arcvec::ArcVec;

#[macro_export]
macro_rules! eprintln(
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)* ).unwrap()
        }
    )
);
#[macro_export]
macro_rules! eprint(
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            write!(&mut ::std::io::stderr(), $($arg)* ).unwrap()
        }
    )
);

pub fn rand_vec<G: Rng, R: Rand>(rng: &mut G, size: usize) -> Vec<R> {
    let mut vec = Vec::with_capacity(size);

    for _ in 0..size {
        vec.push(R::rand(rng));
    }

    return vec
}

pub fn alloc_buffer(size: usize) -> Vec<u8> {
    let mut vec = Vec::with_capacity(size);
    unsafe {
        vec.set_len(size);
    }

    vec
}

pub mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));

    pub fn version_string() -> String {
        format!("librespot-{}", short_sha())
    }
}

pub enum Either<S,T> {
    Left(S),
    Right(T)
}

pub fn hexdump(data: &[u8]) {
    for b in data.iter() {
        eprint!("{:02X} ", b);
    }
    eprintln!("");
}

