use futures_util::{future, FutureExt, StreamExt};
use librespot_playback::player::PlayerEvent;
use log::{error, info, trace, warn};
use sha1::{Digest, Sha1};
use thiserror::Error;
use tokio::sync::mpsc::UnboundedReceiver;
use url::Url;

use librespot::connect::spirc::Spirc;
use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::{ConnectConfig, DeviceType, SessionConfig};
use librespot::core::session::Session;
use librespot::core::version;
use librespot::playback::audio_backend::{self, SinkBuilder, BACKENDS};
use librespot::playback::config::{
    AudioFormat, Bitrate, NormalisationMethod, NormalisationType, PlayerConfig, VolumeCtrl,
};
use librespot::playback::dither;
#[cfg(feature = "alsa-backend")]
use librespot::playback::mixer::alsamixer::AlsaMixer;
use librespot::playback::mixer::{self, MixerConfig, MixerFn};
use librespot::playback::player::{db_to_ratio, ratio_to_db, Player};

mod player_event_handler;
use player_event_handler::{emit_sink_event, run_program_on_events};

use std::env;
use std::io::{stderr, Write};
use std::ops::RangeInclusive;
use std::path::Path;
use std::pin::Pin;
use std::process::exit;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;

fn device_id(name: &str) -> String {
    hex::encode(Sha1::digest(name.as_bytes()))
}

fn usage(program: &str, opts: &getopts::Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    opts.usage(&brief)
}

fn arg_to_var(arg: &str) -> String {
    // To avoid name collisions environment variables must be prepended
    // with `LIBRESPOT_` so option/flag `foo-bar` becomes `LIBRESPOT_FOO_BAR`.
    format!("LIBRESPOT_{}", arg.to_uppercase().replace("-", "_"))
}

fn env_var_present(arg: &str) -> bool {
    env::var(arg_to_var(arg)).is_ok()
}

fn env_var_opt_str(option: &str) -> Option<String> {
    match env::var(arg_to_var(option)) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn setup_logging(quiet: bool, verbose: bool) {
    let mut builder = env_logger::Builder::new();
    match env::var("RUST_LOG") {
        Ok(config) => {
            builder.parse_filters(&config);
            builder.init();

            if verbose {
                warn!("`--verbose` flag overidden by `RUST_LOG` environment variable");
            } else if quiet {
                warn!("`--quiet` flag overidden by `RUST_LOG` environment variable");
            }
        }
        Err(_) => {
            if verbose {
                builder.parse_filters("libmdns=info,librespot=trace");
            } else if quiet {
                builder.parse_filters("libmdns=warn,librespot=warn");
            } else {
                builder.parse_filters("libmdns=info,librespot=info");
            }
            builder.init();

            if verbose && quiet {
                warn!("`--verbose` and `--quiet` are mutually exclusive. Logging can not be both verbose and quiet. Using verbose mode.");
            }
        }
    }
}

fn list_backends() {
    println!("Available backends: ");
    for (&(name, _), idx) in BACKENDS.iter().zip(0..) {
        if idx == 0 {
            println!("- {} (default)", name);
        } else {
            println!("- {}", name);
        }
    }
}

pub fn get_credentials<F: FnOnce(&String) -> Option<String>>(
    username: Option<String>,
    password: Option<String>,
    cached_credentials: Option<Credentials>,
    prompt: F,
) -> Option<Credentials> {
    if let Some(username) = username {
        if let Some(password) = password {
            return Some(Credentials::with_password(username, password));
        }

        match cached_credentials {
            Some(credentials) if username == credentials.username => Some(credentials),
            _ => {
                let password = prompt(&username)?;
                Some(Credentials::with_password(username, password))
            }
        }
    } else {
        cached_credentials
    }
}

#[derive(Debug, Error)]
pub enum ParseFileSizeError {
    #[error("empty argument")]
    EmptyInput,
    #[error("invalid suffix")]
    InvalidSuffix,
    #[error("invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseFloatError),
    #[error("non-finite number specified")]
    NotFinite(f64),
}

pub fn parse_file_size(input: &str) -> Result<u64, ParseFileSizeError> {
    use ParseFileSizeError::*;

    let mut iter = input.chars();
    let mut suffix = iter.next_back().ok_or(EmptyInput)?;
    let mut suffix_len = 0;

    let iec = matches!(suffix, 'i' | 'I');

    if iec {
        suffix_len += 1;
        suffix = iter.next_back().ok_or(InvalidSuffix)?;
    }

    let base: u64 = if iec { 1024 } else { 1000 };

    suffix_len += 1;
    let exponent = match suffix.to_ascii_uppercase() {
        '0'..='9' if !iec => {
            suffix_len -= 1;
            0
        }
        'K' => 1,
        'M' => 2,
        'G' => 3,
        'T' => 4,
        'P' => 5,
        'E' => 6,
        'Z' => 7,
        'Y' => 8,
        _ => return Err(InvalidSuffix),
    };

    let num = {
        let mut iter = input.chars();

        for _ in (&mut iter).rev().take(suffix_len) {}

        iter.as_str().parse::<f64>()?
    };

    if !num.is_finite() {
        return Err(NotFinite(num));
    }

    Ok((num * base.pow(exponent) as f64) as u64)
}

fn get_version_string() -> String {
    #[cfg(debug_assertions)]
    const BUILD_PROFILE: &str = "debug";
    #[cfg(not(debug_assertions))]
    const BUILD_PROFILE: &str = "release";

    format!(
        "librespot {semver} {sha} (Built on {build_date}, Build ID: {build_id}, Profile: {build_profile})",
        semver = version::SEMVER,
        sha = version::SHA_SHORT,
        build_date = version::BUILD_DATE,
        build_id = version::BUILD_ID,
        build_profile = BUILD_PROFILE
    )
}

struct Setup {
    format: AudioFormat,
    backend: SinkBuilder,
    device: Option<String>,
    mixer: MixerFn,
    cache: Option<Cache>,
    player_config: PlayerConfig,
    session_config: SessionConfig,
    connect_config: ConnectConfig,
    mixer_config: MixerConfig,
    credentials: Option<Credentials>,
    enable_discovery: bool,
    zeroconf_port: u16,
    player_event_program: Option<String>,
    emit_sink_events: bool,
}

fn get_setup(args: &[String]) -> Setup {
    const VALID_NORMALISATION_KNEE_RANGE: RangeInclusive<f64> = 0.0..=2.0;
    const VALID_VOLUME_RANGE: RangeInclusive<f64> = 0.0..=100.0;
    const VALID_NORMALISATION_PREGAIN_RANGE: RangeInclusive<f64> = -10.0..=10.0;
    const VALID_NORMALISATION_THRESHOLD_RANGE: RangeInclusive<f64> = -10.0..=0.0;
    const VALID_NORMALISATION_ATTACK_RANGE: RangeInclusive<u64> = 1..=500;
    const VALID_NORMALISATION_RELEASE_RANGE: RangeInclusive<u64> = 1..=1000;

    const AP_PORT: &str = "ap-port";
    const AUTOPLAY: &str = "autoplay";
    const BACKEND: &str = "backend";
    const BITRATE: &str = "bitrate";
    const CACHE: &str = "cache";
    const CACHE_SIZE_LIMIT: &str = "cache-size-limit";
    const DEVICE: &str = "device";
    const DEVICE_TYPE: &str = "device-type";
    const DISABLE_AUDIO_CACHE: &str = "disable-audio-cache";
    const DISABLE_CREDENTIAL_CACHE: &str = "disable-credential-cache";
    const DISABLE_DISCOVERY: &str = "disable-discovery";
    const DISABLE_GAPLESS: &str = "disable-gapless";
    const DITHER: &str = "dither";
    const EMIT_SINK_EVENTS: &str = "emit-sink-events";
    const ENABLE_VOLUME_NORMALISATION: &str = "enable-volume-normalisation";
    const FORMAT: &str = "format";
    const HELP: &str = "help";
    const INITIAL_VOLUME: &str = "initial-volume";
    const MIXER_TYPE: &str = "mixer";
    const ALSA_MIXER_DEVICE: &str = "alsa-mixer-device";
    const ALSA_MIXER_INDEX: &str = "alsa-mixer-index";
    const ALSA_MIXER_CONTROL: &str = "alsa-mixer-control";
    const NAME: &str = "name";
    const NORMALISATION_ATTACK: &str = "normalisation-attack";
    const NORMALISATION_GAIN_TYPE: &str = "normalisation-gain-type";
    const NORMALISATION_KNEE: &str = "normalisation-knee";
    const NORMALISATION_METHOD: &str = "normalisation-method";
    const NORMALISATION_PREGAIN: &str = "normalisation-pregain";
    const NORMALISATION_RELEASE: &str = "normalisation-release";
    const NORMALISATION_THRESHOLD: &str = "normalisation-threshold";
    const ONEVENT: &str = "onevent";
    const PASSTHROUGH: &str = "passthrough";
    const PASSWORD: &str = "password";
    const PROXY: &str = "proxy";
    const QUIET: &str = "quiet";
    const SYSTEM_CACHE: &str = "system-cache";
    const USERNAME: &str = "username";
    const VERBOSE: &str = "verbose";
    const VERSION: &str = "version";
    const VOLUME_CTRL: &str = "volume-ctrl";
    const VOLUME_RANGE: &str = "volume-range";
    const ZEROCONF_PORT: &str = "zeroconf-port";

    // Mostly arbitrary.
    const AUTOPLAY_SHORT: &str = "A";
    const AP_PORT_SHORT: &str = "a";
    const BACKEND_SHORT: &str = "B";
    const BITRATE_SHORT: &str = "b";
    const SYSTEM_CACHE_SHORT: &str = "C";
    const CACHE_SHORT: &str = "c";
    const DITHER_SHORT: &str = "D";
    const DEVICE_SHORT: &str = "d";
    const VOLUME_CTRL_SHORT: &str = "E";
    const VOLUME_RANGE_SHORT: &str = "e";
    const DEVICE_TYPE_SHORT: &str = "F";
    const FORMAT_SHORT: &str = "f";
    const DISABLE_AUDIO_CACHE_SHORT: &str = "G";
    const DISABLE_GAPLESS_SHORT: &str = "g";
    const DISABLE_CREDENTIAL_CACHE_SHORT: &str = "H";
    const HELP_SHORT: &str = "h";
    const CACHE_SIZE_LIMIT_SHORT: &str = "M";
    const MIXER_TYPE_SHORT: &str = "m";
    const ENABLE_VOLUME_NORMALISATION_SHORT: &str = "N";
    const NAME_SHORT: &str = "n";
    const DISABLE_DISCOVERY_SHORT: &str = "O";
    const ONEVENT_SHORT: &str = "o";
    const PASSTHROUGH_SHORT: &str = "P";
    const PASSWORD_SHORT: &str = "p";
    const EMIT_SINK_EVENTS_SHORT: &str = "Q";
    const QUIET_SHORT: &str = "q";
    const INITIAL_VOLUME_SHORT: &str = "R";
    const ALSA_MIXER_DEVICE_SHORT: &str = "S";
    const ALSA_MIXER_INDEX_SHORT: &str = "s";
    const ALSA_MIXER_CONTROL_SHORT: &str = "T";
    const NORMALISATION_ATTACK_SHORT: &str = "U";
    const USERNAME_SHORT: &str = "u";
    const VERSION_SHORT: &str = "V";
    const VERBOSE_SHORT: &str = "v";
    const NORMALISATION_GAIN_TYPE_SHORT: &str = "W";
    const NORMALISATION_KNEE_SHORT: &str = "w";
    const NORMALISATION_METHOD_SHORT: &str = "X";
    const PROXY_SHORT: &str = "x";
    const NORMALISATION_PREGAIN_SHORT: &str = "Y";
    const NORMALISATION_RELEASE_SHORT: &str = "y";
    const NORMALISATION_THRESHOLD_SHORT: &str = "Z";
    const ZEROCONF_PORT_SHORT: &str = "z";

    // Options that have different desc's
    // depending on what backends were enabled at build time.
    #[cfg(feature = "alsa-backend")]
    const MIXER_TYPE_DESC: &str = "Mixer to use {alsa|softvol}. Defaults to softvol.";
    #[cfg(not(feature = "alsa-backend"))]
    const MIXER_TYPE_DESC: &str = "Not supported by the included audio backend(s).";
    #[cfg(any(
        feature = "alsa-backend",
        feature = "rodio-backend",
        feature = "portaudio-backend"
    ))]
    const DEVICE_DESC: &str = "Audio device to use. Use ? to list options if using alsa, portaudio or rodio. Defaults to the backend's default.";
    #[cfg(not(any(
        feature = "alsa-backend",
        feature = "rodio-backend",
        feature = "portaudio-backend"
    )))]
    const DEVICE_DESC: &str = "Not supported by the included audio backend(s).";
    #[cfg(feature = "alsa-backend")]
    const ALSA_MIXER_CONTROL_DESC: &str =
        "Alsa mixer control, e.g. PCM, Master or similar. Defaults to PCM.";
    #[cfg(not(feature = "alsa-backend"))]
    const ALSA_MIXER_CONTROL_DESC: &str = "Not supported by the included audio backend(s).";
    #[cfg(feature = "alsa-backend")]
    const ALSA_MIXER_DEVICE_DESC: &str = "Alsa mixer device, e.g hw:0 or similar from `aplay -l`. Defaults to `--device` if specified, default otherwise.";
    #[cfg(not(feature = "alsa-backend"))]
    const ALSA_MIXER_DEVICE_DESC: &str = "Not supported by the included audio backend(s).";
    #[cfg(feature = "alsa-backend")]
    const ALSA_MIXER_INDEX_DESC: &str = "Alsa index of the cards mixer. Defaults to 0.";
    #[cfg(not(feature = "alsa-backend"))]
    const ALSA_MIXER_INDEX_DESC: &str = "Not supported by the included audio backend(s).";
    #[cfg(feature = "alsa-backend")]
    const INITIAL_VOLUME_DESC: &str = "Initial volume in % from 0 - 100. Default for softvol: 50. For the alsa mixer: the current volume.";
    #[cfg(not(feature = "alsa-backend"))]
    const INITIAL_VOLUME_DESC: &str = "Initial volume in % from 0 - 100. Defaults to 50.";
    #[cfg(feature = "alsa-backend")]
    const VOLUME_RANGE_DESC: &str = "Range of the volume control (dB) from 0.0 to 100.0. Default for softvol: 60.0. For the alsa mixer: what the control supports.";
    #[cfg(not(feature = "alsa-backend"))]
    const VOLUME_RANGE_DESC: &str =
        "Range of the volume control (dB) from 0.0 to 100.0. Defaults to 60.0.";

    let mut opts = getopts::Options::new();
    opts.optflag(
        HELP_SHORT,
        HELP,
        "Print this help menu.",
    )
    .optflag(
        VERSION_SHORT,
        VERSION,
        "Display librespot version string.",
    )
    .optflag(
        VERBOSE_SHORT,
        VERBOSE,
        "Enable verbose log output.",
    )
    .optflag(
        QUIET_SHORT,
        QUIET,
        "Only log warning and error messages.",
    )
    .optflag(
        DISABLE_AUDIO_CACHE_SHORT,
        DISABLE_AUDIO_CACHE,
        "Disable caching of the audio data.",
    )
    .optflag(
        DISABLE_CREDENTIAL_CACHE_SHORT,
        DISABLE_CREDENTIAL_CACHE,
        "Disable caching of credentials.",
    )
    .optflag(
        DISABLE_DISCOVERY_SHORT,
        DISABLE_DISCOVERY,
        "Disable zeroconf discovery mode.",
    )
    .optflag(
        DISABLE_GAPLESS_SHORT,
        DISABLE_GAPLESS,
        "Disable gapless playback.",
    )
    .optflag(
        EMIT_SINK_EVENTS_SHORT,
        EMIT_SINK_EVENTS,
        "Run PROGRAM set by `--onevent` before the sink is opened and after it is closed.",
    )
    .optflag(
        AUTOPLAY_SHORT,
        AUTOPLAY,
        "Automatically play similar songs when your music ends.",
    )
    .optflag(
        PASSTHROUGH_SHORT,
        PASSTHROUGH,
        "Pass a raw stream to the output. Only works with the pipe and subprocess backends.",
    )
    .optflag(
        ENABLE_VOLUME_NORMALISATION_SHORT,
        ENABLE_VOLUME_NORMALISATION,
        "Play all tracks at approximately the same apparent volume.",
    )
    .optopt(
        NAME_SHORT,
        NAME,
        "Device name. Defaults to Librespot.",
        "NAME",
    )
    .optopt(
        BITRATE_SHORT,
        BITRATE,
        "Bitrate (kbps) {96|160|320}. Defaults to 160.",
        "BITRATE",
    )
    .optopt(
        FORMAT_SHORT,
        FORMAT,
        "Output format {F64|F32|S32|S24|S24_3|S16}. Defaults to S16.",
        "FORMAT",
    )
    .optopt(
        DITHER_SHORT,
        DITHER,
        "Specify the dither algorithm to use {none|gpdf|tpdf|tpdf_hp}. Defaults to tpdf for formats S16, S24, S24_3 and none for other formats.",
        "DITHER",
    )
    .optopt(
        DEVICE_TYPE_SHORT,
        DEVICE_TYPE,
        "Displayed device type. Defaults to speaker.",
        "TYPE",
    )
    .optopt(
        CACHE_SHORT,
        CACHE,
        "Path to a directory where files will be cached.",
        "PATH",
    )
    .optopt(
        SYSTEM_CACHE_SHORT,
        SYSTEM_CACHE,
        "Path to a directory where system files (credentials, volume) will be cached. May be different from the `--cache` option value.",
        "PATH",
    )
    .optopt(
        CACHE_SIZE_LIMIT_SHORT,
        CACHE_SIZE_LIMIT,
        "Limits the size of the cache for audio files. It's possible to use suffixes like K, M or G, e.g. 16G for example.",
        "SIZE"
    )
    .optopt(
        BACKEND_SHORT,
        BACKEND,
        "Audio backend to use. Use ? to list options.",
        "NAME",
    )
    .optopt(
        USERNAME_SHORT,
        USERNAME,
        "Username used to sign in with.",
        "USERNAME",
    )
    .optopt(
        PASSWORD_SHORT,
        PASSWORD,
        "Password used to sign in with.",
        "PASSWORD",
    )
    .optopt(
        ONEVENT_SHORT,
        ONEVENT,
        "Run PROGRAM when a playback event occurs.",
        "PROGRAM",
    )
    .optopt(
        ALSA_MIXER_CONTROL_SHORT,
        ALSA_MIXER_CONTROL,
        ALSA_MIXER_CONTROL_DESC,
        "NAME",
    )
    .optopt(
        ALSA_MIXER_DEVICE_SHORT,
        ALSA_MIXER_DEVICE,
        ALSA_MIXER_DEVICE_DESC,
        "DEVICE",
    )
    .optopt(
        ALSA_MIXER_INDEX_SHORT,
        ALSA_MIXER_INDEX,
        ALSA_MIXER_INDEX_DESC,
        "NUMBER",
    )
    .optopt(
        MIXER_TYPE_SHORT,
        MIXER_TYPE,
        MIXER_TYPE_DESC,
        "MIXER",
    )
    .optopt(
        DEVICE_SHORT,
        DEVICE,
        DEVICE_DESC,
        "NAME",
    )
    .optopt(
        INITIAL_VOLUME_SHORT,
        INITIAL_VOLUME,
        INITIAL_VOLUME_DESC,
        "VOLUME",
    )
    .optopt(
        VOLUME_CTRL_SHORT,
        VOLUME_CTRL,
        "Volume control scale type {cubic|fixed|linear|log}. Defaults to log.",
        "VOLUME_CTRL"
    )
    .optopt(
        VOLUME_RANGE_SHORT,
        VOLUME_RANGE,
        VOLUME_RANGE_DESC,
        "RANGE",
    )
    .optopt(
        NORMALISATION_METHOD_SHORT,
        NORMALISATION_METHOD,
        "Specify the normalisation method to use {basic|dynamic}. Defaults to dynamic.",
        "METHOD",
    )
    .optopt(
        NORMALISATION_GAIN_TYPE_SHORT,
        NORMALISATION_GAIN_TYPE,
        "Specify the normalisation gain type to use {track|album|auto}. Defaults to auto.",
        "TYPE",
    )
    .optopt(
        NORMALISATION_PREGAIN_SHORT,
        NORMALISATION_PREGAIN,
        "Pregain (dB) applied by volume normalisation from -10.0 to 10.0. Defaults to 0.0.",
        "PREGAIN",
    )
    .optopt(
        NORMALISATION_THRESHOLD_SHORT,
        NORMALISATION_THRESHOLD,
        "Threshold (dBFS) at which point the dynamic limiter engages to prevent clipping from 0.0 to -10.0. Defaults to -2.0.",
        "THRESHOLD",
    )
    .optopt(
        NORMALISATION_ATTACK_SHORT,
        NORMALISATION_ATTACK,
        "Attack time (ms) in which the dynamic limiter reduces gain from 1 to 500. Defaults to 5.",
        "TIME",
    )
    .optopt(
        NORMALISATION_RELEASE_SHORT,
        NORMALISATION_RELEASE,
        "Release or decay time (ms) in which the dynamic limiter restores gain from 1 to 1000. Defaults to 100.",
        "TIME",
    )
    .optopt(
        NORMALISATION_KNEE_SHORT,
        NORMALISATION_KNEE,
        "Knee steepness of the dynamic limiter from 0.0 to 2.0. Defaults to 1.0.",
        "KNEE",
    )
    .optopt(
        ZEROCONF_PORT_SHORT,
        ZEROCONF_PORT,
        "The port the internal server advertises over zeroconf 1 - 65535. Ports <= 1024 may require root privileges.",
        "PORT",
    )
    .optopt(
        PROXY_SHORT,
        PROXY,
        "HTTP proxy to use when connecting.",
        "URL",
    )
    .optopt(
        AP_PORT_SHORT,
        AP_PORT,
        "Connect to an AP with a specified port 1 - 65535. If no AP with that port is present a fallback AP will be used. Available ports are usually 80, 443 and 4070.",
        "PORT",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!(
                "Error parsing command line options: {}\n{}",
                f,
                usage(&args[0], &opts)
            );
            exit(1);
        }
    };

    let opt_present = |opt| matches.opt_present(opt) || env_var_present(opt);

    let opt_str = |opt| {
        if matches.opt_present(opt) {
            matches.opt_str(opt)
        } else {
            env_var_opt_str(opt)
        }
    };

    if opt_present(HELP) {
        println!("{}", usage(&args[0], &opts));
        exit(0);
    }

    if opt_present(VERSION) {
        println!("{}", get_version_string());
        exit(0);
    }

    setup_logging(opt_present(QUIET), opt_present(VERBOSE));

    info!("{}", get_version_string());

    let librespot_env_vars: Vec<String> = env::vars_os()
        .filter_map(|(k, v)| {
            let mut env_var = None;
            if let Some(key) = k.to_str() {
                if key.starts_with("LIBRESPOT_") {
                    if matches!(key, "LIBRESPOT_PASSWORD" | "LIBRESPOT_USERNAME") {
                        // Don't log creds.
                        env_var = Some(format!("\t\t{}=XXXXXXXX", key));
                    } else if let Some(value) = v.to_str() {
                        env_var = Some(format!("\t\t{}={}", key, value));
                    }
                }
            }

            env_var
        })
        .collect();

    if !librespot_env_vars.is_empty() {
        trace!("Environment variable(s):");

        for kv in librespot_env_vars {
            trace!("{}", kv);
        }
    }

    let cmd_args = &args[1..];

    let cmd_args_len = cmd_args.len();

    if cmd_args_len > 0 {
        trace!("Command line argument(s):");

        for (index, key) in cmd_args.iter().enumerate() {
            if key.starts_with('-') || key.starts_with("--") {
                if matches!(key.as_str(), "--password" | "-p" | "--username" | "-u") {
                    // Don't log creds.
                    trace!("\t\t{} XXXXXXXX", key);
                } else {
                    let mut value = "".to_string();
                    let next = index + 1;
                    if next < cmd_args_len {
                        let next_key = cmd_args[next].clone();
                        if !next_key.starts_with('-') && !next_key.starts_with("--") {
                            value = next_key;
                        }
                    }

                    trace!("\t\t{} {}", key, value);
                }
            }
        }
    }

    #[cfg(not(feature = "alsa-backend"))]
    for a in &[
        MIXER_TYPE,
        ALSA_MIXER_DEVICE,
        ALSA_MIXER_INDEX,
        ALSA_MIXER_CONTROL,
    ] {
        if opt_present(a) {
            warn!("Alsa specific options have no effect if the alsa backend is not enabled at build time.");
            break;
        }
    }

    let backend_name = opt_str(BACKEND);
    if backend_name == Some("?".into()) {
        list_backends();
        exit(0);
    }

    let backend = audio_backend::find(backend_name).unwrap_or_else(|| {
        error!(
            "Invalid `--{}` / `-{}`: {}",
            BACKEND,
            BACKEND_SHORT,
            opt_str(BACKEND).unwrap_or_default()
        );
        list_backends();
        exit(1);
    });

    let format = opt_str(FORMAT)
        .as_deref()
        .map(|format| {
            AudioFormat::from_str(format).unwrap_or_else(|_| {
                error!("Invalid `--{}` / `-{}`: {}", FORMAT, FORMAT_SHORT, format);
                println!(
                    "Valid `--{}` / `-{}` values: F64, F32, S32, S24, S24_3, S16",
                    FORMAT, FORMAT_SHORT
                );
                println!("Default: {:?}", AudioFormat::default());
                exit(1);
            })
        })
        .unwrap_or_default();

    #[cfg(any(
        feature = "alsa-backend",
        feature = "rodio-backend",
        feature = "portaudio-backend"
    ))]
    let device = opt_str(DEVICE);

    #[cfg(any(
        feature = "alsa-backend",
        feature = "rodio-backend",
        feature = "portaudio-backend"
    ))]
    if device == Some("?".into()) {
        backend(device, format);
        exit(0);
    }

    #[cfg(not(any(
        feature = "alsa-backend",
        feature = "rodio-backend",
        feature = "portaudio-backend"
    )))]
    let device: Option<String> = None;

    #[cfg(not(any(
        feature = "alsa-backend",
        feature = "rodio-backend",
        feature = "portaudio-backend"
    )))]
    if opt_present(DEVICE) {
        warn!(
            "The `--{}` / `-{}` option is not supported by the included audio backend(s), and has no effect.",
            DEVICE, DEVICE_SHORT,
        );
    }

    #[cfg(feature = "alsa-backend")]
    let mixer_type = opt_str(MIXER_TYPE);
    #[cfg(not(feature = "alsa-backend"))]
    let mixer_type: Option<String> = None;

    let mixer = mixer::find(mixer_type.as_deref()).unwrap_or_else(|| {
        error!(
            "Invalid `--{}` / `-{}`: {}",
            MIXER_TYPE,
            MIXER_TYPE_SHORT,
            opt_str(MIXER_TYPE).unwrap_or_default()
        );
        println!(
            "Valid `--{}` / `-{}` values: alsa, softvol",
            MIXER_TYPE, MIXER_TYPE_SHORT
        );
        println!("Default: softvol");
        exit(1);
    });

    let mixer_config = {
        let mixer_default_config = MixerConfig::default();

        #[cfg(feature = "alsa-backend")]
        let device = opt_str(ALSA_MIXER_DEVICE).unwrap_or_else(|| {
            if let Some(ref device_name) = device {
                device_name.to_string()
            } else {
                mixer_default_config.device.clone()
            }
        });

        #[cfg(not(feature = "alsa-backend"))]
        let device = mixer_default_config.device;

        #[cfg(feature = "alsa-backend")]
        let index = opt_str(ALSA_MIXER_INDEX)
            .map(|index| {
                index.parse::<u32>().unwrap_or_else(|_| {
                    error!(
                        "Invalid `--{}` / `-{}`: {}",
                        ALSA_MIXER_INDEX, ALSA_MIXER_INDEX_SHORT, index
                    );
                    println!("Default: {}", mixer_default_config.index);
                    exit(1);
                })
            })
            .unwrap_or_else(|| mixer_default_config.index);

        #[cfg(not(feature = "alsa-backend"))]
        let index = mixer_default_config.index;

        #[cfg(feature = "alsa-backend")]
        let control = opt_str(ALSA_MIXER_CONTROL).unwrap_or(mixer_default_config.control);

        #[cfg(not(feature = "alsa-backend"))]
        let control = mixer_default_config.control;

        let volume_range = opt_str(VOLUME_RANGE)
            .map(|range| {
                let on_error = || {
                    error!(
                        "Invalid `--{}` / `-{}`: {}",
                        VOLUME_RANGE, VOLUME_RANGE_SHORT, range
                    );
                    println!(
                        "Valid `--{}` / `-{}` values: {} - {}",
                        VOLUME_RANGE,
                        VOLUME_RANGE_SHORT,
                        VALID_VOLUME_RANGE.start(),
                        VALID_VOLUME_RANGE.end()
                    );
                    #[cfg(feature = "alsa-backend")]
                    println!(
                        "Default: softvol - {}, alsa - what the control supports",
                        VolumeCtrl::DEFAULT_DB_RANGE
                    );
                    #[cfg(not(feature = "alsa-backend"))]
                    println!("Default: {}", VolumeCtrl::DEFAULT_DB_RANGE);
                };

                let range = range.parse::<f64>().unwrap_or_else(|_| {
                    on_error();
                    exit(1);
                });

                if !(VALID_VOLUME_RANGE).contains(&range) {
                    on_error();
                    exit(1);
                }

                range
            })
            .unwrap_or_else(|| match mixer_type.as_deref() {
                #[cfg(feature = "alsa-backend")]
                Some(AlsaMixer::NAME) => 0.0, // let alsa query the control
                _ => VolumeCtrl::DEFAULT_DB_RANGE,
            });

        let volume_ctrl = opt_str(VOLUME_CTRL)
            .as_deref()
            .map(|volume_ctrl| {
                VolumeCtrl::from_str_with_range(volume_ctrl, volume_range).unwrap_or_else(|_| {
                    error!(
                        "Invalid `--{}` / `-{}`: {}",
                        VOLUME_CTRL, VOLUME_CTRL_SHORT, volume_ctrl
                    );
                    println!(
                        "Valid `--{}` / `-{}` values: cubic, fixed, linear, log",
                        VOLUME_CTRL, VOLUME_CTRL
                    );
                    println!("Default: log");
                    exit(1);
                })
            })
            .unwrap_or_else(|| VolumeCtrl::Log(volume_range));

        MixerConfig {
            device,
            control,
            index,
            volume_ctrl,
        }
    };

    let cache = {
        let volume_dir = opt_str(SYSTEM_CACHE)
            .or_else(|| opt_str(CACHE))
            .map(|p| p.into());

        let cred_dir = if opt_present(DISABLE_CREDENTIAL_CACHE) {
            None
        } else {
            volume_dir.clone()
        };

        let audio_dir = if opt_present(DISABLE_AUDIO_CACHE) {
            None
        } else {
            opt_str(CACHE)
                .as_ref()
                .map(|p| AsRef::<Path>::as_ref(p).join("files"))
        };

        let limit = if audio_dir.is_some() {
            opt_str(CACHE_SIZE_LIMIT)
                .as_deref()
                .map(parse_file_size)
                .map(|e| {
                    e.unwrap_or_else(|e| {
                        error!(
                            "Invalid `--{}` / `-{}`: {}",
                            CACHE_SIZE_LIMIT, CACHE_SIZE_LIMIT_SHORT, e
                        );
                        exit(1);
                    })
                })
        } else {
            None
        };

        if audio_dir.is_none() && opt_present(CACHE_SIZE_LIMIT) {
            warn!(
                "Without a `--{}` / `-{}` path, and/or if the `--{}` / `-{}` flag is set, `--{}` / `-{}` has no effect.",
                CACHE, CACHE_SHORT, DISABLE_AUDIO_CACHE, DISABLE_AUDIO_CACHE_SHORT, CACHE_SIZE_LIMIT, CACHE_SIZE_LIMIT_SHORT
            );
        }

        match Cache::new(cred_dir, volume_dir, audio_dir, limit) {
            Ok(cache) => Some(cache),
            Err(e) => {
                warn!("Cannot create cache: {}", e);
                None
            }
        }
    };

    let credentials = {
        let cached_credentials = cache.as_ref().and_then(Cache::credentials);

        let password = |username: &String| -> Option<String> {
            write!(stderr(), "Password for {}: ", username).ok()?;
            stderr().flush().ok()?;
            rpassword::read_password().ok()
        };

        get_credentials(
            opt_str(USERNAME),
            opt_str(PASSWORD),
            cached_credentials,
            password,
        )
    };

    let enable_discovery = !opt_present(DISABLE_DISCOVERY);

    if credentials.is_none() && !enable_discovery {
        error!("Credentials are required if discovery is disabled.");
        exit(1);
    }

    if !enable_discovery && opt_present(ZEROCONF_PORT) {
        warn!(
            "With the `--{}` / `-{}` flag set `--{}` / `-{}` has no effect.",
            DISABLE_DISCOVERY, DISABLE_DISCOVERY_SHORT, ZEROCONF_PORT, ZEROCONF_PORT_SHORT
        );
    }

    let zeroconf_port = if enable_discovery {
        opt_str(ZEROCONF_PORT)
            .map(|port| {
                let on_error = || {
                    error!(
                        "Invalid `--{}` / `-{}`: {}",
                        ZEROCONF_PORT, ZEROCONF_PORT_SHORT, port
                    );
                    println!(
                        "Valid `--{}` / `-{}` values: 1 - 65535",
                        ZEROCONF_PORT, ZEROCONF_PORT_SHORT
                    );
                };

                let port = port.parse::<u16>().unwrap_or_else(|_| {
                    on_error();
                    exit(1);
                });

                if port == 0 {
                    on_error();
                    exit(1);
                }

                port
            })
            .unwrap_or(0)
    } else {
        0
    };

    let connect_config = {
        let connect_default_config = ConnectConfig::default();

        let name = opt_str(NAME).unwrap_or_else(|| connect_default_config.name.clone());

        let initial_volume = opt_str(INITIAL_VOLUME)
            .map(|initial_volume| {
                let on_error = || {
                    error!(
                        "Invalid `--{}` / `-{}`: {}",
                        INITIAL_VOLUME, INITIAL_VOLUME_SHORT, initial_volume
                    );
                    println!(
                        "Valid `--{}` / `-{}` values: 0 - 100",
                        INITIAL_VOLUME, INITIAL_VOLUME_SHORT
                    );
                    #[cfg(feature = "alsa-backend")]
                    println!(
                        "Default: {}, or the current value when the alsa mixer is used.",
                        connect_default_config.initial_volume.unwrap_or_default()
                    );
                    #[cfg(not(feature = "alsa-backend"))]
                    println!(
                        "Default: {}",
                        connect_default_config.initial_volume.unwrap_or_default()
                    );
                };

                let volume = initial_volume.parse::<u16>().unwrap_or_else(|_| {
                    on_error();
                    exit(1);
                });

                if volume > 100 {
                    on_error();
                    exit(1);
                }

                (volume as f32 / 100.0 * VolumeCtrl::MAX_VOLUME as f32) as u16
            })
            .or_else(|| match mixer_type.as_deref() {
                #[cfg(feature = "alsa-backend")]
                Some(AlsaMixer::NAME) => None,
                _ => cache.as_ref().and_then(Cache::volume),
            });

        let device_type = opt_str(DEVICE_TYPE)
            .as_deref()
            .map(|device_type| {
                DeviceType::from_str(device_type).unwrap_or_else(|_| {
                    error!("Invalid `--{}` / `-{}`: {}", DEVICE_TYPE, DEVICE_TYPE_SHORT, device_type);
                    println!("Valid `--{}` / `-{}` values: computer, tablet, smartphone, speaker, tv, avr, stb, audiodongle, \
                        gameconsole, castaudio, castvideo, automobile, smartwatch, chromebook, carthing, homething",
                        DEVICE_TYPE, DEVICE_TYPE_SHORT
                    );
                    println!("Default: speaker");
                    exit(1);
                })
            })
            .unwrap_or_default();

        let has_volume_ctrl = !matches!(mixer_config.volume_ctrl, VolumeCtrl::Fixed);
        let autoplay = opt_present(AUTOPLAY);

        ConnectConfig {
            name,
            device_type,
            initial_volume,
            has_volume_ctrl,
            autoplay,
        }
    };

    let session_config = {
        let device_id = device_id(&connect_config.name);

        SessionConfig {
            user_agent: version::VERSION_STRING.to_string(),
            device_id,
            proxy: opt_str(PROXY).or_else(|| std::env::var("http_proxy").ok()).map(
                |s| {
                    match Url::parse(&s) {
                        Ok(url) => {
                            if url.host().is_none() || url.port_or_known_default().is_none() {
                                error!("Invalid proxy url, only URLs on the format \"http://host:port\" are allowed");
                                exit(1);
                            }

                            if url.scheme() != "http" {
                                error!("Only unsecure http:// proxies are supported");
                                exit(1);
                            }

                            url
                        },
                        Err(e) => {
                            error!("Invalid proxy URL: {}, only URLs in the format \"http://host:port\" are allowed", e);
                            exit(1);
                        }
                    }
                },
            ),
            ap_port: opt_str(AP_PORT)
                .map(|port| {
                    let on_error = || {
                        error!("Invalid `--{}` / `-{}`: {}", AP_PORT, AP_PORT_SHORT, port);
                        println!("Valid `--{}` / `-{}` values: 1 - 65535", AP_PORT, AP_PORT_SHORT);
                    };

                    let port = port.parse::<u16>().unwrap_or_else(|_| {
                        on_error();
                        exit(1);
                    });

                    if port == 0 {
                        on_error();
                        exit(1);
                    }

                    port
                }),
        }
    };

    let player_config = {
        let player_default_config = PlayerConfig::default();

        let bitrate = opt_str(BITRATE)
            .as_deref()
            .map(|bitrate| {
                Bitrate::from_str(bitrate).unwrap_or_else(|_| {
                    error!(
                        "Invalid `--{}` / `-{}`: {}",
                        BITRATE, BITRATE_SHORT, bitrate
                    );
                    println!(
                        "Valid `--{}` / `-{}` values: 96, 160, 320",
                        BITRATE, BITRATE_SHORT
                    );
                    println!("Default: 160");
                    exit(1);
                })
            })
            .unwrap_or(player_default_config.bitrate);

        let gapless = !opt_present(DISABLE_GAPLESS);

        let normalisation = opt_present(ENABLE_VOLUME_NORMALISATION);

        let normalisation_method;
        let normalisation_type;
        let normalisation_pregain;
        let normalisation_threshold;
        let normalisation_attack;
        let normalisation_release;
        let normalisation_knee;

        if !normalisation {
            for a in &[
                NORMALISATION_METHOD,
                NORMALISATION_GAIN_TYPE,
                NORMALISATION_PREGAIN,
                NORMALISATION_THRESHOLD,
                NORMALISATION_ATTACK,
                NORMALISATION_RELEASE,
                NORMALISATION_KNEE,
            ] {
                if opt_present(a) {
                    warn!(
                        "Without the `--{}` / `-{}` flag normalisation options have no effect.",
                        ENABLE_VOLUME_NORMALISATION, ENABLE_VOLUME_NORMALISATION_SHORT,
                    );
                    break;
                }
            }

            normalisation_method = player_default_config.normalisation_method;
            normalisation_type = player_default_config.normalisation_type;
            normalisation_pregain = player_default_config.normalisation_pregain;
            normalisation_threshold = player_default_config.normalisation_threshold;
            normalisation_attack = player_default_config.normalisation_attack;
            normalisation_release = player_default_config.normalisation_release;
            normalisation_knee = player_default_config.normalisation_knee;
        } else {
            normalisation_method = opt_str(NORMALISATION_METHOD)
                .as_deref()
                .map(|method| {
                    warn!(
                        "`--{}` / `-{}` will be deprecated in a future release.",
                        NORMALISATION_METHOD, NORMALISATION_METHOD_SHORT
                    );

                    let method = NormalisationMethod::from_str(method).unwrap_or_else(|_| {
                        error!(
                            "Invalid `--{}` / `-{}`: {}",
                            NORMALISATION_METHOD, NORMALISATION_METHOD_SHORT, method
                        );
                        println!(
                            "Valid `--{}` / `-{}` values: basic, dynamic",
                            NORMALISATION_METHOD, NORMALISATION_METHOD_SHORT
                        );
                        println!("Default: {:?}", player_default_config.normalisation_method);
                        exit(1);
                    });

                    if matches!(method, NormalisationMethod::Basic) {
                        warn!(
                            "`--{}` / `-{}` {:?} will be deprecated in a future release.",
                            NORMALISATION_METHOD, NORMALISATION_METHOD_SHORT, method
                        );
                    }

                    method
                })
                .unwrap_or(player_default_config.normalisation_method);

            normalisation_type = opt_str(NORMALISATION_GAIN_TYPE)
                .as_deref()
                .map(|gain_type| {
                    NormalisationType::from_str(gain_type).unwrap_or_else(|_| {
                        error!(
                            "Invalid `--{}` / `-{}`: {}",
                            NORMALISATION_GAIN_TYPE, NORMALISATION_GAIN_TYPE_SHORT, gain_type
                        );
                        println!(
                            "Valid `--{}` / `-{}` values: track, album, auto",
                            NORMALISATION_GAIN_TYPE, NORMALISATION_GAIN_TYPE_SHORT,
                        );
                        println!("Default: {:?}", player_default_config.normalisation_type);
                        exit(1);
                    })
                })
                .unwrap_or(player_default_config.normalisation_type);

            normalisation_pregain = opt_str(NORMALISATION_PREGAIN)
                .map(|pregain| {
                    let on_error = || {
                        error!(
                            "Invalid `--{}` / `-{}`: {}",
                            NORMALISATION_PREGAIN, NORMALISATION_PREGAIN_SHORT, pregain
                        );
                        println!(
                            "Valid `--{}` / `-{}` values: {} - {}",
                            NORMALISATION_PREGAIN,
                            NORMALISATION_PREGAIN_SHORT,
                            VALID_NORMALISATION_PREGAIN_RANGE.start(),
                            VALID_NORMALISATION_PREGAIN_RANGE.end()
                        );
                        println!("Default: {}", player_default_config.normalisation_pregain);
                    };

                    let pregain = pregain.parse::<f64>().unwrap_or_else(|_| {
                        on_error();
                        exit(1);
                    });

                    if !(VALID_NORMALISATION_PREGAIN_RANGE).contains(&pregain) {
                        on_error();
                        exit(1);
                    }

                    pregain
                })
                .unwrap_or(player_default_config.normalisation_pregain);

            normalisation_threshold = opt_str(NORMALISATION_THRESHOLD)
                .map(|threshold| {
                    let on_error = || {
                        error!(
                            "Invalid `--{}` / `-{}`: {}",
                            NORMALISATION_THRESHOLD, NORMALISATION_THRESHOLD_SHORT, threshold
                        );
                        println!(
                            "Valid `--{}` / `-{}` values: {} - {}",
                            NORMALISATION_THRESHOLD,
                            NORMALISATION_THRESHOLD_SHORT,
                            VALID_NORMALISATION_THRESHOLD_RANGE.start(),
                            VALID_NORMALISATION_THRESHOLD_RANGE.end()
                        );
                        println!(
                            "Default: {}",
                            ratio_to_db(player_default_config.normalisation_threshold)
                        );
                    };

                    let threshold = threshold.parse::<f64>().unwrap_or_else(|_| {
                        on_error();
                        exit(1);
                    });

                    if !(VALID_NORMALISATION_THRESHOLD_RANGE).contains(&threshold) {
                        on_error();
                        exit(1);
                    }

                    db_to_ratio(threshold)
                })
                .unwrap_or(player_default_config.normalisation_threshold);

            normalisation_attack = opt_str(NORMALISATION_ATTACK)
                .map(|attack| {
                    let on_error = || {
                        error!(
                            "Invalid `--{}` / `-{}`: {}",
                            NORMALISATION_ATTACK, NORMALISATION_ATTACK_SHORT, attack
                        );
                        println!(
                            "Valid `--{}` / `-{}` values: {} - {}",
                            NORMALISATION_ATTACK,
                            NORMALISATION_ATTACK_SHORT,
                            VALID_NORMALISATION_ATTACK_RANGE.start(),
                            VALID_NORMALISATION_ATTACK_RANGE.end()
                        );
                        println!(
                            "Default: {}",
                            player_default_config.normalisation_attack.as_millis()
                        );
                    };

                    let attack = attack.parse::<u64>().unwrap_or_else(|_| {
                        on_error();
                        exit(1);
                    });

                    if !(VALID_NORMALISATION_ATTACK_RANGE).contains(&attack) {
                        on_error();
                        exit(1);
                    }

                    Duration::from_millis(attack)
                })
                .unwrap_or(player_default_config.normalisation_attack);

            normalisation_release = opt_str(NORMALISATION_RELEASE)
                .map(|release| {
                    let on_error = || {
                        error!(
                            "Invalid `--{}` / `-{}`: {}",
                            NORMALISATION_RELEASE, NORMALISATION_RELEASE_SHORT, release
                        );
                        println!(
                            "Valid `--{}` / `-{}` values: {} - {}",
                            NORMALISATION_RELEASE,
                            NORMALISATION_RELEASE_SHORT,
                            VALID_NORMALISATION_RELEASE_RANGE.start(),
                            VALID_NORMALISATION_RELEASE_RANGE.end()
                        );
                        println!(
                            "Default: {}",
                            player_default_config.normalisation_release.as_millis()
                        );
                    };

                    let release = release.parse::<u64>().unwrap_or_else(|_| {
                        on_error();
                        exit(1);
                    });

                    if !(VALID_NORMALISATION_RELEASE_RANGE).contains(&release) {
                        on_error();
                        exit(1);
                    }

                    Duration::from_millis(release)
                })
                .unwrap_or(player_default_config.normalisation_release);

            normalisation_knee = opt_str(NORMALISATION_KNEE)
                .map(|knee| {
                    let on_error = || {
                        error!(
                            "Invalid `--{}` / `-{}`: {}",
                            NORMALISATION_KNEE, NORMALISATION_KNEE_SHORT, knee
                        );
                        println!(
                            "Valid `--{}` / `-{}` values: {} - {}",
                            NORMALISATION_KNEE,
                            NORMALISATION_KNEE_SHORT,
                            VALID_NORMALISATION_KNEE_RANGE.start(),
                            VALID_NORMALISATION_KNEE_RANGE.end()
                        );
                        println!("Default: {}", player_default_config.normalisation_knee);
                    };

                    let knee = knee.parse::<f64>().unwrap_or_else(|_| {
                        on_error();
                        exit(1);
                    });

                    if !(VALID_NORMALISATION_KNEE_RANGE).contains(&knee) {
                        on_error();
                        exit(1);
                    }

                    knee
                })
                .unwrap_or(player_default_config.normalisation_knee);
        }

        let ditherer_name = opt_str(DITHER);
        let ditherer = match ditherer_name.as_deref() {
            // explicitly disabled on command line
            Some("none") => None,
            // explicitly set on command line
            Some(_) => {
                if matches!(format, AudioFormat::F64 | AudioFormat::F32) {
                    error!("Dithering is not available with format: {:?}.", format);
                    exit(1);
                }

                Some(dither::find_ditherer(ditherer_name).unwrap_or_else(|| {
                    error!(
                        "Invalid `--{}` / `-{}`: {}",
                        DITHER,
                        DITHER_SHORT,
                        opt_str(DITHER).unwrap_or_default()
                    );
                    println!(
                        "Valid `--{}` / `-{}` values: none, gpdf, tpdf, tpdf_hp",
                        DITHER, DITHER_SHORT
                    );
                    println!(
                        "Default: tpdf for formats S16, S24, S24_3 and none for other formats"
                    );
                    exit(1);
                }))
            }
            // nothing set on command line => use default
            None => match format {
                AudioFormat::S16 | AudioFormat::S24 | AudioFormat::S24_3 => {
                    player_default_config.ditherer
                }
                _ => None,
            },
        };

        let passthrough = opt_present(PASSTHROUGH);

        PlayerConfig {
            bitrate,
            gapless,
            passthrough,
            normalisation,
            normalisation_type,
            normalisation_method,
            normalisation_pregain,
            normalisation_threshold,
            normalisation_attack,
            normalisation_release,
            normalisation_knee,
            ditherer,
        }
    };

    let player_event_program = opt_str(ONEVENT);
    let emit_sink_events = opt_present(EMIT_SINK_EVENTS);

    Setup {
        format,
        backend,
        device,
        mixer,
        cache,
        player_config,
        session_config,
        connect_config,
        mixer_config,
        credentials,
        enable_discovery,
        zeroconf_port,
        player_event_program,
        emit_sink_events,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    const RUST_BACKTRACE: &str = "RUST_BACKTRACE";
    if env::var(RUST_BACKTRACE).is_err() {
        env::set_var(RUST_BACKTRACE, "full")
    }

    let args: Vec<String> = std::env::args().collect();
    let setup = get_setup(&args);

    let mut last_credentials = None;
    let mut spirc: Option<Spirc> = None;
    let mut spirc_task: Option<Pin<_>> = None;
    let mut player_event_channel: Option<UnboundedReceiver<PlayerEvent>> = None;
    let mut auto_connect_times: Vec<Instant> = vec![];
    let mut discovery = None;
    let mut connecting: Pin<Box<dyn future::FusedFuture<Output = _>>> = Box::pin(future::pending());

    if setup.enable_discovery {
        let device_id = setup.session_config.device_id.clone();

        discovery = Some(
            librespot::discovery::Discovery::builder(device_id)
                .name(setup.connect_config.name.clone())
                .device_type(setup.connect_config.device_type)
                .port(setup.zeroconf_port)
                .launch()
                .unwrap(),
        );
    }

    if let Some(credentials) = setup.credentials {
        last_credentials = Some(credentials.clone());
        connecting = Box::pin(
            Session::connect(
                setup.session_config.clone(),
                credentials,
                setup.cache.clone(),
            )
            .fuse(),
        );
    }

    loop {
        tokio::select! {
            credentials = async { discovery.as_mut().unwrap().next().await }, if discovery.is_some() => {
                match credentials {
                    Some(credentials) => {
                        last_credentials = Some(credentials.clone());
                        auto_connect_times.clear();

                        if let Some(spirc) = spirc.take() {
                            spirc.shutdown();
                        }
                        if let Some(spirc_task) = spirc_task.take() {
                            // Continue shutdown in its own task
                            tokio::spawn(spirc_task);
                        }

                        connecting = Box::pin(Session::connect(
                            setup.session_config.clone(),
                            credentials,
                            setup.cache.clone(),
                        ).fuse());
                    },
                    None => {
                        warn!("Discovery stopped!");
                        discovery = None;
                    }
                }
            },
            session = &mut connecting, if !connecting.is_terminated() => match session {
                Ok(session) => {
                    let mixer_config = setup.mixer_config.clone();
                    let mixer = (setup.mixer)(mixer_config);
                    let player_config = setup.player_config.clone();
                    let connect_config = setup.connect_config.clone();

                    let audio_filter = mixer.get_audio_filter();
                    let format = setup.format;
                    let backend = setup.backend;
                    let device = setup.device.clone();
                    let (player, event_channel) =
                        Player::new(player_config, session.clone(), audio_filter, move || {
                            (backend)(device, format)
                        });

                    if setup.emit_sink_events {
                        if let Some(player_event_program) = setup.player_event_program.clone() {
                            player.set_sink_event_callback(Some(Box::new(move |sink_status| {
                                match emit_sink_event(sink_status, &player_event_program) {
                                    Ok(e) if e.success() => (),
                                    Ok(e) => {
                                        if let Some(code) = e.code() {
                                            warn!("Sink event program returned exit code {}", code);
                                        } else {
                                            warn!("Sink event program returned failure");
                                        }
                                    },
                                    Err(e) => {
                                        warn!("Emitting sink event failed: {}", e);
                                    },
                                }
                            })));
                        }
                    };

                    let (spirc_, spirc_task_) = Spirc::new(connect_config, session, player, mixer);

                    spirc = Some(spirc_);
                    spirc_task = Some(Box::pin(spirc_task_));
                    player_event_channel = Some(event_channel);
                },
                Err(e) => {
                    error!("Connection failed: {}", e);
                    exit(1);
                }
            },
            _ = async { spirc_task.as_mut().unwrap().await }, if spirc_task.is_some() => {
                spirc_task = None;

                warn!("Spirc shut down unexpectedly");
                while !auto_connect_times.is_empty()
                    && ((Instant::now() - auto_connect_times[0]).as_secs() > 600)
                {
                    let _ = auto_connect_times.remove(0);
                }

                if let Some(credentials) = last_credentials.clone() {
                    if auto_connect_times.len() >= 5 {
                        warn!("Spirc shut down too often. Not reconnecting automatically.");
                    } else {
                        auto_connect_times.push(Instant::now());

                        connecting = Box::pin(Session::connect(
                            setup.session_config.clone(),
                            credentials,
                            setup.cache.clone(),
                        ).fuse());
                    }
                }
            },
            event = async { player_event_channel.as_mut().unwrap().recv().await }, if player_event_channel.is_some() => match event {
                Some(event) => {
                    if let Some(program) = &setup.player_event_program {
                        if let Some(child) = run_program_on_events(event, program) {
                            if child.is_ok() {

                                let mut child = child.unwrap();

                                tokio::spawn(async move {
                                    match child.wait().await  {
                                        Ok(e) if e.success() => (),
                                        Ok(e) => {
                                            if let Some(code) = e.code() {
                                                warn!("On event program returned exit code {}", code);
                                            } else {
                                                warn!("On event program returned failure");
                                            }
                                        },
                                        Err(e) => {
                                            warn!("On event program failed: {}", e);
                                        },
                                    }
                                });
                            } else {
                                warn!("On event program failed to start");
                            }
                        }
                    }
                },
                None => {
                    player_event_channel = None;
                }
            },
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    info!("Gracefully shutting down");

    // Shutdown spirc if necessary
    if let Some(spirc) = spirc {
        spirc.shutdown();

        if let Some(mut spirc_task) = spirc_task {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => (),
                _ = spirc_task.as_mut() => ()
            }
        }
    }
}
