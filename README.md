# librespot
*librespot* is an open source client library for Spotify. It enables
applications to use Spotify's service, without using the official but
closed-source libspotify. Additionally, it will provide extra features
which are not available in the official library.

## Building
Rust 1.17.0 or later is required to build librespot.

**If you are building librespot on macOS, the homebrew provided rust may fail due to the way in which homebrew installs rust. In this case, uninstall the homebrew version of rust and use [rustup](https://www.rustup.rs/), and librespot should then build.** 

It also requires a C, with portaudio.

On debian / ubuntu, the following command will install these dependencies :
```shell
sudo apt-get install build-essential portaudio19-dev
```

On Fedora systems, the following command will install these dependencies :
```shell
sudo dnf install portaudio-devel make gcc
```

On macOS, using homebrew :
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

## Cross-compiling
A cross compilation environment is provided as a docker image.
Build the image from the root of the project with the following command :

```
$ docker build -t librespot-cross -f contrib/Dockerfile .
```

The resulting image can be used to build librespot for linux x86_64, armhf (compatible e. g. with Raspberry Pi 2 or 3, but not with Raspberry Pi 1 or Zero) and armel.
The compiled binaries will be located in /tmp/librespot-build

```
docker run -v /tmp/librespot-build:/build librespot-cross
```

If only one architecture is desired, cargo can be invoked directly with the appropriate options :
```shell
docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --no-default-features --features alsa-backend
docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features alsa-backend
docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --target arm-unknown-linux-gnueabi --no-default-features --features alsa-backend
```

Don't forget to set the `with-tremor` feature flag if your target device does not have floating-point capabilities.

## Development
When developing *librespot*, it is preferable to use Rust nightly, and build it using the following :
```shell
cargo build --no-default-features --features "nightly portaudio-backend"
```

This produces better compilation error messages than with the default configuration.

## Disclaimer
Using this code to connect to Spotify's API is probably forbidden by them.
Use at your own risk.

## Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/sashahilton00/spotify-connect-resources

## License
Everything in this repository is licensed under the MIT license.

