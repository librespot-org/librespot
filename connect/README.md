[//]: # (This readme is optimized for inline rustdoc, if some links don't work, they will when included in lib.rs)

# Connect

The connect module of librespot. Provides the option to create your own connect device
and stream to it like any other official spotify client.

The [`Spirc`] is the entrypoint to creating your own connect device. It can be
configured with the given [`ConnectConfig`] options and requires some additional data
to start up the device. 

When creating a new [`Spirc`] it returns two items. The [`Spirc`] itself, which is can 
be used as to control the local connect device. And a [`Future`](std::future::Future), 
lets name it `SpircTask`, that starts and executes the event loop of the connect device 
when awaited.

To get an understanding how to handle the `SpircTask`, it is recommended to take look 
at the code of the `librespot` binary. As the [`src/main.rs`](https://github.com/librespot-org/librespot/blob/dev/src/main.rs#L1943) 
file is quite overwhelming to just understand how to handle the `SpircTask` it is 
recommended to ignore all setup code and skip to the main-loop (around line 1940).

# Example

```rust
use std::{future::Future, thread};

use librespot_connect::{ConnectConfig, Spirc};
use librespot_core::{authentication::Credentials, Error, Session, SessionConfig};
use librespot_playback::{
    audio_backend, mixer,
    config::{AudioFormat, PlayerConfig},
    mixer::{MixerConfig, NoOpVolume},
    player::Player
};

async fn create_basic_spirc() -> Result<(Spirc, impl Future<Output=()>), Error> {
    // when using a cache you can acquire the credentials from there
    let credentials = Credentials::with_access_token("access-token-here");
    let session = Session::new(SessionConfig::default(), None);

    let backend = audio_backend::find(None).expect("will default to rodio");

    let player = Player::new(
        PlayerConfig::default(),
        session.clone(),
        Box::new(NoOpVolume),
        move || {
            let format = AudioFormat::default();
            let device = None;
            backend(device, format)
        },
    );

    let mixer = mixer::find(None).expect("will default to SoftMixer");

    Spirc::new(
        ConnectConfig::default(),
        session,
        credentials,
        player,
        mixer(MixerConfig::default())
    ).await
}
```