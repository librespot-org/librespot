# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) since v0.2.0.

## [Unreleased]

### Added

- [core] Add `SpotifyUri` type to represent more types of URI than `SpotifyId` can
- [main] `--local-file-dir` / `-l` option added to binary to specify local file directories to pull from
- [metadata] `Local` variant added to `UniqueFields` enum (breaking)ss
- [playback] Local files can now be played with the following caveats:
  - They must be sampled at 44,100 Hz
  - They cannot be played from a Connect device using the dedicated 'Local Files' playlist; they must be added to another playlist first
- [playback] `local_file_directories` field added to `PlayerConfig` struct (breaking)
  
### Changed

- [playback] Changed type of `SpotifyId` fields in `PlayerEvent` members to `SpotifyUri` (breaking)
- [metadata] Changed arguments for `Metadata` trait from `&SpotifyId` to `&SpotifyUri` (breaking)
- [player] `load` function changed from accepting a `SpotifyId` to accepting a `SpotifyUri` (breaking)
- [player] `preload` function changed from accepting a `SpotifyId` to accepting a `SpotifyUri` (breaking)
- [spclient] `get_radio_for_track` function changed from accepting a `SpotifyId` to accepting a `SpotifyUri` (breaking)

### Removed

- [core] Removed `SpotifyItemType` enum; the new `SpotifyUri` is an enum over all item types and so which variant it is 
  describes its item type (breaking)
- [core] Removed `NamedSpotifyId` struct; it was made obsolete by `SpotifyUri` (breaking)
- [core] The following methods have been removed from `SpotifyId` and moved to `SpotifyUri` (breaking):
  - `is_playable`
  - `from_uri`
  - `to_uri`

## [v0.7.1] - 2025-08-31

### Changed

- [connect] Shuffling was adjusted, so that shuffle and repeat can be used combined

### Fixed

- [connect] Repeat context will not go into autoplay anymore and triggering autoplay while shuffling shouldn't reshuffle anymore
- [connect] Only deletes the connect state on dealer shutdown instead on disconnecting
- [core] Fixed a problem where in `spclient` where an HTTP/411 error was thrown because the header was set wrong
- [main] Use the config instead of the type default for values that are not provided by the user

## [0.7.0] - 2025-08-24

### Changed

- [core] MSRV is now 1.85 with Rust edition 2024 (breaking)
- [core] AP connect and handshake have a combined 5 second timeout.
- [core] `stream_from_cdn` now accepts the URL as `TryInto<Uri>` instead of `CdnUrl` (breaking)
- [core] Add TLS backend selection with native-tls and rustls-tls options, defaulting to native-tls
- [connect] Replaced `has_volume_ctrl` with `disable_volume` in `ConnectConfig` (breaking)
- [connect] Changed `initial_volume` from `Option<u16>` to `u16` in `ConnectConfig` (breaking)
- [connect] Replaced `SpircLoadCommand` with `LoadRequest`, `LoadRequestOptions` and `LoadContextOptions` (breaking)
- [connect] Moved all public items to the highest level (breaking)
- [connect] Replaced Mercury usage in `Spirc` with Dealer
- [metadata] Replaced `AudioFileFormat` with own enum. (breaking)
- [playback] Changed trait `Mixer::open` to return `Result<Self, Error>` instead of `Self` (breaking)
- [playback] Changed type alias `MixerFn` to return `Result<Arc<dyn Mixer>, Error>` instead of `Arc<dyn Mixer>` (breaking)
- [playback] Optimize audio conversion to always dither at 16-bit level, and improve performance
- [playback] Normalizer maintains better stereo imaging, while also being faster
- [oauth] Remove loopback address requirement from `redirect_uri` when spawning callback handling server versus using stdin.

### Added

- [connect] Add command line parameter for setting volume steps.
- [connect] Add support for `seek_to`, `repeat_track` and `autoplay` for `Spirc` loading
- [connect] Add `pause` parameter to `Spirc::disconnect` method (breaking)
- [connect] Add `volume_steps` to `ConnectConfig` (breaking)
- [connect] Add and enforce rustdoc
- [playback] Add `track` field to `PlayerEvent::RepeatChanged` (breaking)
- [playback] Add `PlayerEvent::PositionChanged` event to notify about the current playback position
- [core] Add `request_with_options` and `request_with_protobuf_and_options` to `SpClient`
- [core] Add `try_get_urls` to `CdnUrl`
- [oauth] Add `OAuthClient` and `OAuthClientBuilder` structs to achieve a more customizable login process

### Fixed

- [test] Missing bindgen breaks crossbuild on recent runners. Now installing latest bindgen in addition.
- [core] Fix "no native root CA certificates found" on platforms unsupported
  by `rustls-native-certs`.
- [core] Fix all APs rejecting with "TryAnotherAP" when connecting session
  on Android platform.
- [core] Fix "Invalid Credentials" when using a Keymaster access token and
  client ID on Android platform.
- [connect] Fix "play" command not handled if missing "offset" property
- [discovery] Fix libmdns zerconf setup errors not propagating to the main task.
- [metadata] `Show::trailer_uri` is now optional since it isn't always present (breaking)
- [metadata] Fix incorrect parsing of audio format
- [connect] Handle transfer of playback with empty "uri" field
- [connect] Correctly apply playing/paused state when transferring playback
- [player] Saturate invalid seek positions to track duration
- [audio] Fall back to other URLs in case of a failure when downloading from CDN
- [core] Metadata requests failing with 500 Internal Server Error
- [player] Rodio backend did not honor audio output format request

### Deprecated

- [oauth] `get_access_token()` function marked for deprecation
- [core] `try_get_url()` function marked for deprecation

### Removed

- [core] Removed `get_canvases` from SpClient (breaking)
- [core] DeviceType `homething` removed due to crashes on Android (breaking)
- [metadata] Removed `genres` from Album (breaking)
- [metadata] Removed `genre` from Artists (breaking)

## [0.6.0] - 2024-10-30

This version takes another step into the direction of the HTTP API, fixes a
couple of bugs, and makes it easier for developers to mock a certain platform.
Also it adds the option to choose avahi, dnssd or libmdns as your zeroconf
backend for Spotify Connect discovery.

### Changed

- [core] The `access_token` for http requests is now acquired by `login5`
- [core] MSRV is now 1.75 (breaking)
- [discovery] librespot can now be compiled with multiple MDNS/DNS-SD backends
  (avahi, dns_sd, libmdns) which can be selected using a CLI flag. The defaults
  are unchanged (breaking).

### Added

- [core] Add `get_token_with_client_id()` to get a token for a specific client ID
- [core] Add `login` (mobile) and `auth_token` retrieval via login5
- [core] Add `OS` and `os_version` to `config.rs`
- [discovery] Added a new MDNS/DNS-SD backend which connects to Avahi via D-Bus.

### Fixed

- [connect] Fixes initial volume showing zero despite playing in full volume instead
- [core] Fix "source slice length (16) does not match destination slice length
  (20)" panic on some tracks

## [0.5.0] - 2024-10-15

This version is be a major departure from the architecture up until now. It
focuses on implementing the "new Spotify API". This means moving large parts
of the Spotify protocol from Mercury to HTTP. A lot of this was reverse
engineered before by @devgianlu of librespot-java. It was long overdue that we
started implementing it too, not in the least because new features like the
hopefully upcoming Spotify HiFi depend on it.

Splitting up the work on the new Spotify API, v0.5.0 brings HTTP-based file
downloads and metadata access. Implementing the "dealer" (replacing the current
Mercury-based SPIRC message bus with WebSockets, also required for social plays)
is a large and separate effort, slated for some later release.

While at it, we are taking the liberty to do some major refactoring to make
librespot more robust. Consequently not only the Spotify API changed but large
parts of the librespot API too. For downstream maintainers, we realise that it
can be a lot to move from the current codebase to this one, but believe us it
will be well worth it.

All these changes are likely to introduce new bugs as well as some regressions.
We appreciate all your testing and contributions to the repository:
https://github.com/librespot-org/librespot

### Changed

- [all] Assertions were changed into `Result` or removed (breaking)
- [all] Purge use of `unwrap`, `expect` and return `Result` (breaking)
- [all] `chrono` replaced with `time` (breaking)
- [all] `time` updated (CVE-2020-26235)
- [all] Improve lock contention and performance (breaking)
- [all] Use a single `player` instance. Eliminates occasional `player` and
  `audio backend` restarts, which can cause issues with some playback
  configurations.
- [all] Updated and removed unused dependencies
- [audio] Files are now downloaded over the HTTPS CDN (breaking)
- [audio] Improve file opening and seeking performance (breaking)
- [core] MSRV is now 1.74 (breaking)
- [connect] `DeviceType` moved out of `connect` into `core` (breaking)
- [connect] Update and expose all `spirc` context fields (breaking)
- [connect] Add `Clone, Defaut` traits to `spirc` contexts
- [connect] Autoplay contexts are now retrieved with the `spclient` (breaking)
- [contrib] Updated Docker image
- [core] Message listeners are registered before authenticating. As a result
  there now is a separate `Session::new` and subsequent `session.connect`.
  (breaking)
- [core] `ConnectConfig` moved out of `core` into `connect` (breaking)
- [core] `client_id` for `get_token` moved to `SessionConfig` (breaking)
- [core] Mercury code has been refactored for better legibility (breaking)
- [core] Cache resolved access points during runtime (breaking)
- [core] `FileId` is moved out of `SpotifyId`. For now it will be re-exported.
- [core] Report actual platform data on login
- [core] Support `Session` authentication with a Spotify access token
- [core] `Credentials.username` is now an `Option` (breaking)
- [core] `Session::connect` tries multiple access points, retrying each one.
- [core] Each access point connection now timesout after 3 seconds.
- [core] Listen on both IPV4 and IPV6 on non-windows hosts
- [main] `autoplay {on|off}` now acts as an override. If unspecified, `librespot`
  now follows the setting in the Connect client that controls it. (breaking)
- [metadata] Most metadata is now retrieved with the `spclient` (breaking)
- [metadata] Playlists are moved to the `playlist4_external` protobuf (breaking)
- [metadata] Handle playlists that are sent with microsecond-based timestamps
- [playback] The audio decoder has been switched from `lewton` to `Symphonia`.
  This improves the Vorbis sound quality, adds support for MP3 as well as for
  FLAC in the future. (breaking)
- [playback] Improve reporting of actual playback cursor
- [playback] The passthrough decoder is now feature-gated (breaking)
- [playback] `rodio`: call play and pause
- [protocol] protobufs have been updated

### Added

- [all] Check that array indexes are within bounds (panic safety)
- [all] Wrap errors in librespot `Error` type (breaking)
- [audio] Make audio fetch parameters tunable
- [connect] Add option on which zeroconf will bind. Defaults to all interfaces. Ignored by DNS-SD.
- [connect] Add session events
- [connect] Add `repeat`, `set_position_ms` and `set_volume` to `spirc.rs`
- [contrib] Add `event_handler_example.py`
- [core] Send metrics with metadata queries: client ID, country & product
- [core] Verify Spotify server certificates (prevents man-in-the-middle attacks)
- [core] User attributes are stored in `Session` upon login, accessible with a
  getter and setter, and automatically updated as changes are pushed by the
  Spotify infrastructure (breaking)
- [core] HTTPS is now supported, including for proxies (breaking)
- [core] Resolve `spclient` and `dealer` access points (breaking)
- [core] Get and cache tokens through new token provider (breaking)
- [core] `spclient` is the API for HTTP-based calls to the Spotify servers.
  It supports a lot of functionality, including audio previews and image
  downloads even if librespot doesn't use that for playback itself.
- [core] Support downloading of lyrics
- [core] Support parsing `SpotifyId` for local files
- [core] Support parsing `SpotifyId` for named playlists
- [core] Add checks and handling for stale server connections.
- [core] Fix potential deadlock waiting for audio decryption keys.
- [discovery] Add option to show playback device as a group
- [main] Add all player events to `player_event_handler.rs`
- [main] Add an event worker thread that runs async to the main thread(s) but
  sync to itself to prevent potential data races for event consumers
- [metadata] All metadata fields in the protobufs are now exposed (breaking)
- [oauth] Standalone module to obtain Spotify access token using OAuth authorization code flow.
- [playback] Explicit tracks are skipped if the controlling Connect client has
  disabled such content. Applications that use librespot as a library without
  Connect should use the 'filter-explicit-content' user attribute in the session.
- [playback] Add metadata support via a `TrackChanged` event
- [connect] Add `activate` and `load` functions to `Spirc`, allowing control over local connect sessions
- [metadata] Add `Lyrics`
- [discovery] Add discovery initialisation retries if within the 1st min of uptime

### Fixed

- [connect] Set `PlayStatus` to the correct value when Player is loading to
  avoid blanking out the controls when `self.play_status` is `LoadingPlay` or
  `LoadingPause` in `spirc.rs`
- [connect] Handle attempts to play local files better by basically ignoring
  attempts to load them in `handle_remote_update` in `spirc.rs`
- [connect] Loading previous or next tracks, or looping back on repeat, will
  only start playback when we were already playing
- [connect, playback] Clean up and de-noise events and event firing
- [core] Fixed frequent disconnections for some users
- [core] More strict Spotify ID parsing
- [discovery] Update active user field upon connection
- [playback] Handle invalid track start positions by just starting the track
  from the beginning
- [playback] Handle disappearing and invalid devices better
- [playback] Handle seek, pause, and play commands while loading
- [playback] Handle disabled normalisation correctly when using fixed volume
- [playback] Do not stop sink in gapless mode
- [metadata] Fix missing colon when converting named spotify IDs to URIs

## [0.4.2] - 2022-07-29

Besides a couple of small fixes, this point release is mainly to blacklist the
ap-gew4 and ap-gue1 access points that caused librespot to fail to playback
anything.

Development will now shift to the new HTTP-based API, targeted for a future
v0.5.0 release. The new-api branch will therefore be promoted to dev. This is a
major departure from the old API and although it brings many exciting new
things, it is also likely to introduce new bugs and some regressions.

Long story short, this v0.4.2 release is the most stable that librespot has yet
to offer. But, unless anything big comes up, it is also intended as the last
release to be based on the old API. Happy listening.

### Changed

- [playback] `pipe`: Better error handling
- [playback] `subprocess`: Better error handling

### Added

- [core] `apresolve`: Blacklist ap-gew4 and ap-gue1 access points that cause channel errors
- [playback] `pipe`: Implement stop

### Fixed

- [main] fix `--opt=value` line argument logging
- [playback] `alsamixer`: make `--volume-ctrl fixed` work as expected when combined with `--mixer alsa`

## [0.4.1] - 2022-05-23

This release fixes dependency issues when installing from crates.

### Changed

- [chore] The MSRV is now 1.56

### Fixed

- [playback] Fixed dependency issues when installing from crate

## [0.4.0] - 2022-05-21

Note: This version was yanked, because a corrupt package was uploaded and failed
to install.

This is a polishing release, adding a few little extras and improving on many
thers. We had to break a couple of API's to do so, and therefore bumped the
minor version number. v0.4.x may be the last in series before we migrate from
the current channel-based Spotify backend to a more HTTP-based backend.
Targeting that major effort for a v0.5 release sometime, we intend to maintain
v0.4.x as a stable branch until then.

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

[unreleased]: https://github.com/librespot-org/librespot/compare/v0.7.1...HEAD
[0.7.1]: https://github.com/librespot-org/librespot/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/librespot-org/librespot/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/librespot-org/librespot/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/librespot-org/librespot/compare/v0.4.2...v0.5.0
[0.4.2]: https://github.com/librespot-org/librespot/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/librespot-org/librespot/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/librespot-org/librespot/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/librespot-org/librespot/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/librespot-org/librespot/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/librespot-org/librespot/compare/v0.1.6...v0.2.0
[0.1.6]: https://github.com/librespot-org/librespot/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/librespot-org/librespot/compare/v0.1.3...v0.1.5
[0.1.3]: https://github.com/librespot-org/librespot/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/librespot-org/librespot/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/librespot-org/librespot/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/librespot-org/librespot/releases/tag/v0.1.0
