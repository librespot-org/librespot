# Contributing

## Setup

In order to contribute to librespot, you will first need to set up a suitable Rust build environment, with the necessary dependencies installed. These instructions will walk you through setting up a simple build environment.

You will need to have a C compiler, Rust, and the development libraries for the audio backend(s) you want installed.

### Install Rust

The easiest, and recommended way to get Rust is to use [rustup](https://rustup.rs). You can install `rustup` with this command:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Follow any prompts it gives you to install Rust. Once that’s done, Rust's standard tools should be setup and ready to use.

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

*_Note this is an non extensive list, open a PR to add to it!_*

| Audio backend               | Debian/Ubuntu                    | Fedora  | macOS |
|--------------------|------------------------------| ------------------------------| -- |
|Rodio (default)| `libasound2-dev` | `alsa-lib-devel` | 
|ALSA| `libasound2-dev, pkg-config` |`alsa-lib-devel` |
|PortAudio| `portaudio19-dev`| `portaudio-devel`| `portaudio`
|PulseAudio| `libpulse-dev`| `pulseaudio-libs-devel` | 
|JACK| `libjack-dev` | `jack-audio-connection-kit-devel` | 
|SDL| `libsdl2-dev`| `SDL2-devel` | 
|Pipe| - | - | - |

###### For example, to build an ALSA based backend, you would need to run the following:

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

To build a ```debug``` build, from the project root:

```bash
cargo build
```

And for ```release```:

```bash
cargo build --release
```

You will most likely want to build debug builds when developing, as they are faster, and more verbose, for the purposes of debugging.

There are also a number of compiler feature flags that you can add, in the event that you want to have certain additional features also compiled. The list of these is available on the [wiki](https://github.com/librespot-org/librespot/wiki/Compiling#addition-features).

By default, librespot compiles with the ```rodio-backend``` feature. To compile without default features, you can run with:

```bash
cargo build --no-default-features
```

### Running

Assuming you just compiled a ```debug``` build, you can run librespot with the following command:

```bash
./target/debug/librespot -n Librespot
```

There are various runtime options, documented in the wiki, and visible by running librespot with the ```-h``` argument.

## Reporting an Issue

Issues are tracked in the Github issue tracker of the librespot repo.

If you have encountered a bug, please report it, as we rely on user reports to fix them.

Please also make sure that your issues are helpful. To ensure that your issue is helpful, please read over this brief checklist to avoid the more common pitfalls:

	- Please take a moment to search/read previous similar issues to ensure you aren’t posting a duplicate. Duplicates will be closed immediately.
	- Please include a clear description of what the issue is. Issues with descriptions such as ‘It hangs after 40 minutes’ will be closed immediately.
	- Please include, where possible, steps to reproduce the bug, along with any other material that is related to the bug. For example, if librespot consistently crashes when you try to play a song, please include the Spotify URI of that song. This can be immensely helpful in quickly pinpointing and resolving issues.
	- Lastly, and perhaps most importantly, please include a backtrace where possible. Recent versions of librespot should produce these automatically when it crashes, and print them to the console, but in some cases, you may need to run ‘export RUST_BACKTRACE=full’ before running librespot to enable backtraces.

## Contributing Code

If there is an issue that you would like to write a fix for, or a feature you would like to implement, we use the following flow for updating code in the librespot repo:

```
Fork -> Fix -> PR -> Review -> Merge
```

This is how all code is added to the repository, even by those with write access.

#### Steps before Committing

In order to prepare for a PR, you will need to do a couple of things first:

Make any changes that you are going to make to the code, but do not commit yet.

Make sure that the code is correctly formatted by running:
```bash
cargo fmt --all
```

This command runs the previously installed ```rustfmt```, a code formatting tool that will automatically correct any formatting that you have used that does not conform with the librespot code style. Once that command has run, you will need to rebuild the project:

```bash
cargo build
```

Once it has built, and you have confirmed there are no warnings or errors, you should commit your changes.

```bash
git commit -a -m “My fancy fix”
```

**N.B.** Please, for the sake of a readable history, do not bundle multiple major changes into a single commit. Instead, break it up into multiple commits.

Once you have made the commits you wish to have merged, push them to your forked repo:

```bash
git push
```

Then open a pull request on the main librespot repo.

Once a pull request is under way, it will be reviewed by one of the project maintainers, and either approved for merging, or have changes requested. Please be alert in the review period for possible questions about implementation decisions, implemented behaviour, and requests for changes. Once the PR is approved, it will be merged into the main repo.

Happy Contributing :)
