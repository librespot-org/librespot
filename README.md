[![Build Status](https://travis-ci.org/librespot-org/librespot.svg?branch=master)](https://travis-ci.org/librespot-org/librespot)

# librespot
*librespot* is an open source client library for Spotify. It enables
applications to use Spotify's service, without using the official but
closed-source libspotify. Additionally, it will provide extra features
which are not available in the official library.

Note: librespot only works with Spotify Premium

## This fork
As the origin by [plietar](https://github.com/plietar/) is no longer actively maintained I wanted to have a place for a version of librespot with other peoples forks and features merged.

# Wiki
More information can befound in the [wiki](https://github.com/librespot-org/librespot/wiki)

# Building
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
target/release/librespot --name DEVICENAME 
```

## Disclaimer
Using this code to connect to Spotify's API is probably forbidden by them.
Use at your own risk.

## Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/sashahilton00/spotify-connect-resources

## License
Everything in this repository is licensed under the MIT license.
