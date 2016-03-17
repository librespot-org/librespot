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

It also requires a C and C++ toolchain, with libprotoc and portaudio.

On debian / ubuntu, the following command will install these dependencies :
```shell
sudo apt-get install build-essential portaudio19-dev libprotoc-dev
```

On Fedora systems, the following command will install these dependencies :
```shell
sudo dnf install portaudio-devel libprotoc-dev make gcc gcc-c++
```

On OS X, using homebrew :
```shell
brew install portaudio protobuf
```

Once you've cloned this repository you can build *librespot* using `cargo`.
```shell
cargo build --release
```

## Usage
A sample program implementing a headless Spotify Connect receiver is provided.
Once you've built *librespot*, run it using :
```shell
target/release/librespot --appkey APPKEY --username USERNAME --cache CACHEDIR --name DEVICENAME
```

## Discovery mode
*librespot* can be run in discovery mode, in which case no password is required at startup.
dns-sd or avahi's compatibility layer is required for this. On debian/ubuntu this is the
`libavahi-compat-libdnssd-dev` package. On Fedora, this is the
`avahi-compat-libdns_sd-devel` package. It come preinstalled on OS X.

It must be enabled at build time :
```shell
cargo build --release --features discovery
```

When running *librespot* simply omit the `--username` argument.

## Facebook Accounts
*librespot* can be built with Facebook authentication support. OpenSSL is required for this.

```shell
cargo build --release --features facebook
target/release/librespot --appkey APPKEY --cache CACHEDIR --name DEVICENAME --facebook
```

This will print a link to the console, which must be visited on the same computer *librespot* is running on.

## Development
When developing *librespot*, it is preferable to use Rust nightly, and build it using the following :
```shell
cargo build --no-default-features
```

This produces better compilation error messages than with the default configuration.

## Disclaimer
Using this code to connect to Spotify's API is probably forbidden by them, and
might result in you application key getting banned. Use at you own risk

## Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/sashahilton00/spotify-connect-resources

## License
Everything in this repository is licensed under the MIT license.

