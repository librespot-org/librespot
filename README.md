# librespot
*librespot* is an open source client library for Spotify. It enables
applications to use Spotify's service, without using the official but
closed-source libspotify. Additionally, it provides extra features which are
not available in the official library.

## Status
*librespot* is currently under development and is not fully functional yet. You
are however welcome to experiment with it.

## Building
Building *librespot* requires rust nightly. It will not work on rust stable or
beta.

You will also need the protobuf compiler, and the
[rust-protobuf](https://github.com/stepancheg/rust-protobuf) plugin.
`protoc-gen-rust` must be in your `$PATH`.

Once you've cloned this repository you can build *librespot* using `cargo`.
```shell
cargo build
```

## Usage
A sample program implementing a headless Spotify Connect receiver is provided.
Once you've built *librespot*, run it using :
```shell
target/release/main APPKEY USERNAME PASSWORD CACHEDIR DEVICENAME
```
where `APPKEY` is the path to a Spotify application key file, `USERNAME` and
`PASSWORD` are your Spotify credentials, `CACHEDIR` is the path to directory
where data will be cached, and `DEVICENAME` is the name that will appear in the
Spotify Connect menu.

## Disclaimer
Using this code to connect to Spotify's API is probably forbidden by them, and
might result in you application key getting banned. Use at you own risk

## Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/sashahilton00/spotify-connect-resources

## License
Everything in this repository is licensed under the MIT license.

