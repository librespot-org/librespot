[![Build Status](https://travis-ci.org/ComlOnline/librespot.svg?branch=master)](https://travis-ci.org/ComlOnline/librespot)

# librespot
*librespot* is an open source client library for Spotify. It enables
applications to use Spotify's service, without using the official but
closed-source libspotify. Additionally, it will provide extra features
which are not available in the official library.

Note: librespot only works with Spotify Premium

# THIS FORK
As the origin is no longer maintained I wanted to have a place for a version of librespot with other peoples forks and features merged.


# THANKS
I've done noting more than make this pretty so big thanks to:  
[plietar](https://github.com/plietar/) for making the thing in the first place.  
[kingosticks](https://github.com/kingosticks/) for the Suffling and Repeat.  
[ipha](https://github.com/ipha/) for the start stop audio sink.  
[fossedihelm](https://github.com/fossedihelm/) for [addind a default inital volume and options for it](https://github.com/ComlOnline/librespot/pull/5)  
[brain0](https://github.com/brain0/) for [making pluseaudio more robust against audio failures](https://github.com/ComlOnline/librespot/pull/6)  

## Building
Rust 1.18.0 or later is required to build librespot.

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
target/release/librespot --username USERNAME --cache CACHEDIR --name DEVICENAME [--initial-volume 20]
```

### All options

| Type     | Short | Long                | Description                                     | Hint        |
|----------|-------|---------------------|-------------------------------------------------|-------------|
| Option   | c     | cache               | Path to a directory where files will be cached. | CACHE       |
| Flag     |       | disable-audio-cache | Disable caching of the audio data.              |             |
| Required | n     | name                | Device name                                     | NAME        |
| Option   |       | device-type         | Displayed device type                           | DEVICE_TYPE |
| Option   | b     | bitrate             | Bitrate (96, 160 or 320). Defaults to 160       | BITRATE     |
| Option   |       | onstart             | Run PROGRAM when playback is about to begin.    |             |
| Option   |       | onstop              | Run PROGRAM when playback has ended.            | PROGRAM     |
| Flag     | v     | verbose             | Enable verbose output                           | PROGRAM     |
| Option   | u     | username            | Username to sign in with                        | USERNAME    |
| Option   | p     | password            | Password                                        | PASSWORD    |
| Flag     |       | disable-discovery   | Disable discovery mode                          |             |
| Option   |       | backend             | Audio backend to use. Use '?' to list options   | BACKEND     |
| Option   |       | device              | Audio device to use. Use '?' to list options    | DEVICE      |
| Option   |       | mixer               | Mixer to use                                    | MIXER       |
| Option   |       | initial-volume      | Initial volume in %, once connected [0-100]     | VOLUME      |

Taken from here:
https://github.com/ComlOnline/librespot/blob/master/src/main.rs#L88

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
