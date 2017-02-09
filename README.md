# librespot
*librespot* is an open source client library for Spotify. It enables
applications to use Spotify's service, without using the official but
closed-source libspotify. Additionally, it will provide extra features
which are not available in the official library.

## Status
*librespot* is currently under development and is not fully functional yet. You
are however welcome to experiment with it.

## Building
Rust 1.7.0 or later is required to build librespot.

It also requires a C, with portaudio.

On debian / ubuntu, the following command will install these dependencies :
```shell
sudo apt-get install build-essential portaudio19-dev
```

On Fedora systems, the following command will install these dependencies :
```shell
sudo dnf install portaudio-devel make gcc
```

On OS X, using homebrew :
```shell
brew install portaudio
```

Once you've cloned this repository you can build *librespot* using `cargo`.
```shell
cargo build --release
```

## Usage
A sample program implementing a headless Spotify Connect receiver is provided.
Once you've built *librespot*, run it using :
```shell
target/release/librespot --username USERNAME --cache CACHEDIR --name DEVICENAME
```

## Discovery mode
*librespot* can be run in discovery mode, in which case no password is required at startup.
For that, simply omit the `--username` argument.

## Audio Backends
*librespot* supports various audio backends. Multiple backends can be enabled at compile time by enabling the
corresponding cargo feature. By default, only PortAudio is enabled.

A specific backend can selected at runtime using the `--backend` switch.

```shell
cargo build --features portaudio-backend
target/release/librespot [...] --backend portaudio
```

The following backends are currently available :
- ALSA
- PortAudio 
- PulseAudio

## Disclaimer
Using this code to connect to Spotify's API is probably forbidden by them.
Use at your own risk.

## Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/sashahilton00/spotify-connect-resources

## License
Everything in this repository is licensed under the MIT license.

