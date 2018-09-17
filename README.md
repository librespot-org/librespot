[![Build Status](https://travis-ci.org/librespot-org/librespot.svg?branch=master)](https://travis-ci.org/librespot-org/librespot)
[![Gitter chat](https://badges.gitter.im/librespot-org/librespot.png)](https://gitter.im/librespot-org/spotify-connect-resources)

# librespot
*librespot* is an open source client library for Spotify. It enables
applications to use Spotify's service, without using the official but
closed-source libspotify. Additionally, it will provide extra features
which are not available in the official library.

Note: librespot only works with Spotify Premium

## This fork
As the origin by [plietar](https://github.com/plietar/) is no longer actively maintained, this organisation and repository have been set up so that the project may be maintained and upgraded in the future.

# Documentation
Documentation is currently a work in progress. 

There is some brief documentation on how the protocol works in the [docs](https://github.com/librespot-org/librespot/tree/master/docs) folder, and more general usage and compilation information is available on the [wiki](https://github.com/librespot-org/librespot/wiki). 

[CONTRIBUTING.md](https://github.com/librespot-org/librespot/blob/master/CONTRIBUTING.md) also contains detailed instructions on setting up a development environment, compilation, and contributing guidelines. 

If you wish to learn more about how librespot works overall, the best way is to simply read the code, and ask any questions you have in the Gitter chat linked above.

# Issues

If you run into a bug when using librespot, please search the existing issues before opening a new one. Chances are, we've encountered it before, and have provided a resolution. If not, please open a new one, and where possible, include the backtrace librespot generates on crashing, along with anything we can use to reproduce the issue, eg. the Spotify URI of the song that caused the crash.

# Building
Rust 1.21.0 or later is required to build librespot.

**If you are building librespot on macOS, the homebrew provided rust may fail due to the way in which homebrew installs rust. In this case, uninstall the homebrew version of rust and use [rustup](https://www.rustup.rs/), and librespot should then build. This should have been fixed in more recent versions of Homebrew, but we're leaving this notice here as a warning.**

**We strongly suggest you install rust using rustup, for ease of installation and maintenance.**

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
target/release/librespot --name DEVICENAME 
```

The above is a minimal example. Here is a more fully fledged one:
```shell
target/release/librespot -n "Librespot" -b 320 -c ./cache --enable-volume-normalisation --initial-volume 75 --device-type avr
```
The above command will create a receiver named ```Librespot```, with bitrate set to 320kbps, initial volume at 75%, with volume normalisation enabled, and the device displayed in the app as an Audio/Video Receiver. A folder named ```cache``` will be created/used in the current directory, and be used to cache audio data and credentials.

A full list of runtime options are available [here](https://github.com/librespot-org/librespot/wiki/Options)

## Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/librespot-org/spotify-connect-resources

## Disclaimer
Using this code to connect to Spotify's API is probably forbidden by them.
Use at your own risk.

## License
Everything in this repository is licensed under the MIT license.

## Related Projects
This is a non exhaustive list of projects that either use or have modified librespot. If you'd like to include yours, submit a PR.

- [librespot-golang](https://github.com/librespot-org/librespot-golang) - A golang port of librespot.
- [plugin.audio.spotify](https://github.com/marcelveldt/plugin.audio.spotify) - A Kodi plugin for Spotify.
- [raspotify](https://github.com/dtcooper/raspotify) - Spotify Connect client for the Raspberry Pi that Just Works™
- [Spotifyd](https://github.com/Spotifyd/spotifyd) - A stripped down librespot UNIX daemon.
- [Spotcontrol](https://github.com/badfortrains/spotcontrol) - A golang implementation of a Spotify Connect controller. No playback
functionality.
- [librespot-java](https://github.com/devgianlu/librespot-java) - A Java port of librespot.

