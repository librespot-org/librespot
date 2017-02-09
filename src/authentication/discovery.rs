use base64;
use crypto::digest::Digest;
use crypto::mac::Mac;
use crypto;
use diffie_hellman::{DH_GENERATOR, DH_PRIME};
use futures::sync::mpsc;
use futures::{Future, Stream, BoxFuture};
use hyper::server::{Service, NewService, Request, Response, Http};
use hyper::{self, Get, Post, StatusCode};
use mdns;
use num_bigint::BigUint;
use rand;
use std::collections::BTreeMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Handle;
use url;

use authentication::Credentials;
use util;

#[derive(Clone)]
struct Discovery(Arc<DiscoveryInner>);
struct DiscoveryInner {
    private_key: BigUint,
    public_key: BigUint,
    device_id: String,
    device_name: String,
    tx: mpsc::UnboundedSender<Credentials>,
}

impl Discovery {
    pub fn new(device_name: String, device_id: String)
        -> (Discovery, mpsc::UnboundedReceiver<Credentials>)
    {
        let (tx, rx) = mpsc::unbounded();

        let key_data = util::rand_vec(&mut rand::thread_rng(), 95);
        let private_key = BigUint::from_bytes_be(&key_data);
        let public_key = util::powm(&DH_GENERATOR, &private_key, &DH_PRIME);

        let discovery = Discovery(Arc::new(DiscoveryInner {
            device_name: device_name.to_owned(),
            device_id: device_id.to_owned(),
            private_key: private_key,
            public_key: public_key,
            tx: tx,
        }));

        (discovery, rx)
    }

    pub fn serve(&self, addr: &SocketAddr, handle: &Handle)
        -> hyper::Result<SocketAddr>
    {
        let listener = TcpListener::bind(addr, handle)?;
        let addr = listener.local_addr()?;

        let http = Http::new();
        let svc = self.clone();
        let handle_ = handle.clone();

        let task = listener.incoming().for_each(move |(socket, addr)| {
            http.bind_connection(&handle_, socket, addr, svc.clone());
            Ok(())
        });

        handle.spawn(task.map_err(|e| panic!(e)));

        Ok(addr)
    }
}

impl Discovery {
    fn handle_get_info(&self, _params: &BTreeMap<String, String>)
        -> ::futures::Finished<Response, hyper::Error>
    {
        let public_key = self.0.public_key.to_bytes_be();
        let public_key = base64::encode(&public_key);

        let result = json!({
            "status": 101,
            "statusString": "ERROR-OK",
            "spotifyError": 0,
            "version": "2.1.0",
            "deviceID": (self.0.device_id),
            "remoteName": (self.0.device_name),
            "activeUser": "",
            "publicKey": (public_key),
            "deviceType": "UNKNOWN",
            "libraryVersion": "0.1.0",
            "accountReq": "PREMIUM",
            "brandDisplayName": "librespot",
            "modelDisplayName": "librespot",
        });

        let body = result.to_string();
        ::futures::finished(Response::new().with_body(body))
    }

    fn handle_add_user(&self, params: &BTreeMap<String, String>)
        -> ::futures::Finished<Response, hyper::Error>
    {
        let username = params.get("userName").unwrap();
        let encrypted_blob = params.get("blob").unwrap();
        let client_key = params.get("clientKey").unwrap();

        let encrypted_blob = base64::decode(encrypted_blob).unwrap();

        let client_key = base64::decode(client_key).unwrap();
        let client_key = BigUint::from_bytes_be(&client_key);

        let shared_key = util::powm(&client_key, &self.0.private_key, &DH_PRIME);

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

        let credentials = Credentials::with_blob(username.to_owned(), &decrypted, &self.0.device_id);

        self.0.tx.send(credentials).unwrap();

        let result = json!({
            "status": 101,
            "spotifyError": 0,
            "statusString": "ERROR-OK"
        });

        let body = result.to_string();
        ::futures::finished(Response::new().with_body(body))
    }

    fn not_found(&self)
        -> ::futures::Finished<Response, hyper::Error>
    {
        ::futures::finished(Response::new().with_status(StatusCode::NotFound))
    }
}

impl Service for Discovery {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Response, hyper::Error>;

    fn call(&self, request: Request) -> Self::Future {
        let mut params = BTreeMap::new();

        let (method, uri, _, _, body) = request.deconstruct();
        if let Some(query) = uri.query() {
            params.extend(url::form_urlencoded::parse(query.as_bytes()).into_owned());
        }

        debug!("{:?} {:?} {:?}", method, uri.path(), params);

        let this = self.clone();
        body.fold(Vec::new(), |mut acc, chunk| {
            acc.extend_from_slice(chunk.as_ref());
            Ok::<_, hyper::Error>(acc)
        }).map(move |body| {
            params.extend(url::form_urlencoded::parse(&body).into_owned());
            params
        }).and_then(move |params| {
            match (method, params.get("action").map(AsRef::as_ref)) {
                (Get, Some("getInfo")) => this.handle_get_info(&params),
                (Post, Some("addUser")) => this.handle_add_user(&params),
                _ => this.not_found(),
            }
        }).boxed()
    }
}

impl NewService for Discovery {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

use tokio_core::reactor::Core;

pub fn discovery_login<A,B>(device_name: A, device_id: B) -> Result<Credentials, ()>
    where A: Into<String>,
          B: Into<String>
{
    let device_name = device_name.into();
    let device_id = device_id.into();

    let (discovery, creds_rx) = Discovery::new(device_name.clone(), device_id);

    let creds_rx = creds_rx.into_future()
        .map(move |(creds, _)| creds.unwrap()).map_err(|(e, _)| e);

    let addr = "0.0.0.0:0".parse().unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let listening_addr = discovery.serve(&addr, &handle).unwrap();

    let responder = mdns::Responder::spawn(&handle).unwrap();
    let _svc = responder.register(
        "_spotify-connect._tcp".to_owned(),
        device_name,
        listening_addr.port(),
        &["VERSION=1.0", "CPath=/"]);

    let creds = core.run(creds_rx).unwrap();

    Ok(creds)
}
