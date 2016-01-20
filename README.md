# librespot
*librespot* is an open source client library for Spotify. It enables
applications to use Spotify's service, without using the official but
closed-source libspotify. Additionally, it will provide extra features
which are not available in the official library.

## Status
*librespot* is currently under development and is not fully functional yet. You
are however welcome to experiment with it.

## Building
Building *librespot* requires rust nightly. It will not work on rust stable or
beta.

It also requires a C and C++ toolchain, with libprotoc and portaudio.

On debian / ubuntu, the following command will install these dependencies :
```shell
sudo apt-get install build-essential portaudio19-dev libprotoc-dev
```

On OS X, using homebrew :
```shell
brew install portaudio protobuf
```

Once you've cloned this repository you can build *librespot* using `cargo`.
```shell
cargo build
```

## Usage
A sample program implementing a headless Spotify Connect receiver is provided.
Once you've built *librespot*, run it using :
```shell
target/debug/librespot --appkey APPKEY --username USERNAME --cache CACHEDIR --name DEVICENAME
```

## Discovery mode
*librespot* can be run in discovery mode, in which case no password is required at startup.
dns-sd or avahi's compatibility layer is required for this. On debian/ubuntu this is the
`libavahi-compat-libdnssd-dev` package. It come preinstalled on OS X.

It must be enabled at build time :
```shell
cargo build --features discovery
```

When running *librespot* simply omit the `--username` argument.

## Facebook Accounts
If you connect using a facebook account, librespot will not show up among the
devices in the Spotify app. What you need to do is apply for a
[device password](http://www.spotify.com/account/set-device-password/) and
use that to sign in instead.

## Disclaimer
Using this code to connect to Spotify's API is probably forbidden by them, and
might result in you application key getting banned. Use at you own risk

## Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/sashahilton00/spotify-connect-resources

## License
Everything in this repository is licensed under the MIT license.

