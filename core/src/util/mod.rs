use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::{Rand, Rng};
use std::mem;
use std::ops::{Mul, Rem, Shr};

mod int128;

pub use util::int128::u128;

pub fn rand_vec<G: Rng, R: Rand>(rng: &mut G, size: usize) -> Vec<R> {
    rng.gen_iter().take(size).collect()
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

pub trait ReadSeek: ::std::io::Read + ::std::io::Seek {}
impl<T: ::std::io::Read + ::std::io::Seek> ReadSeek for T {}

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

impl<T: Seq> SeqGenerator<T> {
    pub fn new(value: T) -> Self {
        SeqGenerator(value)
    }

    pub fn get(&mut self) -> T {
        let value = self.0.next();
        mem::replace(&mut self.0, value)
    }
}
