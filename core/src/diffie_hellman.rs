use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use once_cell::sync::Lazy;
use rand::{CryptoRng, Rng};

static DH_GENERATOR: Lazy<BigUint> = Lazy::new(|| BigUint::from_bytes_be(&[0x02]));
static DH_PRIME: Lazy<BigUint> = Lazy::new(|| {
    BigUint::from_bytes_be(&[
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc9, 0x0f, 0xda, 0xa2, 0x21, 0x68, 0xc2,
        0x34, 0xc4, 0xc6, 0x62, 0x8b, 0x80, 0xdc, 0x1c, 0xd1, 0x29, 0x02, 0x4e, 0x08, 0x8a, 0x67,
        0xcc, 0x74, 0x02, 0x0b, 0xbe, 0xa6, 0x3b, 0x13, 0x9b, 0x22, 0x51, 0x4a, 0x08, 0x79, 0x8e,
        0x34, 0x04, 0xdd, 0xef, 0x95, 0x19, 0xb3, 0xcd, 0x3a, 0x43, 0x1b, 0x30, 0x2b, 0x0a, 0x6d,
        0xf2, 0x5f, 0x14, 0x37, 0x4f, 0xe1, 0x35, 0x6d, 0x6d, 0x51, 0xc2, 0x45, 0xe4, 0x85, 0xb5,
        0x76, 0x62, 0x5e, 0x7e, 0xc6, 0xf4, 0x4c, 0x42, 0xe9, 0xa6, 0x3a, 0x36, 0x20, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    ])
});

fn powm(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let mut base = base.clone();
    let mut exp = exp.clone();
    let mut result: BigUint = One::one();

    while !exp.is_zero() {
        if exp.is_odd() {
            result = (result * &base) % modulus;
        }
        exp >>= 1;
        base = (&base * &base) % modulus;
    }

    result
}

pub struct DhLocalKeys {
    private_key: BigUint,
    public_key: BigUint,
}

impl DhLocalKeys {
    pub fn random<R: Rng + CryptoRng>(rng: &mut R) -> DhLocalKeys {
        let private_key = rng.gen_biguint(95 * 8);
        let public_key = powm(&DH_GENERATOR, &private_key, &DH_PRIME);

        DhLocalKeys {
            private_key,
            public_key,
        }
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.public_key.to_bytes_be()
    }

    pub fn shared_secret(&self, remote_key: &[u8]) -> Vec<u8> {
        let shared_key = powm(
            &BigUint::from_bytes_be(remote_key),
            &self.private_key,
            &DH_PRIME,
        );
        shared_key.to_bytes_be()
    }
}
