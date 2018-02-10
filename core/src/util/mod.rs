use num_bigint::BigUint;
use num_traits::{Zero, One};
use num_integer::Integer;
use rand::{Rng, Rand};
use std::mem;
use std::ops::{Mul, Rem, Shr};
use std::process::Command;

mod int128;
mod spotify_id;

pub use util::int128::u128;
pub use util::spotify_id::{SpotifyId, FileId};

pub fn rand_vec<G: Rng, R: Rand>(rng: &mut G, size: usize) -> Vec<R> {
    rng.gen_iter().take(size).collect()
}

pub fn run_program(program: &str) {
    info!("Running {}", program);
    let mut v: Vec<&str> = program.split_whitespace().collect();
    let status = Command::new(&v.remove(0))
            .args(&v)
            .status()
            .expect("program failed to start");
    info!("Exit status: {}", status);
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

pub trait Seq {
    fn next(&self) -> Self;
}

macro_rules! impl_seq {
    ($($ty:ty)*) => { $(
        impl Seq for $ty {
            fn next(&self) -> Self { *self + 1 }
        }
    )* }
}

impl_seq!(u8 u16 u32 u64 usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SeqGenerator<T: Seq>(T);

impl <T: Seq> SeqGenerator<T> {
    pub fn new(value: T) -> Self {
        SeqGenerator(value)
    }

    pub fn get(&mut self) -> T {
        let value = self.0.next();
        mem::replace(&mut self.0, value)
    }
}
