# Connect
The connect module of librespot.
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
    // todo: panics here because session needs a reactor runtime for tokio
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