use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::Stream;
use librespot_core::authentication::Credentials;
use librespot_core::config::ConnectConfig;

pub struct DiscoveryStream(librespot_discovery::Discovery);

impl Stream for DiscoveryStream {
    type Item = Credentials;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

pub fn discovery(
    config: ConnectConfig,
    device_id: String,
    port: u16,
) -> io::Result<DiscoveryStream> {
    librespot_discovery::Discovery::builder(device_id)
        .device_type(config.device_type)
        .port(port)
        .name(config.name)
        .launch()
        .map(DiscoveryStream)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
