use futures::StreamExt;
use librespot_discovery::DeviceType;
use sha1::{Digest, Sha1};
use simple_logger::SimpleLogger;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let name = "Librespot";
    let device_id = hex::encode(Sha1::digest(name.as_bytes()));

    let mut server = librespot_discovery::Discovery::builder(device_id)
        .name(name)
        .device_type(DeviceType::Computer)
        .launch()
        .unwrap();

    while let Some(x) = server.next().await {
        println!("Received {:?}", x);
    }
}
