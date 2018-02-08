[![Build Status](https://travis-ci.org/librespot-org/librespot.svg?branch=master)](https://travis-ci.org/librespot-org/librespot)
[![Gitter chat](https://badges.gitter.im/librespot-org/librespot.png)](https://gitter.im/sashahilton00/spotify-connect-resources)

# librespot
*librespot* is an open source client library for Spotify. It enables
applications to use Spotify's service, without using the official but
closed-source libspotify. Additionally, it will provide extra features
which are not available in the official library.

Note: librespot only works with Spotify Premium

## This fork
As the origin by [plietar](https://github.com/plietar/) is no longer actively maintained, this organisation and repository have been set up so that the project may be maintained and upgraded in the future.

# Wiki
More information can be found in the [wiki](https://github.com/librespot-org/librespot/wiki)

# Building
Rust 1.20.0 or later is required to build librespot.

**If you are building librespot on macOS, the homebrew provided rust may fail due to the way in which homebrew installs rust. In this case, uninstall the homebrew version of rust and use [rustup](https://www.rustup.rs/), and librespot should then build. This should have been fixed in more recent versions of Homebrew, but we're leaving this notice here as a warning.**

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

## Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/sashahilton00/spotify-connect-resources

## To-Do/Feature Requests
If there is a feature request that is being considered, or has been widely requested, it should be listed below. Please do not use this for bug reports or special use case feature requests.

- [ ] Add support for contexts (used by dynamic playlists, Spotify Radio, green now-playing bar, etc.) ([#57](https://github.com/librespot-org/librespot/issues/57))
- [ ] Document the Spotify Protocol and provide reference example.
- [ ] Implement API to allow wrappers to be written for librespot.
- [x] Logarithmic volume scaling ([#10](https://github.com/librespot-org/librespot/issues/10))
- [ ] Fix Shuffle & Repeat functionality
- [ ] Provide automatic release binaries for download
- [ ] Provide an adequate method for exporting metadata ([#7](https://github.com/librespot-org/librespot/issues/7))
  - [ ] Provide API Documentation
  - [ ] Provide Schema/Versioning

## Disclaimer
Using this code to connect to Spotify's API is probably forbidden by them.
Use at your own risk.

## License
Everything in this repository is licensed under the MIT license.
