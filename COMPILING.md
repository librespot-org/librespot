# Compiling

## Setup

In order to compile librespot, you will first need to set up a suitable Rust build environment, with the necessary dependencies installed. You will need to have a C compiler, Rust, and the development libraries for the audio backend(s) you want installed. These instructions will walk you through setting up a simple build environment.

### Install Rust
The easiest, and recommended way to get Rust is to use [rustup](https://rustup.rs). Once that’s installed, Rust's standard tools should be set up and ready to use.

#### Additional Rust tools - `rustfmt`
To ensure a consistent codebase, we utilise [`rustfmt`](https://github.com/rust-lang/rustfmt) and [`clippy`](https://github.com/rust-lang/rust-clippy), which are installed by default with `rustup` these days, else they can be installed manually with:
```bash
rustup component add rustfmt
rustup component add clippy
```
Using `cargo fmt` and `cargo clippy` is not optional, as our CI checks against this repo's rules.

### General dependencies
Along with Rust, you will also require a C compiler.

On Debian/Ubuntu, install with:
```shell
sudo apt-get install build-essential

```
On Fedora systems, install with:
```shell
sudo dnf install gcc
```
### Audio library dependencies
Depending on the chosen backend, specific development libraries are required.

*_Note this is an non-exhaustive list, open a PR to add to it!_*

| Audio backend      | Debian/Ubuntu                | Fedora                            | macOS       |
|--------------------|------------------------------|-----------------------------------|-------------|
|Rodio (default)     | `libasound2-dev`             | `alsa-lib-devel`                  |             |
|ALSA                | `libasound2-dev, pkg-config` | `alsa-lib-devel`                  |             |
|GStreamer | `gstreamer1.0-plugins-base libgstreamer-plugins-base1.0-dev gstreamer1.0-plugins-good libgstreamer-plugins-good1.0-dev` | `gstreamer1 gstreamer1-devel gstreamer1-plugins-base-devel gstreamer1-plugins-good` | `gstreamer gst-devtools gst-plugins-base gst-plugins-good` |
|PortAudio           | `portaudio19-dev`            | `portaudio-devel`                 | `portaudio` |
|PulseAudio          | `libpulse-dev`               | `pulseaudio-libs-devel`           |             |
|JACK                | `libjack-dev`                | `jack-audio-connection-kit-devel` |  `jack`     |
|JACK over Rodio     | `libjack-dev`                | `jack-audio-connection-kit-devel` |  `jack`     |
|SDL                 | `libsdl2-dev`                | `SDL2-devel`                      |  `sdl2`     |
|Pipe & subprocess   |  -                           |  -                                |  -          |

###### For example, to build an ALSA based backend, you would need to run the following to install the required dependencies:

On Debian/Ubuntu:
```shell
sudo apt-get install libasound2-dev pkg-config

```
On Fedora systems:
```shell
sudo dnf install alsa-lib-devel
```

### Zeroconf library dependencies
Depending on the chosen backend, specific development libraries are required.

*_Note this is an non-exhaustive list, open a PR to add to it!_*

| Zeroconf backend   | Debian/Ubuntu                | Fedora                            | macOS       |
|--------------------|------------------------------|-----------------------------------|-------------|
|avahi               |                              |                                   |             |
|dns_sd              | `libavahi-compat-libdnssd-dev pkg-config` | `avahi-compat-libdns_sd-devel` |   |
|libmdns (default)   |                              |                                   |             |

### Getting the Source

The recommended method is to first fork the repo, so that you have a copy that you have read/write access to. After that, it’s a simple case of cloning your fork.

```bash
git clone git@github.com:YOUR_USERNAME/librespot.git
```

## Compiling & Running

Once your build environment is setup, compiling the code is pretty simple.

### Compiling

To build a ```debug``` build with the default backend, from the project root run:

```bash
cargo build
```

And for ```release```:

```bash
cargo build --release
```

You will most likely want to build debug builds when developing, as they compile faster, and more verbose, and as the name suggests, are for the purposes of debugging. When submitting a bug report, it is recommended to use a debug build to capture stack traces.

There are also a number of compiler feature flags that you can add, in the event that you want to have certain additional features also compiled. The list of these is available on the [wiki](https://github.com/librespot-org/librespot/wiki/Compiling#addition-features).

By default, librespot compiles with the ```rodio-backend``` and ```with-libmdns``` features. To compile without default features, you can run with:

```bash
cargo build --no-default-features
```

Note that this will also disable zeroconf discovery backends for Spotify Connect. For normal use cases, select at least one audio and discovery backend.
For example, to build with the ALSA audio and libmdns discovery backend:

```bash
cargo build --no-default-features --features "alsa-backend with-libmdns"
```

#### Cross Compiling for Raspberry Pi

To cross compile for Raspberry pi, get the arm-unknown-gnueabihf target for Rust:

```bash
rustup target add armv7-unknown-linux-gnueabi
```

You need to manually add a linker, which can be installed with apt:

```bash
sudo apt install gcc-arm-linux-gnueabihf
```

The target must be specified to cargo along with the linker.
This can be done via `.cargo/config.toml`, or via command line:

```bash
RUSTFLAGS="-C linker=arm-linux-gnueabihf-gcc" cargo build --no-default-features --target armv7-unknown-linux-gnueabihf
```

Note that this does not work with armv6 (found e.g. in Raspberry Pi 1 and Zero). Previously, you could use the linker in https://github.com/raspberrypi/tools, but this is now too old and will give an error when compiling Librespot (since `3e85d77bfb3520e16c35bb7db3912c76e8cc90d7`). More info: https://github.com/japaric/rust-cross/issues/42

### Running

Assuming you just compiled a ```debug``` build, you can run librespot with the following command:

```bash
./target/debug/librespot
```

There are various runtime options, documented in the wiki, and visible by running librespot with the ```-h``` argument.

Note that debug builds may cause buffer underruns and choppy audio when dithering is enabled (which it is by default). You can disable dithering with ```--dither none```.
