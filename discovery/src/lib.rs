//! Advertises this device to Spotify clients in the local network.
//!
//! This device will show up in the list of "available devices".
//! Once it is selected from the list, [`Credentials`] are received.
//! Those can be used to establish a new Session with [`librespot_core`].
//!
//! This library uses mDNS and DNS-SD so that other devices can find it,
//! and spawns an http server to answer requests of Spotify clients.

mod server;

use std::{
    borrow::Cow,
    io,
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::Stream;
use thiserror::Error;

use self::server::DiscoveryServer;

pub use crate::core::Error;
use librespot_core as core;

/// Credentials to be used in [`librespot`](`librespot_core`).
pub use crate::core::authentication::Credentials;

/// Determining the icon in the list of available devices.
pub use crate::core::config::DeviceType;

/// Makes this device visible to Spotify clients in the local network.
///
/// `Discovery` implements the [`Stream`] trait. Every time this device
/// is selected in the list of available devices, it yields [`Credentials`].
pub struct Discovery {
    server: DiscoveryServer,

    #[cfg(not(feature = "with-dns-sd"))]
    _svc: libmdns::Service,
    #[cfg(feature = "with-dns-sd")]
    _svc: dns_sd::DNSService,
}

/// A builder for [`Discovery`].
pub struct Builder {
    server_config: server::Config,
    port: u16,
    zeroconf_ip: Vec<std::net::IpAddr>,
}

/// Errors that can occur while setting up a [`Discovery`] instance.
#[derive(Debug, Error)]
pub enum DiscoveryError {
    #[error("Creating SHA1 block cipher failed")]
    AesError(#[from] aes::cipher::InvalidLength),
    #[error("Setting up dns-sd failed: {0}")]
    DnsSdError(#[from] io::Error),
    #[error("Creating SHA1 HMAC failed for base key {0:?}")]
    HmacError(Vec<u8>),
    #[error("Setting up the HTTP server failed: {0}")]
    HttpServerError(#[from] hyper::Error),
    #[error("Missing params for key {0}")]
    ParamsError(&'static str),
}

impl From<DiscoveryError> for Error {
    fn from(err: DiscoveryError) -> Self {
        match err {
            DiscoveryError::AesError(_) => Error::unavailable(err),
            DiscoveryError::DnsSdError(_) => Error::unavailable(err),
            DiscoveryError::HmacError(_) => Error::invalid_argument(err),
            DiscoveryError::HttpServerError(_) => Error::unavailable(err),
            DiscoveryError::ParamsError(_) => Error::invalid_argument(err),
        }
    }
}

impl Builder {
    /// Starts a new builder using the provided device and client IDs.
    pub fn new<T: Into<String>>(device_id: T, client_id: T) -> Self {
        Self {
            server_config: server::Config {
                name: "Librespot".into(),
                device_type: DeviceType::default(),
                is_group: false,
                device_id: device_id.into(),
                client_id: client_id.into(),
            },
            port: 0,
            zeroconf_ip: vec![],
        }
    }

    /// Sets the name to be displayed. Default is `"Librespot"`.
    pub fn name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.server_config.name = name.into();
        self
    }

    /// Sets the device type which is visible as icon in other Spotify clients. Default is `Speaker`.
    pub fn device_type(mut self, device_type: DeviceType) -> Self {
        self.server_config.device_type = device_type;
        self
    }

    /// Sets whether the device is a group. This affects the icon in Spotify clients. Default is `false`.
    pub fn is_group(mut self, is_group: bool) -> Self {
        self.server_config.is_group = is_group;
        self
    }

    /// Set the ip addresses on which it should listen to incoming connections. The default is all interfaces.
    pub fn zeroconf_ip(mut self, zeroconf_ip: Vec<std::net::IpAddr>) -> Self {
        self.zeroconf_ip = zeroconf_ip;
        self
    }

    /// Sets the port on which it should listen to incoming connections.
    /// The default value `0` means any port.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets up the [`Discovery`] instance.
    ///
    /// # Errors
    /// If setting up the mdns service or creating the server fails, this function returns an error.
    pub fn launch(self) -> Result<Discovery, Error> {
        let mut port = self.port;
        let name = self.server_config.name.clone().into_owned();
        let server = DiscoveryServer::new(self.server_config, &mut port)?;
        let _zeroconf_ip = self.zeroconf_ip;
        let svc;

        #[cfg(feature = "with-dns-sd")]
        {
            svc = dns_sd::DNSService::register(
                Some(name.as_ref()),
                "_spotify-connect._tcp",
                None,
                None,
                port,
                &["VERSION=1.0", "CPath=/"],
            )?;
        }

        #[cfg(not(feature = "with-dns-sd"))]
        {
            let _svc = if !_zeroconf_ip.is_empty() {
                libmdns::Responder::spawn_with_ip_list(
                    &tokio::runtime::Handle::current(),
                    _zeroconf_ip,
                )?
            } else {
                libmdns::Responder::spawn(&tokio::runtime::Handle::current())?
            };
            svc = _svc.register(
                "_spotify-connect._tcp".to_owned(),
                name,
                port,
                &["VERSION=1.0", "CPath=/"],
            );
        }

        Ok(Discovery { server, _svc: svc })
    }
}

impl Discovery {
    /// Starts a [`Builder`] with the provided device id.
    pub fn builder<T: Into<String>>(device_id: T, client_id: T) -> Builder {
        Builder::new(device_id, client_id)
    }

    /// Create a new instance with the specified device id and default paramaters.
    pub fn new<T: Into<String>>(device_id: T, client_id: T) -> Result<Self, Error> {
        Self::builder(device_id, client_id).launch()
    }
}

impl Stream for Discovery {
    type Item = Credentials;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.server).poll_next(cx)
    }
}
