use crypto;
use crypto::mac::Mac;
use num::{BigUint, FromPrimitive};
use rand;
use std::io::Write;

use util;

lazy_static! {
    static ref DH_GENERATOR: BigUint = BigUint::from_u64(0x2).unwrap();
    static ref DH_PRIME: BigUint = BigUint::from_bytes_be(&[
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

pub struct PrivateKeys {
    private_key: BigUint,
    public_key: BigUint,
}

pub struct SharedKeys {
    //private: PrivateKeys,
    challenge: Vec<u8>,
    send_key: Vec<u8>,
    recv_key: Vec<u8>
}

impl PrivateKeys {
    pub fn new() -> PrivateKeys {
        let key_data = util::rand_vec(&mut rand::thread_rng(), 95);
        Self::new_with_key(&key_data)
    }

    pub fn new_with_key(key_data: &[u8]) -> PrivateKeys {
        let private_key = BigUint::from_bytes_be(key_data);
        let public_key = util::powm(&DH_GENERATOR, &private_key, &DH_PRIME);

        PrivateKeys {
            private_key: private_key,
            public_key: public_key,
        }
    }

    /*
    pub fn private_key(&self) -> Vec<u8> {
        return self.private_key.to_bytes_be();
    }
    */

    pub fn public_key(&self) -> Vec<u8> {
        return self.public_key.to_bytes_be();
    }

    pub fn add_remote_key(self, remote_key: &[u8], client_packet: &[u8], server_packet: &[u8]) -> SharedKeys {
        let shared_key = util::powm(&BigUint::from_bytes_be(remote_key), &self.private_key, &DH_PRIME);

        let mut data = Vec::with_capacity(0x64);
        let mut mac = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &shared_key.to_bytes_be());

        for i in 1..6 {
            mac.input(client_packet);
            mac.input(server_packet);
            mac.input(&[i]);
            data.write(&mac.result().code()).unwrap();
            mac.reset();
        }

        mac = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &data[..0x14]);
        mac.input(client_packet);
        mac.input(server_packet);

        SharedKeys {
            //private: self,
            challenge: mac.result().code().to_vec(),
            send_key: data[0x14..0x34].to_vec(),
            recv_key: data[0x34..0x54].to_vec(),
        }
    }
}

impl SharedKeys {
    pub fn challenge(&self) -> &[u8] {
        &self.challenge
    }

    pub fn send_key(&self) -> &[u8] {
        &self.send_key
    }

    pub fn recv_key(&self) -> &[u8] {
        &self.recv_key
    }
}

