# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) since v0.2.0.

## [Unreleased]
### Added
- [discovery] The crate `librespot-discovery` for discovery in LAN was created. Its functionality was previously part of `librespot-connect`.
- [playback] Add support for dithering with `--dither` for lower requantization error (breaking)
- [playback] Add `--volume-range` option to set dB range and control `log` and `cubic` volume control curves
- [playback] `alsamixer`: support for querying dB range from Alsa softvol
- [playback] Add `--format F64` (supported by Alsa and GStreamer only)

### Changed
- [audio, playback] Moved `VorbisDecoder`, `VorbisError`, `AudioPacket`, `PassthroughDecoder`, `PassthroughError`, `AudioError`, `AudioDecoder` and the `convert` module from `librespot-audio` to `librespot-playback`. The underlying crates `vorbis`, `librespot-tremor`, `lewton` and `ogg` should be used directly. (breaking)
- [audio, playback] Use `Duration` for time constants and functions (breaking)
- [connect, playback] Moved volume controls from `librespot-connect` to `librespot-playback` crate
- [connect] Synchronize player volume with mixer volume on playback
- [playback] Store and pass samples in 64-bit floating point
- [playback] Make cubic volume control available to all mixers with `--volume-ctrl cubic`
- [playback] Normalize volumes to `[0.0..1.0]` instead of `[0..65535]` for greater precision and performance (breaking)
- [playback] `alsamixer`: complete rewrite (breaking)
- [playback] `alsamixer`: query card dB range for the `log` volume control unless specified otherwise
- [playback] `alsamixer`: use `--device` name for `--mixer-card` unless specified otherwise
- [playback] `player`: consider errors in `sink.start`, `sink.stop` and `sink.write` fatal and `exit(1)` (breaking)
- [playback] `player`: make `convert` and `decoder` public so you can implement your own `Sink`

### Deprecated
- [connect] The `discovery` module was deprecated in favor of the `librespot-discovery` crate

### Removed
- [connect] Removed no-op mixer started/stopped logic (breaking)
- [playback] Removed `with-vorbis` and `with-tremor` features
- [playback] `alsamixer`: removed `--mixer-linear-volume` option; use `--volume-ctrl linear` instead

### Fixed
- [connect] Fix step size on volume up/down events
- [playback] Incorrect `PlayerConfig::default().normalisation_threshold` caused distortion when using dynamic volume normalisation downstream 
- [playback] Fix `log` and `cubic` volume controls to be mute at zero volume
- [playback] Fix `S24_3` format on big-endian systems
- [playback] `alsamixer`: make `cubic` consistent between cards that report minimum volume as mute, and cards that report some dB value
- [playback] `alsamixer`: make `--volume-ctrl {linear|log}` work as expected
- [playback] `alsa`, `gstreamer`, `pulseaudio`: always output in native endianness
- [playback] `alsa`: revert buffer size to ~500 ms
- [playback] `alsa`, `pipe`, `pulseaudio`: better error handling

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
