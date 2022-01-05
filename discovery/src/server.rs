use std::{
    borrow::Cow,
    collections::BTreeMap,
    convert::Infallible,
    net::{Ipv4Addr, SocketAddr},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use aes_ctr::{
    cipher::generic_array::GenericArray,
    cipher::{NewStreamCipher, SyncStreamCipher},
    Aes128Ctr,
};
use futures_core::Stream;
use futures_util::{FutureExt, TryFutureExt};
use hmac::{Hmac, Mac, NewMac};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, StatusCode,
};
use log::{debug, error, warn};
use serde_json::json;
use sha1::{Digest, Sha1};
use tokio::sync::{mpsc, oneshot};

use super::DiscoveryError;

use crate::{
    connect::config::DeviceType,
    core::{authentication::Credentials, diffie_hellman::DhLocalKeys, Error},
};

type Params<'a> = BTreeMap<Cow<'a, str>, Cow<'a, str>>;

pub struct Config {
    pub name: Cow<'static, str>,
    pub device_type: DeviceType,
    pub device_id: String,
}

struct RequestHandler {
    config: Config,
    keys: DhLocalKeys,
    tx: mpsc::UnboundedSender<Credentials>,
}

impl RequestHandler {
    fn new(config: Config) -> (Self, mpsc::UnboundedReceiver<Credentials>) {
        let (tx, rx) = mpsc::unbounded_channel();

        let discovery = Self {
            config,
            keys: DhLocalKeys::random(&mut rand::thread_rng()),
            tx,
        };

        (discovery, rx)
    }

    fn handle_get_info(&self) -> Response<hyper::Body> {
        let public_key = base64::encode(&self.keys.public_key());
        let device_type: &str = self.config.device_type.into();

        let body = json!({
            "status": 101,
            "statusString": "ERROR-OK",
            "spotifyError": 0,
            "version": crate::core::version::SEMVER,
            "deviceID": (self.config.device_id),
            "remoteName": (self.config.name),
            "activeUser": "",
            "publicKey": (public_key),
            "deviceType": (device_type),
            "libraryVersion": crate::core::version::SEMVER,
            "accountReq": "PREMIUM",
            "brandDisplayName": "librespot",
            "modelDisplayName": "librespot",
            "resolverVersion": "0",
            "groupStatus": "NONE",
            "voiceSupport": "NO",
        })
        .to_string();

        Response::new(Body::from(body))
    }

    fn handle_add_user(&self, params: &Params<'_>) -> Result<Response<hyper::Body>, Error> {
        let username_key = "userName";
        let username = params
            .get(username_key)
            .ok_or(DiscoveryError::ParamsError(username_key))?
            .as_ref();

        let blob_key = "blob";
        let encrypted_blob = params
            .get(blob_key)
            .ok_or(DiscoveryError::ParamsError(blob_key))?;

        let clientkey_key = "clientKey";
        let client_key = params
            .get(clientkey_key)
            .ok_or(DiscoveryError::ParamsError(clientkey_key))?;

        let encrypted_blob = base64::decode(encrypted_blob.as_bytes())?;

        let client_key = base64::decode(client_key.as_bytes())?;
        let shared_key = self.keys.shared_secret(&client_key);

        let encrypted_blob_len = encrypted_blob.len();
        if encrypted_blob_len < 16 {
            return Err(DiscoveryError::HmacError(encrypted_blob.to_vec()).into());
        }

        let iv = &encrypted_blob[0..16];
        let encrypted = &encrypted_blob[16..encrypted_blob_len - 20];
        let cksum = &encrypted_blob[encrypted_blob_len - 20..encrypted_blob_len];

        let base_key = Sha1::digest(&shared_key);
        let base_key = &base_key[..16];

        let checksum_key = {
            let mut h = Hmac::<Sha1>::new_from_slice(base_key)
                .map_err(|_| DiscoveryError::HmacError(base_key.to_vec()))?;
            h.update(b"checksum");
            h.finalize().into_bytes()
        };

        let encryption_key = {
            let mut h = Hmac::<Sha1>::new_from_slice(base_key)
                .map_err(|_| DiscoveryError::HmacError(base_key.to_vec()))?;
            h.update(b"encryption");
            h.finalize().into_bytes()
        };

        let mut h = Hmac::<Sha1>::new_from_slice(&checksum_key)
            .map_err(|_| DiscoveryError::HmacError(base_key.to_vec()))?;
        h.update(encrypted);
        if h.verify(cksum).is_err() {
            warn!("Login error for user {:?}: MAC mismatch", username);
            let result = json!({
                "status": 102,
                "spotifyError": 1,
                "statusString": "ERROR-MAC"
            });

            let body = result.to_string();
            return Ok(Response::new(Body::from(body)));
        }

        let decrypted = {
            let mut data = encrypted.to_vec();
            let mut cipher = Aes128Ctr::new(
                GenericArray::from_slice(&encryption_key[0..16]),
                GenericArray::from_slice(iv),
            );
            cipher.apply_keystream(&mut data);
            data
        };

        let credentials = Credentials::with_blob(username, &decrypted, &self.config.device_id)?;

        self.tx.send(credentials)?;

        let result = json!({
            "status": 101,
            "spotifyError": 0,
            "statusString": "ERROR-OK"
        });

        let body = result.to_string();
        Ok(Response::new(Body::from(body)))
    }

    fn not_found(&self) -> Response<hyper::Body> {
        let mut res = Response::default();
        *res.status_mut() = StatusCode::NOT_FOUND;
        res
    }

    async fn handle(
        self: Arc<Self>,
        request: Request<Body>,
    ) -> Result<hyper::Result<Response<Body>>, Error> {
        let mut params = Params::new();

        let (parts, body) = request.into_parts();

        if let Some(query) = parts.uri.query() {
            let query_params = form_urlencoded::parse(query.as_bytes());
            params.extend(query_params);
        }

        if parts.method != Method::GET {
            debug!("{:?} {:?} {:?}", parts.method, parts.uri.path(), params);
        }

        let body = hyper::body::to_bytes(body).await?;

        params.extend(form_urlencoded::parse(&body));

        let action = params.get("action").map(Cow::as_ref);

        Ok(Ok(match (parts.method, action) {
            (Method::GET, Some("getInfo")) => self.handle_get_info(),
            (Method::POST, Some("addUser")) => self.handle_add_user(&params)?,
            _ => self.not_found(),
        }))
    }
}

pub struct DiscoveryServer {
    cred_rx: mpsc::UnboundedReceiver<Credentials>,
    _close_tx: oneshot::Sender<Infallible>,
}

impl DiscoveryServer {
    pub fn new(config: Config, port: &mut u16) -> Result<hyper::Result<Self>, Error> {
        let (discovery, cred_rx) = RequestHandler::new(config);
        let discovery = Arc::new(discovery);

        let (close_tx, close_rx) = oneshot::channel();

        let address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), *port);

        let make_service = make_service_fn(move |_| {
            let discovery = discovery.clone();
            async move {
                Ok::<_, hyper::Error>(service_fn(move |request| {
                    discovery
                        .clone()
                        .handle(request)
                        .inspect_err(|e| error!("could not handle discovery request: {}", e))
                        .and_then(|x| async move { Ok(x) })
                        .map(Result::unwrap) // guaranteed by `and_then` above
                }))
            }
        });

        let server = hyper::Server::try_bind(&address)?.serve(make_service);

        *port = server.local_addr().port();
        debug!("Zeroconf server listening on 0.0.0.0:{}", *port);

        tokio::spawn(async {
            let result = server
                .with_graceful_shutdown(async {
                    debug!("Shutting down discovery server");
                    if close_rx.await.is_ok() {
                        debug!("unable to close discovery Rx channel completely");
                    }
                })
                .await;

            if let Err(e) = result {
                warn!("Discovery server failed: {}", e);
            }
        });

        Ok(Ok(Self {
            cred_rx,
            _close_tx: close_tx,
        }))
    }
}

impl Stream for DiscoveryServer {
    type Item = Credentials;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Credentials>> {
        self.cred_rx.poll_recv(cx)
    }
}
