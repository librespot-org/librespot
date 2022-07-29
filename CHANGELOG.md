# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) since v0.2.0.

## [0.4.2] - 2022-07-29

### Changed
- [playback] `pipe`: Better error handling
- [playback] `subprocess`: Better error handling

### Added
- [core] `apresolve`: Blacklist ap-gew4 and ap-gue1 access points that cause channel errors
- [playback] `pipe`: Implement stop

### Fixed
- [main] fix `--opt=value` line argument logging
- [playback] `alsamixer`: make `--volume-ctrl fixed` work as expected when combined with `--mixer alsa`

## Removed

## [0.4.1] - 2022-05-23

### Changed
- [chore] The MSRV is now 1.56

### Fixed
- [playback] Fixed dependency issues when installing from crate

## [0.4.0] - 2022-05-21

### Changed
- [chore] The MSRV is now 1.53
- [contrib] Hardened security of the `systemd` service units
- [core] `Session`: `connect()` now returns the long-term credentials
- [core] `Session`: `connect()` now accepts a flag if the credentails should be stored via the cache
- [main] Different option descriptions and error messages based on what backends are enabled at build time
- [playback] More robust dynamic limiter for very wide dynamic range (breaking)
- [playback] `alsa`: improve `--device ?` output for the Alsa backend
- [playback] `gstreamer`: create own context, set correct states and use sync handler
- [playback] `pipe`: create file if it doesn't already exist
- [playback] `Sink`: `write()` now receives ownership of the packet (breaking)

### Added
- [main] Enforce reasonable ranges for option values (breaking)
- [main] Add the ability to parse environment variables
- [main] Log now emits warning when trying to use options that would otherwise have no effect
- [main] Verbose logging now logs all parsed environment variables and command line arguments (credentials are redacted)
- [main] Add a `-q`, `--quiet` option that changes the logging level to WARN
- [main] Add `disable-credential-cache` flag (breaking)
- [main] Add a short name for every flag and option
- [playback] `pulseaudio`: set the PulseAudio name to match librespot's device name via `PULSE_PROP_application.name` environment variable (user set env var value takes precedence) (breaking)
- [playback] `pulseaudio`: set icon to `audio-x-generic` so we get an icon instead of a placeholder via `PULSE_PROP_application.icon_name` environment variable (user set env var value takes precedence) (breaking)
- [playback] `pulseaudio`: set values to: `PULSE_PROP_application.version`, `PULSE_PROP_application.process.binary`, `PULSE_PROP_stream.description`, `PULSE_PROP_media.software` and `PULSE_PROP_media.role` environment variables (user set env var values take precedence) (breaking)

### Fixed
- [connect] Don't panic when activating shuffle without previous interaction
- [core] Removed unsafe code (breaking)
- [main] Fix crash when built with Avahi support but Avahi is locally unavailable
- [main] Prevent hang when discovery is disabled and there are no credentials or when bad credentials are given
- [main] Don't panic when parsing options, instead list valid values and exit
- [main] `--alsa-mixer-device` and `--alsa-mixer-index` now fallback to the card and index specified in `--device`.
- [playback] Adhere to ReplayGain spec when calculating gain normalisation factor
- [playback] `alsa`: make `--volume-range` overrides apply to Alsa softvol controls

### Removed
- [playback] `alsamixer`: previously deprecated options `mixer-card`, `mixer-name` and `mixer-index` have been removed

## [0.3.1] - 2021-10-24

### Changed
- Include build profile in the displayed version information
- [playback] Improve dithering CPU usage by about 33%

### Fixed
- [connect] Partly fix behavior after last track of an album/playlist

## [0.3.0] - 2021-10-13

### Added
- [discovery] The crate `librespot-discovery` for discovery in LAN was created. Its functionality was previously part of `librespot-connect`.
- [playback] Add support for dithering with `--dither` for lower requantization error (breaking)
- [playback] Add `--volume-range` option to set dB range and control `log` and `cubic` volume control curves
- [playback] `alsamixer`: support for querying dB range from Alsa softvol
- [playback] Add `--format F64` (supported by Alsa and GStreamer only)
- [playback] Add `--normalisation-gain-type auto` that switches between album and track automatically

### Changed
- [audio, playback] Moved `VorbisDecoder`, `VorbisError`, `AudioPacket`, `PassthroughDecoder`, `PassthroughError`, `DecoderError`, `AudioDecoder` and the `convert` module from `librespot-audio` to `librespot-playback`. The underlying crates `vorbis`, `librespot-tremor`, `lewton` and `ogg` should be used directly. (breaking)
- [audio, playback] Use `Duration` for time constants and functions (breaking)
- [connect, playback] Moved volume controls from `librespot-connect` to `librespot-playback` crate
- [connect] Synchronize player volume with mixer volume on playback
- [playback] Store and pass samples in 64-bit floating point
- [playback] Make cubic volume control available to all mixers with `--volume-ctrl cubic`
- [playback] Normalize volumes to `[0.0..1.0]` instead of `[0..65535]` for greater precision and performance (breaking)
- [playback] `alsamixer`: complete rewrite (breaking)
- [playback] `alsamixer`: query card dB range for the volume control unless specified otherwise
- [playback] `alsamixer`: use `--device` name for `--mixer-card` unless specified otherwise
- [playback] `player`: consider errors in `sink.start`, `sink.stop` and `sink.write` fatal and `exit(1)` (breaking)
- [playback] `player`: make `convert` and `decoder` public so you can implement your own `Sink`
- [playback] `player`: update default normalisation threshold to -2 dBFS
- [playback] `player`: default normalisation type is now `auto`

### Deprecated
- [connect] The `discovery` module was deprecated in favor of the `librespot-discovery` crate
- [playback] `alsamixer`: renamed `mixer-card` to `alsa-mixer-device`
- [playback] `alsamixer`: renamed `mixer-name` to `alsa-mixer-control`
- [playback] `alsamixer`: renamed `mixer-index` to `alsa-mixer-index`

### Removed
- [connect] Removed no-op mixer started/stopped logic (breaking)
- [playback] Removed `with-vorbis` and `with-tremor` features
- [playback] `alsamixer`: removed `--mixer-linear-volume` option, now that `--volume-ctrl {linear|log}` work as expected on Alsa

### Fixed
- [connect] Fix step size on volume up/down events
- [connect] Fix looping back to the first track after the last track of an album or playlist
- [playback] Incorrect `PlayerConfig::default().normalisation_threshold` caused distortion when using dynamic volume normalisation downstream
- [playback] Fix `log` and `cubic` volume controls to be mute at zero volume
- [playback] Fix `S24_3` format on big-endian systems
- [playback] `alsamixer`: make `cubic` consistent between cards that report minimum volume as mute, and cards that report some dB value
- [playback] `alsamixer`: make `--volume-ctrl {linear|log}` work as expected
- [playback] `alsa`, `gstreamer`, `pulseaudio`: always output in native endianness
- [playback] `alsa`: revert buffer size to ~500 ms
- [playback] `alsa`, `pipe`, `pulseaudio`: better error handling
- [metadata] Skip tracks whose Spotify ID's can't be found (e.g. local files, which aren't supported)

## [0.2.0] - 2021-05-04

## [0.1.6] - 2021-02-22

## [0.1.5] - 2021-02-21

## [0.1.3] - 2020-07-29

## [0.1.2] - 2020-07-22

## [0.1.1] - 2020-01-30

## [0.1.0] - 2019-11-06

[0.4.2]: https://github.com/librespot-org/librespot/compare/v0.4.1..v0.4.2
[0.4.1]: https://github.com/librespot-org/librespot/compare/v0.4.0..v0.4.1
[0.4.0]: https://github.com/librespot-org/librespot/compare/v0.3.1..v0.4.0
[0.3.1]: https://github.com/librespot-org/librespot/compare/v0.3.0..v0.3.1
[0.3.0]: https://github.com/librespot-org/librespot/compare/v0.2.0..v0.3.0
[0.2.0]: https://github.com/librespot-org/librespot/compare/v0.1.6..v0.2.0
[0.1.6]: https://github.com/librespot-org/librespot/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/librespot-org/librespot/compare/v0.1.3..v0.1.5
[0.1.3]: https://github.com/librespot-org/librespot/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/librespot-org/librespot/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/librespot-org/librespot/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/librespot-org/librespot/releases/tag/v0.1.0
