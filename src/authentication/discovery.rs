use crypto;
use crypto::mac::Mac;
use crypto::digest::Digest;
use dns_sd::DNSService;
use hyper;
use hyper::net::NetworkListener;
use num::BigUint;
use url;
use rand;
use rustc_serialize::base64::{self, ToBase64, FromBase64};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::sync::{mpsc, Mutex};

use authentication::Credentials;
use diffie_hellman::{DH_GENERATOR, DH_PRIME};
use util;

struct ServerHandler {
    credentials_tx: Mutex<mpsc::Sender<Credentials>>,
    private_key: BigUint,
    public_key: BigUint,
    device_id: String,
    device_name: String,
}

impl ServerHandler {
    fn handle_get_info(&self, _params: &BTreeMap<String, String>,
                       mut response: hyper::server::Response<hyper::net::Fresh>) {

        let public_key = self.public_key.to_bytes_be()
                                        .to_base64(base64::STANDARD);

        let result = json!({
            "status": 101,
            "statusString": "ERROR-OK",
            "spotifyError": 0,
            "version": "2.1.0",
            "deviceID": (self.device_id),
            "remoteName": (self.device_name),
            "activeUser": "",
            "publicKey": (public_key),
            "deviceType": "UNKNOWN",
            "libraryVersion": "0.1.0",
            "accountReq": "PREMIUM",
            "brandDisplayName": "librespot",
            "modelDisplayName": "librespot",
        });

        *response.status_mut() = hyper::status::StatusCode::Ok;
        response.start().unwrap().write_all(result.to_string().as_bytes()).unwrap();
    }

    fn handle_add_user(&self, params: &BTreeMap<String, String>,
                       mut response: hyper::server::Response<hyper::net::Fresh>) {

        let username = params.get("userName").unwrap();
        let encrypted_blob = params.get("blob").unwrap();
        let client_key = params.get("clientKey").unwrap();

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

        let credentials = Credentials::with_blob(username.to_owned(), &decrypted, &self.device_id);

        self.credentials_tx.lock().unwrap().send(credentials).unwrap();

        let result = json!({
            "status": 101,
            "spotifyError": 0,
            "statusString": "ERROR-OK"
        });

        *response.status_mut() = hyper::status::StatusCode::Ok;
        response.start().unwrap().write_all(result.to_string().as_bytes()).unwrap();
    }

    fn not_found(&self, mut response: hyper::server::Response<hyper::net::Fresh>) {

        *response.status_mut() = hyper::status::StatusCode::NotFound
    }
}

impl hyper::server::Handler for ServerHandler {
    fn handle<'a, 'k>(&'a self,
                      mut request: hyper::server::Request<'a, 'k>,
                      response: hyper::server::Response<'a, hyper::net::Fresh>) {

        if let hyper::uri::RequestUri::AbsolutePath(path) = request.uri.clone() {
            let (_, query, _) = url::parse_path(&path).unwrap();
            let mut params = query.map_or(vec![], |q| url::form_urlencoded::parse(q.as_bytes()))
                                  .into_iter().collect::<BTreeMap<_,_>>();

            if request.method == hyper::method::Method::Post {
                let mut body = Vec::new();
                request.read_to_end(&mut body).unwrap();
                let form = url::form_urlencoded::parse(&body);
                params.extend(form);
            }

            match params.get("action").map(AsRef::as_ref) {
                Some("getInfo") => self.handle_get_info(&params, response),
                Some("addUser") => self.handle_add_user(&params, response),
                _ => self.not_found(response),
            }
        } else {
            self.not_found(response)
        }
    }
}

pub fn discovery_login(device_name: &str, device_id: &str) -> Result<Credentials, ()> {
    let (tx, rx) = mpsc::channel();

    let key_data = util::rand_vec(&mut rand::thread_rng(), 95);
    let private_key = BigUint::from_bytes_be(&key_data);
    let public_key = util::powm(&DH_GENERATOR, &private_key, &DH_PRIME);

    let handler = ServerHandler {
        device_name: device_name.to_owned(),
        device_id: device_id.to_owned(),
        private_key: private_key,
        public_key: public_key,
        credentials_tx: Mutex::new(tx),
    };

    let mut listener = hyper::net::HttpListener::new("0.0.0.0:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let mut server = hyper::Server::new(listener).handle(handler).unwrap();

    let _svc = DNSService::register(Some(device_name),
                                    "_spotify-connect._tcp",
                                    None,
                                    None,
                                    port,
                                    &["VERSION=1.0", "CPath=/"]
                                    ).unwrap();

    let cred = rx.recv().unwrap();
    server.close().unwrap();
    Ok(cred)
}
