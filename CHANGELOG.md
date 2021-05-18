# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) since v0.2.0.

## [Unreleased]

### Removed
* [audio] Removed `with-vorbis` and `with-tremor` features
* [audio] Removed `VorbisDecoder`, `VorbisError`, `AudioPacket`, `PassthroughDecoder`, `PassthroughError`, `AudioError`, `AudioDecoder` and the `convert` module from `librespot_audio`. The underlying crates `vorbis`, `librespot-tremor`, `lewton` and `ogg` should be used directly.

### Fixed

* [playback] Incorrect `PlayerConfig::default().normalisation_threshold` caused distortion when using dynamic volume normalisation downstream

## [0.2.0] - 2021-05-04

## [0.1.6] - 2021-02-22

## [0.1.5] - 2021-02-21

## [0.1.3] - 2020-07-29

## [0.1.2] - 2020-07-22

## [0.1.1] - 2020-01-30

## [0.1.0] - 2019-11-06

[unreleased]: https://github.com/librespot-org/librespot/compare/v0.2.0..HEAD
[0.2.0]: https://github.com/librespot-org/librespot/compare/v0.1.6..v0.2.0
[0.1.6]: https://github.com/librespot-org/librespot/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/librespot-org/librespot/compare/v0.1.3..v0.1.5
[0.1.3]: https://github.com/librespot-org/librespot/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/librespot-org/librespot/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/librespot-org/librespot/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/librespot-org/librespot/releases/tag/v0.1.0
