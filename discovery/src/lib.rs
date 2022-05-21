//! Advertises this device to Spotify clients in the local network.
//!
//! This device will show up in the list of "available devices".
//! Once it is selected from the list, [`Credentials`] are received.
//! Those can be used to establish a new Session with [`librespot_core`].
//!
//! This library uses mDNS and DNS-SD so that other devices can find it,
//! and spawns an http server to answer requests of Spotify clients.

#![warn(clippy::all, missing_docs, rust_2018_idioms)]

mod server;

use std::borrow::Cow;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use librespot_core as core;
use thiserror::Error;

use self::server::DiscoveryServer;

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
}

/// Errors that can occur while setting up a [`Discovery`] instance.
#[derive(Debug, Error)]
pub enum Error {
    /// Setting up service discovery via DNS-SD failed.
    #[error("Setting up dns-sd failed: {0}")]
    DnsSdError(#[from] io::Error),
    /// Setting up the http server failed.
    #[error("Setting up the http server failed: {0}")]
    HttpServerError(#[from] hyper::Error),
}

impl Builder {
    /// Starts a new builder using the provided device id.
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            server_config: server::Config {
                name: "Librespot".into(),
                device_type: DeviceType::default(),
                device_id: device_id.into(),
            },
            port: 0,
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

        #[cfg(feature = "with-dns-sd")]
        let svc = dns_sd::DNSService::register(
            Some(name.as_ref()),
            "_spotify-connect._tcp",
            None,
            None,
            port,
            &["VERSION=1.0", "CPath=/"],
        )
        .map_err(|e| Error::DnsSdError(io::Error::new(io::ErrorKind::Unsupported, e)))?;

        #[cfg(not(feature = "with-dns-sd"))]
        let svc = libmdns::Responder::spawn(&tokio::runtime::Handle::current())?.register(
            "_spotify-connect._tcp".to_owned(),
            name,
            port,
            &["VERSION=1.0", "CPath=/"],
        );

        Ok(Discovery { server, _svc: svc })
    }
}

impl Discovery {
    /// Starts a [`Builder`] with the provided device id.
    pub fn builder(device_id: impl Into<String>) -> Builder {
        Builder::new(device_id)
    }

    /// Create a new instance with the specified device id and default paramaters.
    pub fn new(device_id: impl Into<String>) -> Result<Self, Error> {
        Self::builder(device_id).launch()
    }
}

impl Stream for Discovery {
    type Item = Credentials;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.server).poll_next(cx)
    }
}
