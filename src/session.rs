use connection::Connection;
use cryptoutil::Crypto;
use protocol;
use util;
use std::iter::{FromIterator,repeat};

use protobuf::*;
use rand::thread_rng;

pub struct Session {
    connection: Connection,
    crypto: Crypto,
}

impl Session {
    pub fn new() -> Session {
        Session {
            connection: Connection::connect(),
            crypto: Crypto::new(),
        }
    }

    pub fn login(&mut self) {
        let request = protobuf_init!(protocol::keyexchange::Request::new(), {
            data0 => {
                data0: 0x05,
                data1: 0x01,
                data2: 0x10800000000,
            },
            data1: 0,
            data2.data0 => {
                data0: self.crypto.public_key(),
                data1: 1,
            },
            random: util::rand_vec(&mut thread_rng(), 0x10),
            data4: vec![0x1e],
            data5: vec![0x08, 0x01]
        });

        let init_client_packet =
            self.connection.send_packet_prefix(&[0,4], &request.write_to_bytes().unwrap());
        let init_server_packet =
            self.connection.recv_packet();

        let response : protocol::keyexchange::Response =
            parse_from_bytes(&init_server_packet).unwrap();

        protobuf_bind!(response, { data.data0.data0.data0: remote_key });

        self.crypto.setup(&remote_key, &init_client_packet, &init_server_packet);

        return;
        let appkey = vec![];
        let request = protobuf_init!(protocol::authentication::AuthRequest::new(), {
            credentials => {
                username: "USERNAME".to_string(),
                method: protocol::authentication::AuthRequest_LoginMethod::PASSWORD,
                password: b"PASSWORD".to_vec(),
            },
            data1 => {
                data0: 0,
                data1: 0,
                partner: "Partner blabla".to_string(),
                deviceid: "abc".to_string()
            },
            version: "master-v1.8.0-gad9e5b46".to_string(),
            data3 => {
                data0: 1,
                appkey1: appkey[0x1..0x81].to_vec(),
                appkey2: appkey[0x81..0x141].to_vec(),
                data3: "".to_string(),
                data4: Vec::from_iter(repeat(0).take(20))
            }
        });
        //println!("{:?}", response);
    }
}

