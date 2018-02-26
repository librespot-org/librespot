# Contributing

## Setup

In order to contribute to librespot, you will first need to set up a suitable rust build environment, with the necessary dependenices installed. These instructions will walk you through setting up a simple build environment.

You will need to have C compiler, rust, and portaudio installed.

### Install Rust

The easiest, and recommended way to get rust setu is to use [rustup](https://rustup.rs). You can install rustup with this command:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Follow any prompts it gives you to install rust. Once that’s done, rust is ready to use.
 
### Install Other Dependencies
On debian / ubuntu, the following command will install these dependencies :

```bash
sudo apt-get install build-essential portaudio19-dev
```

On Fedora systems, the following command will install these dependencies :

```bash
sudo dnf install portaudio-devel make gcc
```

On macOS, using homebrew :

```bash
brew install portaudio
```

### Getting the Source

The recommended method is to first fork the repo, so that you have a copy that you have read/write access to. After that, it’s a simple case of git cloning.

```bash
git clone git@github.com:YOURUSERNAME/librespot.git
```

CD to the newly cloned repo...

```bash
cd librespot
```

### Development Extra Steps

If you are looking to carry out development on librespot:

```bash
rustup override set nightly
```

The command above overrides the default rust in the directory housing librespot to use the ```nightly``` version, as opposed to the ```stable``` version.

Then, run the command below to install [rustfmt](https://github.com/rust-lang-nursery/rustfmt) for the ```nightly``` toolchain. This is not optional, as Travis CI is set up to check that code is compliant with rustfmt.

```bash
rustup component add rustfmt-preview
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

By default, librespot compiles with the ```portaudio-backend``` feature. To compile without default features, you can run with:

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

#### Steps before Commiting

In order to prepare for a PR, you will need to do a couple of things first:

Make any changes that you are going to make to the code, but do not commit yet.

Make sure you are using rust ```nightly``` to build librespot. Once this is confirmed, you will need to run the following command:

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

**N.B.** Please, for the sake of a readable history, do not bundle multipe major changes into a single commit. Instead, break it up into multiple commits.

Once you have made the commits you wish to have merged, push them to your forked repo:

```bash
git push
```

Then open a pull request on the main librespot repo.

Once a pull request is under way, it will be reviewed by one of the project maintainers, and either approved for merging, or have changes requested. Please be alert in the review period for possible questions about implementation decisions, implemented behaviour, and requests for changes. Once the PR is approved, it will be merged into the main repo.

Happy Contributing :)
