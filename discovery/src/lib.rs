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
    any::Any,
    borrow::Cow,
    error::Error as StdError,
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

    /// An opaque handle to the DNS-SD service. Dropping this will unregister the service.
    #[allow(unused)]
    svc: Box<dyn Any>,
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
    DnsSdError(#[source] Box<dyn StdError + Send + Sync>),

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

const DNS_SD_SERVICE_NAME: &str = "_spotify-connect._tcp";
const TXT_RECORD: [&str; 2] = ["VERSION=1.0", "CPath=/"];

#[cfg(feature = "with-dns-sd")]
fn launch_dns_sd(
    name: Cow<'static, str>,
    _zeroconf_ip: Vec<std::net::IpAddr>,
    port: u16,
) -> Result<Box<dyn Any>, Error> {
    let svc = dns_sd::DNSService::register(
        Some(name.as_ref()),
        DNS_SD_SERVICE_NAME,
        None,
        None,
        port,
        &TXT_RECORD,
    )
    .map_err(|e| DiscoveryError::DnsSdError(Box::new(e)))?;

    Ok(Box::new(svc))
}

#[cfg(not(feature = "with-dns-sd"))]
fn launch_libmdns(
    name: Cow<'static, str>,
    zeroconf_ip: Vec<std::net::IpAddr>,
    port: u16,
) -> Result<Box<dyn Any>, Error> {
    let svc = if !zeroconf_ip.is_empty() {
        libmdns::Responder::spawn_with_ip_list(&tokio::runtime::Handle::current(), zeroconf_ip)
    } else {
        libmdns::Responder::spawn(&tokio::runtime::Handle::current())
    }
    .map_err(|e| DiscoveryError::DnsSdError(Box::new(e)))?
    .register(
        DNS_SD_SERVICE_NAME.to_owned(),
        name.into_owned(),
        port,
        &TXT_RECORD,
    );

    Ok(Box::new(svc))
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
        let name = self.server_config.name.clone();
        let zeroconf_ip = self.zeroconf_ip;

        let mut port = self.port;
        let server = DiscoveryServer::new(self.server_config, &mut port)?;

        #[cfg(feature = "with-dns-sd")]
        let svc = launch_dns_sd(name, zeroconf_ip, port)?;

        #[cfg(not(feature = "with-dns-sd"))]
        let svc = launch_libmdns(name, zeroconf_ip, port)?;

        Ok(Discovery { server, svc })
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
