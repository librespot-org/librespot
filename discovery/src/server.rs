use std::{
    borrow::Cow,
    collections::BTreeMap,
    convert::Infallible,
    net::{Ipv4Addr, SocketAddr, TcpListener},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use aes::cipher::{KeyIvInit, StreamCipher};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use bytes::Bytes;
use futures_core::Stream;
use futures_util::{FutureExt, TryFutureExt};
use hmac::{Hmac, Mac};
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming, Method, Request, Response, StatusCode};

use hyper_util::{rt::TokioIo, server::graceful::GracefulShutdown};
use log::{debug, error, warn};
use serde_json::json;
use sha1::{Digest, Sha1};
use tokio::sync::{mpsc, oneshot};

use super::DiscoveryError;

use crate::{
    core::config::DeviceType,
    core::{authentication::Credentials, diffie_hellman::DhLocalKeys, Error},
};

type Aes128Ctr = ctr::Ctr128BE<aes::Aes128>;

type Params<'a> = BTreeMap<Cow<'a, str>, Cow<'a, str>>;

pub struct Config {
    pub name: Cow<'static, str>,
    pub device_type: DeviceType,
    pub device_id: String,
    pub is_group: bool,
    pub client_id: String,
}

struct RequestHandler {
    config: Config,
    username: Mutex<Option<String>>,
    keys: DhLocalKeys,
    tx: mpsc::UnboundedSender<Credentials>,
}

impl RequestHandler {
    fn new(config: Config) -> (Self, mpsc::UnboundedReceiver<Credentials>) {
        let (tx, rx) = mpsc::unbounded_channel();

        let discovery = Self {
            config,
            username: Mutex::new(None),
            keys: DhLocalKeys::random(&mut rand::thread_rng()),
            tx,
        };

        (discovery, rx)
    }

    fn active_user(&self) -> String {
        if let Ok(maybe_username) = self.username.lock() {
            maybe_username.clone().unwrap_or(String::new())
        } else {
            warn!("username lock corrupted; read failed");
            String::from("!")
        }
    }

    fn handle_get_info(&self) -> Response<Full<Bytes>> {
        let public_key = BASE64.encode(self.keys.public_key());
        let device_type: &str = self.config.device_type.into();
        let active_user = self.active_user();

        // options based on zeroconf guide, search for `groupStatus` on page
        let group_status = if self.config.is_group {
            "GROUP"
        } else {
            "NONE"
        };

        // See: https://developer.spotify.com/documentation/commercial-hardware/implementation/guides/zeroconf/
        let body = json!({
            "status": 101,
            "statusString": "OK",
            "spotifyError": 0,
            // departing from the Spotify documentation, Google Cast uses "5.0.0"
            "version": "2.9.0",
            "deviceID": (self.config.device_id),
            "deviceType": (device_type),
            "remoteName": (self.config.name),
            // valid value seen in the wild: "empty"
            "publicKey": (public_key),
            "brandDisplayName": "librespot",
            "modelDisplayName": "librespot",
            "libraryVersion": crate::core::version::SEMVER,
            "resolverVersion": "1",
            // valid values are "GROUP" and "NONE"
            "groupStatus": group_status,
            // valid value documented & seen in the wild: "accesstoken"
            // Using it will cause clients to fail to connect.
            "tokenType": "default",
            "clientID": (self.config.client_id),
            "productID": 0,
            // Other known scope: client-authorization-universal
            // Comma-separated.
            "scope": "streaming",
            "availability": "",
            "supported_drm_media_formats": [],
            // TODO: bitmask but what are the flags?
            "supported_capabilities": 1,
            // undocumented but should still work
            "accountReq": "PREMIUM",
            "activeUser": active_user,
            // others seen-in-the-wild:
            // - "deviceAPI_isGroup": False
        })
        .to_string();
        let body = Bytes::from(body);
        Response::new(Full::new(body))
    }

    fn handle_add_user(&self, params: &Params<'_>) -> Result<Response<Full<Bytes>>, Error> {
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

        let encrypted_blob = BASE64.decode(encrypted_blob.as_bytes())?;

        let client_key = BASE64.decode(client_key.as_bytes())?;
        let shared_key = self.keys.shared_secret(&client_key);

        let encrypted_blob_len = encrypted_blob.len();
        if encrypted_blob_len < 16 {
            return Err(DiscoveryError::HmacError(encrypted_blob.to_vec()).into());
        }

        let iv = &encrypted_blob[0..16];
        let encrypted = &encrypted_blob[16..encrypted_blob_len - 20];
        let cksum = &encrypted_blob[encrypted_blob_len - 20..encrypted_blob_len];

        let base_key = Sha1::digest(shared_key);
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
        if h.verify_slice(cksum).is_err() {
            warn!("Login error for user {:?}: MAC mismatch", username);
            let result = json!({
                "status": 102,
                "spotifyError": 1,
                "statusString": "ERROR-MAC"
            });

            let body = result.to_string();
            let body = Bytes::from(body);
            return Ok(Response::new(Full::new(body)));
        }

        let decrypted = {
            let mut data = encrypted.to_vec();
            let mut cipher = Aes128Ctr::new_from_slices(&encryption_key[0..16], iv)
                .map_err(DiscoveryError::AesError)?;
            cipher.apply_keystream(&mut data);
            data
        };

        let credentials = Credentials::with_blob(username, decrypted, &self.config.device_id)?;

        {
            let maybe_username = self.username.lock();
            self.tx.send(credentials)?;
            if let Ok(mut username_field) = maybe_username {
                *username_field = Some(String::from(username));
            } else {
                warn!("username lock corrupted; write failed");
            }
        }

        let result = json!({
            "status": 101,
            "spotifyError": 0,
            "statusString": "OK",
        });

        let body = result.to_string();
        let body = Bytes::from(body);
        Ok(Response::new(Full::new(body)))
    }

    fn not_found(&self) -> Response<Full<Bytes>> {
        let mut res = Response::default();
        *res.status_mut() = StatusCode::NOT_FOUND;
        res
    }

    async fn handle(
        self: Arc<Self>,
        request: Request<Incoming>,
    ) -> Result<hyper::Result<Response<Full<Bytes>>>, Error> {
        let mut params = Params::new();

        let (parts, body) = request.into_parts();

        if let Some(query) = parts.uri.query() {
            let query_params = form_urlencoded::parse(query.as_bytes());
            params.extend(query_params);
        }

        if parts.method != Method::GET {
            debug!("{:?} {:?} {:?}", parts.method, parts.uri.path(), params);
        }

        let body = body.collect().await?.to_bytes();

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
    pub fn new(config: Config, port: &mut u16) -> Result<Self, Error> {
        let (discovery, cred_rx) = RequestHandler::new(config);
        let address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), *port);

        let (close_tx, close_rx) = oneshot::channel();

        let listener = match TcpListener::bind(address) {
            Ok(listener) => listener,
            Err(e) => {
                warn!("Discovery server failed to start: {e}");
                return Err(e.into());
            }
        };

        listener.set_nonblocking(true)?;
        let listener = tokio::net::TcpListener::from_std(listener)?;

        match listener.local_addr() {
            Ok(addr) => {
                *port = addr.port();
                debug!("Zeroconf server listening on 0.0.0.0:{}", *port);
            }
            Err(e) => {
                warn!("Discovery server failed to start: {e}");
                return Err(e.into());
            }
        }

        tokio::spawn(async move {
            let discovery = Arc::new(discovery);

            let server = hyper::server::conn::http1::Builder::new();
            let graceful = GracefulShutdown::new();
            let mut close_rx = std::pin::pin!(close_rx);
            loop {
                tokio::select! {
                    Ok((stream, _)) = listener.accept() => {
                        let io = TokioIo::new(stream);
                        let discovery = discovery.clone();

                        let svc = hyper::service::service_fn(move |request| {
                            discovery
                                .clone()
                                .handle(request)
                                .inspect_err(|e| error!("could not handle discovery request: {}", e))
                                .and_then(|x| async move { Ok(x) })
                                .map(Result::unwrap) // guaranteed by `and_then` above
                        });

                        let conn = server.serve_connection(io, svc);
                        let fut = graceful.watch(conn);
                        tokio::spawn(async move {
                            // Errors are logged in the service_fn
                            let _ = fut.await;
                        });
                    }
                    _ = &mut close_rx => {
                        debug!("Shutting down discovery server");
                        break;
                    }
                }
            }

            graceful.shutdown().await;
            debug!("Discovery server stopped");
        });

        Ok(Self {
            cred_rx,
            _close_tx: close_tx,
        })
    }
}

impl Stream for DiscoveryServer {
    type Item = Credentials;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Credentials>> {
        self.cred_rx.poll_recv(cx)
    }
}
