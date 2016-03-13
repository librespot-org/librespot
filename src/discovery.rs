use crypto;
use crypto::mac::Mac;
use crypto::digest::Digest;
use num::BigUint;
use url;
use rand;
use rustc_serialize::base64::{self, ToBase64, FromBase64};
use tiny_http::{Method, Request, Response, ResponseBox, Server};
use zeroconf::DNSService;

use authentication::Credentials;
use session::Session;
use diffie_hellman::{DH_GENERATOR, DH_PRIME};
use util;

pub struct DiscoveryManager {
    session: Session,
    private_key: BigUint,
    public_key: BigUint,
}

fn not_found() -> ResponseBox {
    Response::from_string("Not found").with_status_code(404).boxed()
}

impl DiscoveryManager {
    pub fn new(session: Session) -> DiscoveryManager {
        let key_data = util::rand_vec(&mut rand::thread_rng(), 95);
        let private_key = BigUint::from_bytes_be(&key_data);
        let public_key = util::powm(&DH_GENERATOR, &private_key, &DH_PRIME);

        DiscoveryManager {
            session: session,
            private_key: private_key,
            public_key: public_key,
        }
    }

    fn get_info(&self) -> ResponseBox {
        let public_key = self.public_key.to_bytes_be().to_base64(base64::STANDARD);
        Response::from_string(json!({
            "status": 101,
            "statusString": "ERROR-OK",
            "spotifyError": 0,
            "version": "2.1.0",
            "deviceID": (self.session.device_id()),
            "remoteName": (self.session.config().device_name),
            "activeUser": "",
            "publicKey": (public_key),
            "deviceType": "UNKNOWN",
            "libraryVersion": "0.1.0",
            "accountReq": "PREMIUM",
            "brandDisplayName": "librespot",
            "modelDisplayName": "librespot",
        }).to_string()).boxed()
    }

    fn add_user(&self, params: &[(String, String)]) -> (ResponseBox, Credentials) {
        let &(_, ref username) = params.iter().find(|&&(ref key, _)| key == "userName").unwrap();
        let &(_, ref encrypted_blob) = params.iter().find(|&&(ref key, _)| key == "blob").unwrap();
        let &(_, ref client_key) = params.iter().find(|&&(ref key, _)| key == "clientKey").unwrap();

        let encrypted_blob = encrypted_blob.from_base64().unwrap();

        let client_key = client_key.from_base64().unwrap();
        let client_key = BigUint::from_bytes_be(&client_key);

        let shared_key = util::powm(&client_key, &self.private_key, &DH_PRIME);

        let iv = &encrypted_blob[0..16];
        let encrypted = &encrypted_blob[16..encrypted_blob.len() - 20];
        let cksum = &encrypted_blob[encrypted_blob.len() - 20..encrypted_blob.len()];

        let base_key = {
            let mut data = [0u8; 20];
            let mut h = crypto::sha1::Sha1::new();
            h.input(&shared_key.to_bytes_be());
            h.result(&mut data);
            data[..16].to_owned()
        };

        let checksum_key = {
            let mut h = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &base_key);
            h.input("checksum".as_bytes());
            h.result().code().to_owned()
        };

        let encryption_key = {
            let mut h = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &base_key);
            h.input("encryption".as_bytes());
            h.result().code().to_owned()
        };

        let mac = {
            let mut h = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &checksum_key);
            h.input(encrypted);
            h.result().code().to_owned()
        };

        assert_eq!(&mac[..], cksum);

        let decrypted = {
            let mut data = vec![0u8; encrypted.len()];
            let mut cipher = crypto::aes::ctr(crypto::aes::KeySize::KeySize128,
                                              &encryption_key[0..16],
                                              &iv);
            cipher.process(&encrypted, &mut data);
            String::from_utf8(data).unwrap()
        };

        let response = Response::from_string(json!({
            "status": 101,
            "spotifyError": 0,
            "statusString": "ERROR-OK"
        }).to_string()).boxed();

        let credentials = Credentials::with_blob(username.to_owned(), &decrypted, &self.session.device_id());

        (response, credentials)
    }

    fn handle_request(&self, mut request: Request) -> Option<Credentials> {
        let (_, query, _) = url::parse_path(request.url()).unwrap();
        let mut params = query.map_or(vec![], |q| url::form_urlencoded::parse(q.as_bytes()));

        if *request.method() == Method::Post {
            let mut body = Vec::new();
            request.as_reader().read_to_end(&mut body).unwrap();
            let form = url::form_urlencoded::parse(&body);
            params.extend(form);
        }

        println!("{:?}", params);

        let &(_, ref action) = params.iter().find(|&&(ref key, _)| key == "action").unwrap();
        let (response, credentials) = match action.as_ref() {
            "getInfo" => (self.get_info(), None),
            "addUser" => {
                let (response, credentials) = self.add_user(&params);
                (response, Some(credentials))
            }
            _ => (not_found(), None)
        };

        request.respond(response).unwrap();
        credentials
    }

    pub fn run(&mut self) -> Credentials {
        let server = Server::http("0.0.0.0:8000").unwrap();
        let _svc = DNSService::register(Some(&self.session.config().device_name),
                                        "_spotify-connect._tcp",
                                        None,
                                        None,
                                        8000,
                                        &["VERSION=1.0", "CPath=/"]
                                        ).unwrap();

        for request in server.incoming_requests() {
            if let Some(credentials) = self.handle_request(request) {
                return credentials;
            }
        }

        panic!("No credentials obtained !");
    }
}
