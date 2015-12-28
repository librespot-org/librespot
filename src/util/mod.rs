use num::{BigUint, Integer, Zero, One};
use rand::{Rng,Rand};
use std::io;
use std::ops::{Mul, Rem, Shr};
use std::fs;
use std::path::Path;
use time;

mod int128;
mod spotify_id;
mod arcvec;
mod subfile;
mod zerofile;

pub use util::int128::u128;
pub use util::spotify_id::{SpotifyId, FileId};
pub use util::arcvec::ArcVec;
pub use util::subfile::Subfile;
pub use util::zerofile::ZeroFile;

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

pub mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));

    pub fn version_string() -> String {
        format!("librespot-{}", short_sha())
    }
}

pub fn hexdump(data: &[u8]) {
    for b in data.iter() {
        eprint!("{:02X} ", b);
    }
    eprintln!("");
}

pub trait IgnoreExt {
    fn ignore(self);
}

impl <T, E> IgnoreExt for Result<T, E> {
    fn ignore(self) {
        match self {
            Ok(_)  => (),
            Err(_) => (),
        }
    }
}

pub fn now_ms() -> i64 {
    let ts = time::now_utc().to_timespec();
    ts.sec * 1000 + ts.nsec as i64 / 1000000
}

pub fn mkdir_existing(path: &Path) -> io::Result<()> {
    fs::create_dir(path)
        .or_else(|err| if err.kind() == io::ErrorKind::AlreadyExists {
            Ok(())
        } else {
            Err(err)
        })
}

pub fn powm(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let mut base = base.clone();
    let mut exp = exp.clone();
    let mut result : BigUint = One::one();

    while !exp.is_zero() {
        if exp.is_odd() {
            result = result.mul(&base).rem(modulus);
        }
        exp = exp.shr(1);
        base = (&base).mul(&base).rem(modulus);
    }

    return result;
}

pub struct StrChunks<'s>(&'s str, usize);

pub trait StrChunksExt {
    fn chunks<'s>(&'s self, size: usize) -> StrChunks<'s>;
}

impl StrChunksExt for str {
    fn chunks<'a>(&'a self, size: usize) -> StrChunks<'a> {
        StrChunks(self, size)
    }
}

impl <'s> Iterator for StrChunks<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        let &mut StrChunks(data, size) = self;
        if data.is_empty() {
            None
        } else {
            let ret = Some(&data[..size]);
            self.0 = &data[size..];
            ret
        }
    }
}

