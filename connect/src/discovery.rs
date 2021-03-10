use aes_ctr::cipher::generic_array::GenericArray;
use aes_ctr::cipher::{NewStreamCipher, SyncStreamCipher};
use aes_ctr::Aes128Ctr;
use futures_core::Stream;
use hmac::{Hmac, Mac, NewMac};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode};
use num_bigint::BigUint;
use serde_json::json;
use sha1::{Digest, Sha1};
use tokio::sync::{mpsc, oneshot};

#[cfg(feature = "with-dns-sd")]
use dns_sd::DNSService;

use librespot_core::authentication::Credentials;
use librespot_core::config::ConnectConfig;
use librespot_core::diffie_hellman::{DH_GENERATOR, DH_PRIME};
use librespot_core::util;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::convert::Infallible;
use std::io;
use std::net::{Ipv4Addr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

type HmacSha1 = Hmac<Sha1>;

#[derive(Clone)]
struct Discovery(Arc<DiscoveryInner>);
struct DiscoveryInner {
    config: ConnectConfig,
    device_id: String,
    private_key: BigUint,
    public_key: BigUint,
    tx: mpsc::UnboundedSender<Credentials>,
}

impl Discovery {
    fn new(
        config: ConnectConfig,
        device_id: String,
    ) -> (Discovery, mpsc::UnboundedReceiver<Credentials>) {
        let (tx, rx) = mpsc::unbounded_channel();

        let key_data = util::rand_vec(&mut rand::thread_rng(), 95);
        let private_key = BigUint::from_bytes_be(&key_data);
        let public_key = util::powm(&DH_GENERATOR, &private_key, &DH_PRIME);

        let discovery = Discovery(Arc::new(DiscoveryInner {
            config,
            device_id,
            private_key,
            public_key,
            tx,
        }));

        (discovery, rx)
    }

    fn handle_get_info(&self, _: BTreeMap<Cow<'_, str>, Cow<'_, str>>) -> Response<hyper::Body> {
        let public_key = self.0.public_key.to_bytes_be();
        let public_key = base64::encode(&public_key);

        let result = json!({
            "status": 101,
            "statusString": "ERROR-OK",
            "spotifyError": 0,
            "version": "2.7.1",
            "deviceID": (self.0.device_id),
            "remoteName": (self.0.config.name),
            "activeUser": "",
            "publicKey": (public_key),
            "deviceType": (self.0.config.device_type.to_string().to_uppercase()),
            "libraryVersion": "0.1.0",
            "accountReq": "PREMIUM",
            "brandDisplayName": "librespot",
            "modelDisplayName": "librespot",
            "resolverVersion": "0",
            "groupStatus": "NONE",
            "voiceSupport": "NO",
        });

        let body = result.to_string();
        Response::new(Body::from(body))
    }

    fn handle_add_user(
        &self,
        params: BTreeMap<Cow<'_, str>, Cow<'_, str>>,
    ) -> Response<hyper::Body> {
        let username = params.get("userName").unwrap().as_ref();
        let encrypted_blob = params.get("blob").unwrap();
        let client_key = params.get("clientKey").unwrap();

        let encrypted_blob = base64::decode(encrypted_blob.as_bytes()).unwrap();

        let client_key = base64::decode(client_key.as_bytes()).unwrap();
        let client_key = BigUint::from_bytes_be(&client_key);

        let shared_key = util::powm(&client_key, &self.0.private_key, &DH_PRIME);

        let iv = &encrypted_blob[0..16];
        let encrypted = &encrypted_blob[16..encrypted_blob.len() - 20];
        let cksum = &encrypted_blob[encrypted_blob.len() - 20..encrypted_blob.len()];

        let base_key = Sha1::digest(&shared_key.to_bytes_be());
        let base_key = &base_key[..16];

        let checksum_key = {
            let mut h = HmacSha1::new_varkey(base_key).expect("HMAC can take key of any size");
            h.update(b"checksum");
            h.finalize().into_bytes()
        };

        let encryption_key = {
            let mut h = HmacSha1::new_varkey(&base_key).expect("HMAC can take key of any size");
            h.update(b"encryption");
            h.finalize().into_bytes()
        };

        let mut h = HmacSha1::new_varkey(&checksum_key).expect("HMAC can take key of any size");
        h.update(encrypted);
        if h.verify(cksum).is_err() {
            warn!("Login error for user {:?}: MAC mismatch", username);
            let result = json!({
                "status": 102,
                "spotifyError": 1,
                "statusString": "ERROR-MAC"
            });

            let body = result.to_string();
            return Response::new(Body::from(body));
        }

        let decrypted = {
            let mut data = encrypted.to_vec();
            let mut cipher = Aes128Ctr::new(
                &GenericArray::from_slice(&encryption_key[0..16]),
                &GenericArray::from_slice(iv),
            );
            cipher.apply_keystream(&mut data);
            String::from_utf8(data).unwrap()
        };

        let credentials =
            Credentials::with_blob(username.to_string(), &decrypted, &self.0.device_id);

        self.0.tx.send(credentials).unwrap();

        let result = json!({
            "status": 101,
            "spotifyError": 0,
            "statusString": "ERROR-OK"
        });

        let body = result.to_string();
        Response::new(Body::from(body))
    }

    fn not_found(&self) -> Response<hyper::Body> {
        let mut res = Response::default();
        *res.status_mut() = StatusCode::NOT_FOUND;
        res
    }

    async fn call(self, request: Request<Body>) -> hyper::Result<Response<Body>> {
        let mut params = BTreeMap::new();

        let (parts, body) = request.into_parts();

        if let Some(query) = parts.uri.query() {
            let query_params = url::form_urlencoded::parse(query.as_bytes());
            params.extend(query_params);
        }

        if parts.method != Method::GET {
            debug!("{:?} {:?} {:?}", parts.method, parts.uri.path(), params);
        }

        let body = hyper::body::to_bytes(body).await?;

        params.extend(url::form_urlencoded::parse(&body));

        Ok(
            match (parts.method, params.get("action").map(AsRef::as_ref)) {
                (Method::GET, Some("getInfo")) => self.handle_get_info(params),
                (Method::POST, Some("addUser")) => self.handle_add_user(params),
                _ => self.not_found(),
            },
        )
    }
}

#[cfg(feature = "with-dns-sd")]
pub struct DiscoveryStream {
    credentials: mpsc::UnboundedReceiver<Credentials>,
    _svc: DNSService,
    _close_tx: oneshot::Sender<Infallible>,
}

#[cfg(not(feature = "with-dns-sd"))]
pub struct DiscoveryStream {
    credentials: mpsc::UnboundedReceiver<Credentials>,
    _svc: libmdns::Service,
    _close_tx: oneshot::Sender<Infallible>,
}

pub fn discovery(
    config: ConnectConfig,
    device_id: String,
    port: u16,
) -> io::Result<DiscoveryStream> {
    let (discovery, creds_rx) = Discovery::new(config.clone(), device_id);
    let (close_tx, close_rx) = oneshot::channel();

    let address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port);

    let make_service = make_service_fn(move |_| {
        let discovery = discovery.clone();
        async move { Ok::<_, hyper::Error>(service_fn(move |request| discovery.clone().call(request))) }
    });

    let server = hyper::Server::bind(&address).serve(make_service);

    let s_port = server.local_addr().port();
    debug!("Zeroconf server listening on 0.0.0.0:{}", s_port);

    tokio::spawn(server.with_graceful_shutdown(async {
        close_rx.await.unwrap_err();
        debug!("Shutting down discovery server");
    }));

    #[cfg(feature = "with-dns-sd")]
    let svc = DNSService::register(
        Some(&*config.name),
        "_spotify-connect._tcp",
        None,
        None,
        s_port,
        &["VERSION=1.0", "CPath=/"],
    )
    .unwrap();

    #[cfg(not(feature = "with-dns-sd"))]
    let responder = libmdns::Responder::spawn(&tokio::runtime::Handle::current())?;

    #[cfg(not(feature = "with-dns-sd"))]
    let svc = responder.register(
        "_spotify-connect._tcp".to_owned(),
        config.name,
        s_port,
        &["VERSION=1.0", "CPath=/"],
    );

    Ok(DiscoveryStream {
        credentials: creds_rx,
        _svc: svc,
        _close_tx: close_tx,
    })
}

impl Stream for DiscoveryStream {
    type Item = Credentials;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.credentials.poll_recv(cx)
    }
}
