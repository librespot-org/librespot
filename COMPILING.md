# Compiling

## Setup

In order to compile librespot, you will first need to set up a suitable Rust build environment, with the necessary dependencies installed. You will need to have a C compiler, Rust, and the development libraries for the audio backend(s) you want installed. These instructions will walk you through setting up a simple build environment.

### Install Rust
The easiest, and recommended way to get Rust is to use [rustup](https://rustup.rs). On Unix/MacOS You can install `rustup` with this command:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Follow any prompts it gives you to install Rust. Once that’s done, Rust's standard tools should be setup and ready to use.

*Note: The current minimum required Rust version at the time of writing is 1.40.0, you can find the current minimum version specified in the `.github/workflow/test.yml` file.*

#### Additional Rust tools - `rustfmt`
To ensure a consistent codebase, we utilise [`rustfmt`](https://github.com/rust-lang/rustfmt), which is installed by default with `rustup` these days, else it can be installed manually with:
```bash
rustup component add rustfmt
```
Using `rustfmt` is not optional, as our CI checks against this repo's rules.

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
|PortAudio           | `portaudio19-dev`            | `portaudio-devel`                 | `portaudio` |
|PulseAudio          | `libpulse-dev`               | `pulseaudio-libs-devel`           |             |
|JACK                | `libjack-dev`                | `jack-audio-connection-kit-devel` |             |
|SDL                 | `libsdl2-dev`                | `SDL2-devel`                      |             |
|Pipe                |  -                           |  -                                |  -          |

###### For example, to build an ALSA based backend, you would need to run the following to install the required dependencies:

On Debian/Ubuntu:
```shell
sudo apt-get install libasound2-dev pkg-config

```
On Fedora systems:
```shell
sudo dnf install alsa-lib-devel
```

### Getting the Source

The recommended method is to first fork the repo, so that you have a copy that you have read/write access to. After that, it’s a simple case of cloning your fork.

```bash
git clone git@github.com:YOURUSERNAME/librespot.git
cd librespot
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

By default, librespot compiles with the ```rodio-backend``` feature. To compile without default features, you can run with:

```bash
cargo build --no-default-features
```

Similarly, to build with the ALSA backend:
```bash
cargo build --no-default-features --features "alsa-backend"
```

### Running

Assuming you just compiled a ```debug``` build, you can run librespot with the following command:

```bash
./target/debug/librespot -n Librespot
```

There are various runtime options, documented in the wiki, and visible by running librespot with the ```-h``` argument.
