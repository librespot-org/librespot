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

### TLS library dependencies
librespot requires a TLS implementation for secure connections to Spotify's servers. You can choose between two mutually exclusive options:

#### native-tls (default)
Uses your system's native TLS implementation:
- **Linux**: OpenSSL
- **macOS**: Secure Transport (Security.framework)
- **Windows**: SChannel (Windows TLS)

This is the **default choice** and provides the best compatibility. It integrates with your system's certificate store and is well-tested across platforms.

**When to choose native-tls:**
- You want maximum compatibility
- You're using system-managed certificates
- You're on a standard Linux distribution with OpenSSL
- You're deploying on servers where OpenSSL is already present

**Dependencies:**
On Debian/Ubuntu:
```shell
sudo apt-get install libssl-dev pkg-config
```

On Fedora:
```shell
sudo dnf install openssl-devel pkg-config
```

#### rustls-tls
Uses a Rust-based TLS implementation with `rustls-platform-verifier` for certificate authority (CA) verification:
- **Linux**: Uses system ca-certificates package
- **macOS**: Uses Security.framework for CA verification
- **Windows**: Uses Windows certificate store

**When to choose rustls-tls:**
- You want to avoid external OpenSSL dependencies
- You're building for reproducible/deterministic builds
- You're targeting platforms where OpenSSL is unavailable or problematic (musl, embedded, static linking)
- You're cross-compiling and want to avoid OpenSSL build complexity
- You prefer having cryptographic operations implemented in Rust

**No additional system dependencies required** - rustls is implemented in Rust (with some assembly for performance-critical cryptographic operations) and doesn't require external libraries like OpenSSL.

#### Building with specific TLS backends
```bash
# Default (native-tls)
cargo build

# Explicitly use native-tls
cargo build --no-default-features --features "native-tls rodio-backend with-libmdns"

# Use rustls-tls instead
cargo build --no-default-features --features "rustls-tls rodio-backend with-libmdns"
```

**Important:** The TLS backends are mutually exclusive. Attempting to enable both will result in a compile-time error.

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

There are also a number of compiler feature flags that you can add, in the event that you want to have certain additional features also compiled. All available features and their descriptions are documented in the main [Cargo.toml](Cargo.toml) file. Additional platform-specific information is available on the [wiki](https://github.com/librespot-org/librespot/wiki/Compiling#addition-features).

By default, librespot compiles with the ```native-tls```, ```rodio-backend```, and ```with-libmdns``` features. 

**Note:** librespot requires at least one TLS backend to function. Building with `--no-default-features` alone will fail compilation. For custom feature selection, you must specify at least one TLS backend along with your desired audio and discovery backends.
For example, to build with the ALSA audio, libmdns discovery, and native-tls backends:

```bash
cargo build --no-default-features --features "native-tls alsa-backend with-libmdns"
```

Or to use rustls-tls with ALSA:

```bash
cargo build --no-default-features --features "rustls-tls alsa-backend with-libmdns"
```

### Running

Assuming you just compiled a ```debug``` build, you can run librespot with the following command:

```bash
./target/debug/librespot
```

There are various runtime options, documented in the wiki, and visible by running librespot with the ```-h``` argument.

Note that debug builds may cause buffer underruns and choppy audio when dithering is enabled (which it is by default). You can disable dithering with ```--dither none```.
