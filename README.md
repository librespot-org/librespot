[![Build Status](https://travis-ci.org/librespot-org/librespot.svg?branch=dev)](https://travis-ci.org/librespot-org/librespot)
[![Gitter chat](https://badges.gitter.im/librespot-org/librespot.png)](https://gitter.im/librespot-org/spotify-connect-resources)
[![Crates.io](https://img.shields.io/crates/v/librespot.svg)](https://crates.io/crates/librespot)

Current maintainer is [@awiouy](https://github.com/awiouy) folks.

# librespot
*librespot* is an open source client library for Spotify. It enables applications to use Spotify's service to control and play music via various backends, and to act as a Spotify Connect receiver. It is an alternative to the official and [now deprecated](https://pyspotify.mopidy.com/en/latest/#libspotify-s-deprecation) closed-source `libspotify`. Additionally, it will provide extra features which are not available in the official library.

_Note: librespot only works with Spotify Premium. This will remain the case for the foreseeable future, as we are unlikely to work on implementing the features such as limited skips and adverts that would be required to make librespot compliant with free accounts._

## Quick start
We're available on [crates.io](https://crates.io/crates/librespot) as the _librespot_ package. Simply run `cargo install librespot` to install librespot on your system. Check the wiki for more info and possible [usage options](https://github.com/librespot-org/librespot/wiki/Options).

After installation, you can run librespot from the CLI using a command such as `librespot -n "Librespot Speaker" -b 160` to create a speaker called _Librespot Speaker_ serving 160kbps audio.

## This fork
As the origin by [plietar](https://github.com/plietar/) is no longer actively maintained, this organisation and repository have been set up so that the project may be maintained and upgraded in the future.

# Documentation
Documentation is currently a work in progress, contributions are welcome!

There is some brief documentation on how the protocol works in the [docs](https://github.com/librespot-org/librespot/tree/master/docs) folder, 

[COMPILING.md](https://github.com/librespot-org/librespot/blob/master/COMPILING.md) contains detailed instructions on setting up a development environment, and compiling librespot. More general usage and compilation information is available on the [wiki](https://github.com/librespot-org/librespot/wiki).
[CONTRIBUTING.md](https://github.com/librespot-org/librespot/blob/master/CONTRIBUTING.md) also contains our contributing guidelines.

If you wish to learn more about how librespot works overall, the best way is to simply read the code, and ask any questions you have in our [Gitter Room](https://gitter.im/librespot-org/spotify-connect-resources).

# Issues
If you run into a bug when using librespot, please search the existing issues before opening a new one. Chances are, we've encountered it before, and have provided a resolution. If not, please open a new one, and where possible, include the backtrace librespot generates on crashing, along with anything we can use to reproduce the issue, eg. the Spotify URI of the song that caused the crash.

# Building
A quick walk through of the build process is outlined here, while a detailed compilation guide can be found [here](https://github.com/librespot-org/librespot/blob/master/COMPILING.md). 

## Additional Dependencies
We recently switched to using [Rodio](https://github.com/tomaka/rodio) for audio playback by default, hence for macOS and Windows, you should just be able to clone and build librespot (with the command below).
For Linux, you will need to run the additional commands below, depending on your distro.

On Debian/Ubuntu, the following command will install these dependencies :
```shell
sudo apt-get install build-essential libasound2-dev
```

On Fedora systems, the following command will install these dependencies :
```shell
sudo dnf install alsa-lib-devel make gcc
```

librespot currently offers the following selection of [audio backends](https://github.com/librespot-org/librespot/wiki/Audio-Backends).
```
Rodio (default)
ALSA
PortAudio
PulseAudio
JACK
SDL
Pipe
```
Please check the corresponding [compiling entry](https://github.com/librespot-org/librespot/wiki/Compiling#general-dependencies) for backend specific dependencies.

Once you've installed the dependencies and cloned this repository you can build *librespot* with the default backend using Cargo.
```shell
cargo build --release
```

# Packages

librespot is also available via official package system on various operating systems such as Linux, FreeBSD, NetBSD. [Repology](https://repology.org/project/librespot/versions) offers a good overview.

[![Packaging status](https://repology.org/badge/vertical-allrepos/librespot.svg)](https://repology.org/project/librespot/versions)

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
- [rpi-audio-receiver](https://github.com/nicokaiser/rpi-audio-receiver) - easy Raspbian install scripts for Spotifyd, Bluetooth, Shairport and other audio receivers
- [Spotcontrol](https://github.com/badfortrains/spotcontrol) - A golang implementation of a Spotify Connect controller. No playback
functionality.
- [librespot-java](https://github.com/devgianlu/librespot-java) - A Java port of librespot.
- [ncspot](https://github.com/hrkfdn/ncspot) - Cross-platform ncurses Spotify client.
- [ansible-role-librespot](https://github.com/xMordax/ansible-role-librespot/tree/master) - Ansible role that will build, install and configure Librespot.

