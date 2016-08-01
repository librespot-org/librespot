use num::{BigUint, Integer, Zero, One};
use rand::{Rng, Rand};
use std::io;
use std::ops::{Mul, Rem, Shr};
use std::fs;
use std::path::Path;
use std::time::{UNIX_EPOCH, SystemTime};

mod int128;
mod spotify_id;
mod arcvec;
mod subfile;

pub use util::int128::u128;
pub use util::spotify_id::{SpotifyId, FileId};
pub use util::arcvec::ArcVec;
pub use util::subfile::Subfile;

pub fn rand_vec<G: Rng, R: Rand>(rng: &mut G, size: usize) -> Vec<R> {
    rng.gen_iter().take(size).collect()
}

pub trait IgnoreExt {
    fn ignore(self);
}

impl<T, E> IgnoreExt for Result<T, E> {
    fn ignore(self) {
        match self {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}

pub fn now_ms() -> i64 {
    let dur = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => dur,
        Err(err) => err.duration(),
    };
    (dur.as_secs() * 1000 + (dur.subsec_nanos() / 1000_000) as u64) as i64
}

pub fn mkdir_existing(path: &Path) -> io::Result<()> {
    fs::create_dir(path).or_else(|err| {
        if err.kind() == io::ErrorKind::AlreadyExists {
            Ok(())
        } else {
            Err(err)
        }
    })
}

pub fn powm(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let mut base = base.clone();
    let mut exp = exp.clone();
    let mut result: BigUint = One::one();

    while !exp.is_zero() {
        if exp.is_odd() {
            result = result.mul(&base).rem(modulus);
        }
        exp = exp.shr(1);
        base = (&base).mul(&base).rem(modulus);
    }

    result
}

pub struct StrChunks<'s>(&'s str, usize);

pub trait StrChunksExt {
    fn chunks(&self, size: usize) -> StrChunks;
}

impl StrChunksExt for str {
    fn chunks(&self, size: usize) -> StrChunks {
        StrChunks(self, size)
    }
}

impl<'s> Iterator for StrChunks<'s> {
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

pub trait ReadSeek : ::std::io::Read + ::std::io::Seek { }
impl <T: ::std::io::Read + ::std::io::Seek> ReadSeek for T { }

