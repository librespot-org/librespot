use futures::StreamExt;
use librespot_core::SessionConfig;
use librespot_discovery::DeviceType;
use sha1::{Digest, Sha1};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let name = "Librespot Group";
    let device_id = hex::encode(Sha1::digest(name.as_bytes()));

    let mut server = librespot_discovery::Discovery::builder(
        String::from(name),
        device_id,
        SessionConfig::default().client_id,
        None,
    )
    .device_type(DeviceType::Speaker)
    .is_group(true)
    .build()
    .unwrap();

    while let Some(x) = server.next().await {
        println!("Received {x:?}");
    }
}
