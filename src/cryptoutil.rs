use rand;
use gmp::Mpz;
use std::num::FromPrimitive;
use crypto;
use crypto::mac::Mac;
use std::io::Write;

use util;

lazy_static! {
    static ref DH_GENERATOR: Mpz = Mpz::from_u64(0x2).unwrap();
    static ref DH_PRIME: Mpz = Mpz::from_bytes_be(&[
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc9,
        0x0f, 0xda, 0xa2, 0x21, 0x68, 0xc2, 0x34, 0xc4, 0xc6,
        0x62, 0x8b, 0x80, 0xdc, 0x1c, 0xd1, 0x29, 0x02, 0x4e,
        0x08, 0x8a, 0x67, 0xcc, 0x74, 0x02, 0x0b, 0xbe, 0xa6,
        0x3b, 0x13, 0x9b, 0x22, 0x51, 0x4a, 0x08, 0x79, 0x8e,
        0x34, 0x04, 0xdd, 0xef, 0x95, 0x19, 0xb3, 0xcd, 0x3a,
        0x43, 0x1b, 0x30, 0x2b, 0x0a, 0x6d, 0xf2, 0x5f, 0x14,
        0x37, 0x4f, 0xe1, 0x35, 0x6d, 0x6d, 0x51, 0xc2, 0x45,
        0xe4, 0x85, 0xb5, 0x76, 0x62, 0x5e, 0x7e, 0xc6, 0xf4,
        0x4c, 0x42, 0xe9, 0xa6, 0x3a, 0x36, 0x20, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff ]);
}

pub struct Crypto {
    private_key: Mpz,
    public_key: Mpz,
    shared: Option<SharedKeys>,
}

pub struct SharedKeys {
    pub challenge: Vec<u8>,
    pub send_key: Vec<u8>,
    pub recv_key: Vec<u8>
}

impl Crypto {
    pub fn new() -> Crypto {
        let key_data = util::rand_vec(&mut rand::thread_rng(), 95);
        Self::new_with_key(&key_data)
    }

    pub fn new_with_key(key_data: &[u8]) -> Crypto {
        let private_key = Mpz::from_bytes_be(key_data);
        let public_key = DH_GENERATOR.powm(&private_key, &DH_PRIME);

        Crypto {
            private_key: private_key,
            public_key: public_key,
            shared: None,
        }
    }

    pub fn setup(&mut self, remote_key: &[u8], client_packet: &[u8], server_packet: &[u8]) {
        let shared_key = Mpz::from_bytes_be(remote_key).powm(&self.private_key, &DH_PRIME);

        let mut data = Vec::with_capacity(0x54);
        let mut h = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &shared_key.to_bytes_be());

        for i in 1..6 {
            h.input(client_packet);
            h.input(server_packet);
            h.input(&[i]);
            data.write(&h.result().code()).unwrap();
            h.reset();
        }

        h = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &data[..0x14]);
        h.input(client_packet);
        h.input(server_packet);

        self.shared = Some(SharedKeys{
            challenge: h.result().code().to_vec(),
            send_key: data[0x14..0x34].to_vec(),
            recv_key: data[0x34..0x54].to_vec()
        });
    }

    pub fn public_key(&self) -> Vec<u8> {
        return self.public_key.to_bytes_be();
    }

    pub fn shared(&self) -> &SharedKeys {
        match self.shared {
            Some(ref shared) => shared,
            None => panic!("ABC")
        }
    }
}

