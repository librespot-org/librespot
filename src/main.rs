use std::{
    env,
    fs::create_dir_all,
    ops::RangeInclusive,
    path::{Path, PathBuf},
    pin::Pin,
    process::exit,
    str::FromStr,
    time::{Duration, Instant},
};

use futures_util::StreamExt;
use log::{error, info, trace, warn};
use sha1::{Digest, Sha1};
use thiserror::Error;
use url::Url;

use librespot::{
    connect::{config::ConnectConfig, spirc::Spirc},
    core::{
        authentication::Credentials, cache::Cache, config::DeviceType, version, Session,
        SessionConfig,
    },
    playback::{
        audio_backend::{self, SinkBuilder, BACKENDS},
        config::{
            AudioFormat, Bitrate, NormalisationMethod, NormalisationType, PlayerConfig, VolumeCtrl,
        },
        dither,
        mixer::{self, MixerConfig, MixerFn},
        player::{coefficient_to_duration, duration_to_coefficient, Player},
    },
};

#[cfg(feature = "alsa-backend")]
use librespot::playback::mixer::alsamixer::AlsaMixer;

mod player_event_handler;
use player_event_handler::{run_program_on_sink_events, EventHandler};

fn device_id(name: &str) -> String {
    hex::encode(Sha1::digest(name.as_bytes()))
}

fn usage(program: &str, opts: &getopts::Options) -> String {
    let repo_home = env!("CARGO_PKG_REPOSITORY");
    let desc = env!("CARGO_PKG_DESCRIPTION");
    let version = get_version_string();
    let brief = format!("{version}\n\n{desc}\n\n{repo_home}\n\nUsage: {program} [<Options>]");
    opts.usage(&brief)
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
            println!("- {name} (default)");
        } else {
            println!("- {name}");
        }
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
    zeroconf_ip: Vec<std::net::IpAddr>,
}

fn get_setup() -> Setup {
    const VALID_INITIAL_VOLUME_RANGE: RangeInclusive<u16> = 0..=100;
    const VALID_VOLUME_RANGE: RangeInclusive<f64> = 0.0..=100.0;
    const VALID_NORMALISATION_KNEE_RANGE: RangeInclusive<f64> = 0.0..=10.0;
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
    #[cfg(feature = "passthrough-decoder")]
    const PASSTHROUGH: &str = "passthrough";
    const PASSWORD: &str = "password";
    const PROXY: &str = "proxy";
    const QUIET: &str = "quiet";
    const SYSTEM_CACHE: &str = "system-cache";
    const TEMP_DIR: &str = "tmp";
    const USERNAME: &str = "username";
    const VERBOSE: &str = "verbose";
    const VERSION: &str = "version";
    const VOLUME_CTRL: &str = "volume-ctrl";
    const VOLUME_RANGE: &str = "volume-range";
    const ZEROCONF_PORT: &str = "zeroconf-port";
    const ZEROCONF_INTERFACE: &str = "zeroconf-interface";

    // Mostly arbitrary.
    const AP_PORT_SHORT: &str = "a";
    const AUTOPLAY_SHORT: &str = "A";
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
    const ZEROCONF_INTERFACE_SHORT: &str = "i";
    const CACHE_SIZE_LIMIT_SHORT: &str = "M";
    const MIXER_TYPE_SHORT: &str = "m";
    const ENABLE_VOLUME_NORMALISATION_SHORT: &str = "N";
    const NAME_SHORT: &str = "n";
    const DISABLE_DISCOVERY_SHORT: &str = "O";
    const ONEVENT_SHORT: &str = "o";
    #[cfg(feature = "passthrough-decoder")]
    const PASSTHROUGH_SHORT: &str = "P";
    const PASSWORD_SHORT: &str = "p";
    const EMIT_SINK_EVENTS_SHORT: &str = "Q";
    const QUIET_SHORT: &str = "q";
    const INITIAL_VOLUME_SHORT: &str = "R";
    const ALSA_MIXER_DEVICE_SHORT: &str = "S";
    const ALSA_MIXER_INDEX_SHORT: &str = "s";
    const ALSA_MIXER_CONTROL_SHORT: &str = "T";
    const TEMP_DIR_SHORT: &str = "t";
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

    // Options that have different descriptions
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
        TEMP_DIR_SHORT,
        TEMP_DIR,
        "Path to a directory where files will be temporarily stored while downloading.",
        "PATH",
    )
    .optopt(
        CACHE_SHORT,
        CACHE,
        "Path to a directory where files will be cached after downloading.",
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
        "Knee width (dB) of the dynamic limiter from 0.0 to 10.0. Defaults to 5.0.",
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
        "Connect to an AP with a specified port 1 - 65535. Available ports are usually 80, 443 and 4070.",
        "PORT",
    )
    .optopt(
        AUTOPLAY_SHORT,
        AUTOPLAY,
        "Explicitly set autoplay {on|off}. Defaults to following the client setting.",
        "OVERRIDE",
    )
    .optopt(
        ZEROCONF_INTERFACE_SHORT,
        ZEROCONF_INTERFACE,
        "Comma-separated interface IP addresses on which zeroconf will bind. Defaults to all interfaces. Ignored by DNS-SD.",
        "IP"
    );

    #[cfg(feature = "passthrough-decoder")]
    opts.optflag(
        PASSTHROUGH_SHORT,
        PASSTHROUGH,
        "Pass a raw stream to the output. Only works with the pipe and subprocess backends.",
    );

    let args: Vec<_> = std::env::args_os()
        .filter_map(|s| match s.into_string() {
            Ok(valid) => Some(valid),
            Err(s) => {
                eprintln!(
                    "Command line argument was not valid Unicode and will not be evaluated: {s:?}"
                );
                None
            }
        })
        .collect();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error parsing command line options: {e}");
            println!("\n{}", usage(&args[0], &opts));
            exit(1);
        }
    };

    let stripped_env_key = |k: &str| {
        k.trim_start_matches("LIBRESPOT_")
            .replace('_', "-")
            .to_lowercase()
    };

    let env_vars: Vec<_> = env::vars_os().filter_map(|(k, v)| match k.into_string() {
        Ok(key) if key.starts_with("LIBRESPOT_") => {
            let stripped_key = stripped_env_key(&key);
            // We only care about long option/flag names.
            if stripped_key.chars().count() > 1 && matches.opt_defined(&stripped_key) {
                match v.into_string() {
                    Ok(value) => Some((key, value)),
                    Err(s) => {
                        eprintln!("Environment variable was not valid Unicode and will not be evaluated: {key}={s:?}");
                        None
                    }
                }
            } else {
                None
            }
        },
        _ => None
    })
    .collect();

    let opt_present =
        |opt| matches.opt_present(opt) || env_vars.iter().any(|(k, _)| stripped_env_key(k) == opt);

    let opt_str = |opt| {
        if matches.opt_present(opt) {
            matches.opt_str(opt)
        } else {
            env_vars
                .iter()
                .find(|(k, _)| stripped_env_key(k) == opt)
                .map(|(_, v)| v.to_string())
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

    if !env_vars.is_empty() {
        trace!("Environment variable(s):");

        for (k, v) in &env_vars {
            if matches!(k.as_str(), "LIBRESPOT_PASSWORD" | "LIBRESPOT_USERNAME") {
                trace!("\t\t{k}=\"XXXXXXXX\"");
            } else if v.is_empty() {
                trace!("\t\t{k}=");
            } else {
                trace!("\t\t{k}=\"{v}\"");
            }
        }
    }

    let args_len = args.len();

    if args_len > 1 {
        trace!("Command line argument(s):");

        for (index, key) in args.iter().enumerate() {
            let opt = {
                let key = key.trim_start_matches('-');

                if let Some((s, _)) = key.split_once('=') {
                    s
                } else {
                    key
                }
            };

            if index > 0
                && key.starts_with('-')
                && &args[index - 1] != key
                && matches.opt_defined(opt)
                && matches.opt_present(opt)
            {
                if matches!(opt, PASSWORD | PASSWORD_SHORT | USERNAME | USERNAME_SHORT) {
                    // Don't log creds.
                    trace!("\t\t{opt} \"XXXXXXXX\"");
                } else {
                    let value = matches.opt_str(opt).unwrap_or_default();
                    if value.is_empty() {
                        trace!("\t\t{opt}");
                    } else {
                        trace!("\t\t{opt} \"{value}\"");
                    }
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

    let invalid_error_msg =
        |long: &str, short: &str, invalid: &str, valid_values: &str, default_value: &str| {
            error!("Invalid `--{long}` / `-{short}`: \"{invalid}\"");

            if !valid_values.is_empty() {
                println!("Valid `--{long}` / `-{short}` values: {valid_values}");
            }

            if !default_value.is_empty() {
                println!("Default: {default_value}");
            }
        };

    let empty_string_error_msg = |long: &str, short: &str| {
        error!("`--{long}` / `-{short}` can not be an empty string");
        exit(1);
    };

    let backend = audio_backend::find(backend_name).unwrap_or_else(|| {
        invalid_error_msg(
            BACKEND,
            BACKEND_SHORT,
            &opt_str(BACKEND).unwrap_or_default(),
            "",
            "",
        );

        list_backends();
        exit(1);
    });

    let format = opt_str(FORMAT)
        .as_deref()
        .map(|format| {
            AudioFormat::from_str(format).unwrap_or_else(|_| {
                let default_value = &format!("{:?}", AudioFormat::default());
                invalid_error_msg(
                    FORMAT,
                    FORMAT_SHORT,
                    format,
                    "F64, F32, S32, S24, S24_3, S16",
                    default_value,
                );

                exit(1);
            })
        })
        .unwrap_or_default();

    let device = opt_str(DEVICE);
    if let Some(ref value) = device {
        if value == "?" {
            backend(device, format);
            exit(0);
        } else if value.is_empty() {
            empty_string_error_msg(DEVICE, DEVICE_SHORT);
        }
    }

    #[cfg(feature = "alsa-backend")]
    let mixer_type = opt_str(MIXER_TYPE);
    #[cfg(not(feature = "alsa-backend"))]
    let mixer_type: Option<String> = None;

    let mixer = mixer::find(mixer_type.as_deref()).unwrap_or_else(|| {
        invalid_error_msg(
            MIXER_TYPE,
            MIXER_TYPE_SHORT,
            &opt_str(MIXER_TYPE).unwrap_or_default(),
            "alsa, softvol",
            "softvol",
        );

        exit(1);
    });

    let is_alsa_mixer = match mixer_type.as_deref() {
        #[cfg(feature = "alsa-backend")]
        Some(AlsaMixer::NAME) => true,
        _ => false,
    };

    #[cfg(feature = "alsa-backend")]
    if !is_alsa_mixer {
        for a in &[ALSA_MIXER_DEVICE, ALSA_MIXER_INDEX, ALSA_MIXER_CONTROL] {
            if opt_present(a) {
                warn!("Alsa specific mixer options have no effect if not using the alsa mixer.");
                break;
            }
        }
    }

    let mixer_config = {
        let mixer_default_config = MixerConfig::default();

        #[cfg(feature = "alsa-backend")]
        let index = if !is_alsa_mixer {
            mixer_default_config.index
        } else {
            opt_str(ALSA_MIXER_INDEX)
                .map(|index| {
                    index.parse::<u32>().unwrap_or_else(|_| {
                        invalid_error_msg(
                            ALSA_MIXER_INDEX,
                            ALSA_MIXER_INDEX_SHORT,
                            &index,
                            "",
                            &mixer_default_config.index.to_string(),
                        );

                        exit(1);
                    })
                })
                .unwrap_or_else(|| match device {
                    // Look for the dev index portion of --device.
                    // Specifically <dev index> when --device is <something>:CARD=<card name>,DEV=<dev index>
                    // or <something>:<card index>,<dev index>.

                    // If --device does not contain a ',' it does not contain a dev index.
                    // In the case that the dev index is omitted it is assumed to be 0 (mixer_default_config.index).
                    // Malformed --device values will also fallback to mixer_default_config.index.
                    Some(ref device_name) if device_name.contains(',') => {
                        // Turn <something>:CARD=<card name>,DEV=<dev index> or <something>:<card index>,<dev index>
                        // into DEV=<dev index> or <dev index>.
                        let dev = &device_name[device_name.find(',').unwrap_or_default()..]
                            .trim_start_matches(',');

                        // Turn DEV=<dev index> into <dev index> (noop if it's already <dev index>)
                        // and then parse <dev index>.
                        // Malformed --device values will fail the parse and fallback to mixer_default_config.index.
                        dev[dev.find('=').unwrap_or_default()..]
                            .trim_start_matches('=')
                            .parse::<u32>()
                            .unwrap_or(mixer_default_config.index)
                    }
                    _ => mixer_default_config.index,
                })
        };

        #[cfg(not(feature = "alsa-backend"))]
        let index = mixer_default_config.index;

        #[cfg(feature = "alsa-backend")]
        let device = if !is_alsa_mixer {
            mixer_default_config.device
        } else {
            match opt_str(ALSA_MIXER_DEVICE) {
                Some(mixer_device) => {
                    if mixer_device.is_empty() {
                        empty_string_error_msg(ALSA_MIXER_DEVICE, ALSA_MIXER_DEVICE_SHORT);
                    }

                    mixer_device
                }
                None => match device {
                    Some(ref device_name) => {
                        // Look for the card name or card index portion of --device.
                        // Specifically <card name> when --device is <something>:CARD=<card name>,DEV=<dev index>
                        // or card index when --device is <something>:<card index>,<dev index>.
                        // --device values like `pulse`, `default`, `jack` may be valid but there is no way to
                        // infer automatically what the mixer should be so they fail auto fallback
                        // so --alsa-mixer-device must be manually specified in those situations.
                        let start_index = device_name.find(':').unwrap_or_default();

                        let end_index = match device_name.find(',') {
                            Some(index) if index > start_index => index,
                            _ => device_name.len(),
                        };

                        let card = &device_name[start_index..end_index];

                        if card.starts_with(':') {
                            // mixers are assumed to be hw:CARD=<card name> or hw:<card index>.
                            "hw".to_owned() + card
                        } else {
                            error!(
                                "Could not find an alsa mixer for \"{}\", it must be specified with `--{}` / `-{}`",
                                &device.unwrap_or_default(),
                                ALSA_MIXER_DEVICE,
                                ALSA_MIXER_DEVICE_SHORT
                            );

                            exit(1);
                        }
                    }
                    None => {
                        error!(
                            "`--{}` / `-{}` or `--{}` / `-{}` \
                            must be specified when `--{}` / `-{}` is set to \"alsa\"",
                            DEVICE,
                            DEVICE_SHORT,
                            ALSA_MIXER_DEVICE,
                            ALSA_MIXER_DEVICE_SHORT,
                            MIXER_TYPE,
                            MIXER_TYPE_SHORT
                        );

                        exit(1);
                    }
                },
            }
        };

        #[cfg(not(feature = "alsa-backend"))]
        let device = mixer_default_config.device;

        #[cfg(feature = "alsa-backend")]
        let control = opt_str(ALSA_MIXER_CONTROL).unwrap_or(mixer_default_config.control);

        #[cfg(feature = "alsa-backend")]
        if control.is_empty() {
            empty_string_error_msg(ALSA_MIXER_CONTROL, ALSA_MIXER_CONTROL_SHORT);
        }

        #[cfg(not(feature = "alsa-backend"))]
        let control = mixer_default_config.control;

        let volume_range = opt_str(VOLUME_RANGE)
            .map(|range| match range.parse::<f64>() {
                Ok(value) if (VALID_VOLUME_RANGE).contains(&value) => value,
                _ => {
                    let valid_values = &format!(
                        "{} - {}",
                        VALID_VOLUME_RANGE.start(),
                        VALID_VOLUME_RANGE.end()
                    );

                    #[cfg(feature = "alsa-backend")]
                    let default_value = &format!(
                        "softvol - {}, alsa - what the control supports",
                        VolumeCtrl::DEFAULT_DB_RANGE
                    );

                    #[cfg(not(feature = "alsa-backend"))]
                    let default_value = &VolumeCtrl::DEFAULT_DB_RANGE.to_string();

                    invalid_error_msg(
                        VOLUME_RANGE,
                        VOLUME_RANGE_SHORT,
                        &range,
                        valid_values,
                        default_value,
                    );

                    exit(1);
                }
            })
            .unwrap_or_else(|| {
                if is_alsa_mixer {
                    0.0
                } else {
                    VolumeCtrl::DEFAULT_DB_RANGE
                }
            });

        let volume_ctrl = opt_str(VOLUME_CTRL)
            .as_deref()
            .map(|volume_ctrl| {
                VolumeCtrl::from_str_with_range(volume_ctrl, volume_range).unwrap_or_else(|_| {
                    invalid_error_msg(
                        VOLUME_CTRL,
                        VOLUME_CTRL_SHORT,
                        volume_ctrl,
                        "cubic, fixed, linear, log",
                        "log",
                    );

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

    let tmp_dir = opt_str(TEMP_DIR).map_or(SessionConfig::default().tmp_dir, |p| {
        let tmp_dir = PathBuf::from(p);
        if let Err(e) = create_dir_all(&tmp_dir) {
            error!("could not create or access specified tmp directory: {}", e);
            exit(1);
        }
        tmp_dir
    });

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
                        invalid_error_msg(
                            CACHE_SIZE_LIMIT,
                            CACHE_SIZE_LIMIT_SHORT,
                            &e.to_string(),
                            "",
                            "",
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
        let cached_creds = cache.as_ref().and_then(Cache::credentials);

        if let Some(username) = opt_str(USERNAME) {
            if username.is_empty() {
                empty_string_error_msg(USERNAME, USERNAME_SHORT);
            }
            if let Some(password) = opt_str(PASSWORD) {
                if password.is_empty() {
                    empty_string_error_msg(PASSWORD, PASSWORD_SHORT);
                }
                Some(Credentials::with_password(username, password))
            } else {
                match cached_creds {
                    Some(creds) if username == creds.username => Some(creds),
                    _ => {
                        let prompt = &format!("Password for {username}: ");
                        match rpassword::prompt_password(prompt) {
                            Ok(password) => {
                                if !password.is_empty() {
                                    Some(Credentials::with_password(username, password))
                                } else {
                                    trace!("Password was empty.");
                                    if cached_creds.is_some() {
                                        trace!("Using cached credentials.");
                                    }
                                    cached_creds
                                }
                            }
                            Err(e) => {
                                warn!("Cannot parse password: {}", e);
                                if cached_creds.is_some() {
                                    trace!("Using cached credentials.");
                                }
                                cached_creds
                            }
                        }
                    }
                }
            }
        } else {
            if cached_creds.is_some() {
                trace!("Using cached credentials.");
            }
            cached_creds
        }
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
            .map(|port| match port.parse::<u16>() {
                Ok(value) if value != 0 => value,
                _ => {
                    let valid_values = &format!("1 - {}", u16::MAX);
                    invalid_error_msg(ZEROCONF_PORT, ZEROCONF_PORT_SHORT, &port, valid_values, "");

                    exit(1);
                }
            })
            .unwrap_or(0)
    } else {
        0
    };

    // #1046: not all connections are supplied an `autoplay` user attribute to run statelessly.
    // This knob allows for a manual override.
    let autoplay = match opt_str(AUTOPLAY) {
        Some(value) => match value.as_ref() {
            "on" => Some(true),
            "off" => Some(false),
            _ => {
                invalid_error_msg(
                    AUTOPLAY,
                    AUTOPLAY_SHORT,
                    &opt_str(AUTOPLAY).unwrap_or_default(),
                    "on, off",
                    "",
                );
                exit(1);
            }
        },
        None => SessionConfig::default().autoplay,
    };

    let zeroconf_ip: Vec<std::net::IpAddr> = if opt_present(ZEROCONF_INTERFACE) {
        if let Some(zeroconf_ip) = opt_str(ZEROCONF_INTERFACE) {
            zeroconf_ip
                .split(',')
                .map(|s| {
                    s.trim().parse::<std::net::IpAddr>().unwrap_or_else(|_| {
                        invalid_error_msg(
                            ZEROCONF_INTERFACE,
                            ZEROCONF_INTERFACE_SHORT,
                            s,
                            "IPv4 and IPv6 addresses",
                            "",
                        );
                        exit(1);
                    })
                })
                .collect()
        } else {
            warn!("Unable to use zeroconf-interface option, default to all interfaces.");
            vec![]
        }
    } else {
        vec![]
    };

    let connect_config = {
        let connect_default_config = ConnectConfig::default();

        let name = opt_str(NAME).unwrap_or_else(|| connect_default_config.name.clone());

        if name.is_empty() {
            empty_string_error_msg(NAME, NAME_SHORT);
            exit(1);
        }

        #[cfg(feature = "pulseaudio-backend")]
        {
            if env::var("PULSE_PROP_application.name").is_err() {
                let pulseaudio_name = if name != connect_default_config.name {
                    format!("{} - {}", connect_default_config.name, name)
                } else {
                    name.clone()
                };

                env::set_var("PULSE_PROP_application.name", pulseaudio_name);
            }

            if env::var("PULSE_PROP_application.version").is_err() {
                env::set_var("PULSE_PROP_application.version", version::SEMVER);
            }

            if env::var("PULSE_PROP_application.icon_name").is_err() {
                env::set_var("PULSE_PROP_application.icon_name", "audio-x-generic");
            }

            if env::var("PULSE_PROP_application.process.binary").is_err() {
                env::set_var("PULSE_PROP_application.process.binary", "librespot");
            }

            if env::var("PULSE_PROP_stream.description").is_err() {
                env::set_var("PULSE_PROP_stream.description", "Spotify Connect endpoint");
            }

            if env::var("PULSE_PROP_media.software").is_err() {
                env::set_var("PULSE_PROP_media.software", "Spotify");
            }

            if env::var("PULSE_PROP_media.role").is_err() {
                env::set_var("PULSE_PROP_media.role", "music");
            }
        }

        let initial_volume = opt_str(INITIAL_VOLUME)
            .map(|initial_volume| {
                let volume = match initial_volume.parse::<u16>() {
                    Ok(value) if (VALID_INITIAL_VOLUME_RANGE).contains(&value) => value,
                    _ => {
                        let valid_values = &format!(
                            "{} - {}",
                            VALID_INITIAL_VOLUME_RANGE.start(),
                            VALID_INITIAL_VOLUME_RANGE.end()
                        );

                        #[cfg(feature = "alsa-backend")]
                        let default_value = &format!(
                            "{}, or the current value when the alsa mixer is used.",
                            connect_default_config.initial_volume.unwrap_or_default()
                        );

                        #[cfg(not(feature = "alsa-backend"))]
                        let default_value = &connect_default_config
                            .initial_volume
                            .unwrap_or_default()
                            .to_string();

                        invalid_error_msg(
                            INITIAL_VOLUME,
                            INITIAL_VOLUME_SHORT,
                            &initial_volume,
                            valid_values,
                            default_value,
                        );

                        exit(1);
                    }
                };

                (volume as f32 / 100.0 * VolumeCtrl::MAX_VOLUME as f32) as u16
            })
            .or_else(|| {
                if is_alsa_mixer {
                    None
                } else {
                    cache.as_ref().and_then(Cache::volume)
                }
            });

        let device_type = opt_str(DEVICE_TYPE)
            .as_deref()
            .map(|device_type| {
                DeviceType::from_str(device_type).unwrap_or_else(|_| {
                    invalid_error_msg(
                        DEVICE_TYPE,
                        DEVICE_TYPE_SHORT,
                        device_type,
                        "computer, tablet, smartphone, \
                        speaker, tv, avr, stb, audiodongle, \
                        gameconsole, castaudio, castvideo, \
                        automobile, smartwatch, chromebook, \
                        carthing, homething",
                        DeviceType::default().into(),
                    );

                    exit(1);
                })
            })
            .unwrap_or_default();

        let has_volume_ctrl = !matches!(mixer_config.volume_ctrl, VolumeCtrl::Fixed);

        ConnectConfig {
            name,
            device_type,
            initial_volume,
            has_volume_ctrl,
        }
    };

    let session_config = SessionConfig {
        device_id: device_id(&connect_config.name),
        proxy: opt_str(PROXY).or_else(|| std::env::var("http_proxy").ok()).map(
            |s| {
                match Url::parse(&s) {
                    Ok(url) => {
                        if url.host().is_none() || url.port_or_known_default().is_none() {
                            error!("Invalid proxy url, only URLs on the format \"http(s)://host:port\" are allowed");
                            exit(1);
                        }

                        url
                    },
                    Err(e) => {
                        error!("Invalid proxy URL: \"{}\", only URLs in the format \"http(s)://host:port\" are allowed", e);
                        exit(1);
                    }
                }
            },
        ),
        ap_port: opt_str(AP_PORT).map(|port| match port.parse::<u16>() {
            Ok(value) if value != 0 => value,
            _ => {
                let valid_values = &format!("1 - {}", u16::MAX);
                invalid_error_msg(AP_PORT, AP_PORT_SHORT, &port, valid_values, "");

                exit(1);
            }
        }),
		tmp_dir,
		autoplay,
		..SessionConfig::default()
    };

    let player_config = {
        let player_default_config = PlayerConfig::default();

        let bitrate = opt_str(BITRATE)
            .as_deref()
            .map(|bitrate| {
                Bitrate::from_str(bitrate).unwrap_or_else(|_| {
                    invalid_error_msg(BITRATE, BITRATE_SHORT, bitrate, "96, 160, 320", "160");
                    exit(1);
                })
            })
            .unwrap_or(player_default_config.bitrate);

        let gapless = !opt_present(DISABLE_GAPLESS);

        let normalisation = opt_present(ENABLE_VOLUME_NORMALISATION);

        let normalisation_method;
        let normalisation_type;
        let normalisation_pregain_db;
        let normalisation_threshold_dbfs;
        let normalisation_attack_cf;
        let normalisation_release_cf;
        let normalisation_knee_db;

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
            normalisation_pregain_db = player_default_config.normalisation_pregain_db;
            normalisation_threshold_dbfs = player_default_config.normalisation_threshold_dbfs;
            normalisation_attack_cf = player_default_config.normalisation_attack_cf;
            normalisation_release_cf = player_default_config.normalisation_release_cf;
            normalisation_knee_db = player_default_config.normalisation_knee_db;
        } else {
            normalisation_method = opt_str(NORMALISATION_METHOD)
                .as_deref()
                .map(|method| {
                    NormalisationMethod::from_str(method).unwrap_or_else(|_| {
                        invalid_error_msg(
                            NORMALISATION_METHOD,
                            NORMALISATION_METHOD_SHORT,
                            method,
                            "basic, dynamic",
                            &format!("{:?}", player_default_config.normalisation_method),
                        );

                        exit(1);
                    })
                })
                .unwrap_or(player_default_config.normalisation_method);

            normalisation_type = opt_str(NORMALISATION_GAIN_TYPE)
                .as_deref()
                .map(|gain_type| {
                    NormalisationType::from_str(gain_type).unwrap_or_else(|_| {
                        invalid_error_msg(
                            NORMALISATION_GAIN_TYPE,
                            NORMALISATION_GAIN_TYPE_SHORT,
                            gain_type,
                            "track, album, auto",
                            &format!("{:?}", player_default_config.normalisation_type),
                        );

                        exit(1);
                    })
                })
                .unwrap_or(player_default_config.normalisation_type);

            normalisation_pregain_db = opt_str(NORMALISATION_PREGAIN)
                .map(|pregain| match pregain.parse::<f64>() {
                    Ok(value) if (VALID_NORMALISATION_PREGAIN_RANGE).contains(&value) => value,
                    _ => {
                        let valid_values = &format!(
                            "{} - {}",
                            VALID_NORMALISATION_PREGAIN_RANGE.start(),
                            VALID_NORMALISATION_PREGAIN_RANGE.end()
                        );

                        invalid_error_msg(
                            NORMALISATION_PREGAIN,
                            NORMALISATION_PREGAIN_SHORT,
                            &pregain,
                            valid_values,
                            &player_default_config.normalisation_pregain_db.to_string(),
                        );

                        exit(1);
                    }
                })
                .unwrap_or(player_default_config.normalisation_pregain_db);

            normalisation_threshold_dbfs = opt_str(NORMALISATION_THRESHOLD)
                .map(|threshold| match threshold.parse::<f64>() {
                    Ok(value) if (VALID_NORMALISATION_THRESHOLD_RANGE).contains(&value) => value,
                    _ => {
                        let valid_values = &format!(
                            "{} - {}",
                            VALID_NORMALISATION_THRESHOLD_RANGE.start(),
                            VALID_NORMALISATION_THRESHOLD_RANGE.end()
                        );

                        invalid_error_msg(
                            NORMALISATION_THRESHOLD,
                            NORMALISATION_THRESHOLD_SHORT,
                            &threshold,
                            valid_values,
                            &player_default_config
                                .normalisation_threshold_dbfs
                                .to_string(),
                        );

                        exit(1);
                    }
                })
                .unwrap_or(player_default_config.normalisation_threshold_dbfs);

            normalisation_attack_cf = opt_str(NORMALISATION_ATTACK)
                .map(|attack| match attack.parse::<u64>() {
                    Ok(value) if (VALID_NORMALISATION_ATTACK_RANGE).contains(&value) => {
                        duration_to_coefficient(Duration::from_millis(value))
                    }
                    _ => {
                        let valid_values = &format!(
                            "{} - {}",
                            VALID_NORMALISATION_ATTACK_RANGE.start(),
                            VALID_NORMALISATION_ATTACK_RANGE.end()
                        );

                        invalid_error_msg(
                            NORMALISATION_ATTACK,
                            NORMALISATION_ATTACK_SHORT,
                            &attack,
                            valid_values,
                            &coefficient_to_duration(player_default_config.normalisation_attack_cf)
                                .as_millis()
                                .to_string(),
                        );

                        exit(1);
                    }
                })
                .unwrap_or(player_default_config.normalisation_attack_cf);

            normalisation_release_cf = opt_str(NORMALISATION_RELEASE)
                .map(|release| match release.parse::<u64>() {
                    Ok(value) if (VALID_NORMALISATION_RELEASE_RANGE).contains(&value) => {
                        duration_to_coefficient(Duration::from_millis(value))
                    }
                    _ => {
                        let valid_values = &format!(
                            "{} - {}",
                            VALID_NORMALISATION_RELEASE_RANGE.start(),
                            VALID_NORMALISATION_RELEASE_RANGE.end()
                        );

                        invalid_error_msg(
                            NORMALISATION_RELEASE,
                            NORMALISATION_RELEASE_SHORT,
                            &release,
                            valid_values,
                            &coefficient_to_duration(
                                player_default_config.normalisation_release_cf,
                            )
                            .as_millis()
                            .to_string(),
                        );

                        exit(1);
                    }
                })
                .unwrap_or(player_default_config.normalisation_release_cf);

            normalisation_knee_db = opt_str(NORMALISATION_KNEE)
                .map(|knee| match knee.parse::<f64>() {
                    Ok(value) if (VALID_NORMALISATION_KNEE_RANGE).contains(&value) => value,
                    _ => {
                        let valid_values = &format!(
                            "{} - {}",
                            VALID_NORMALISATION_KNEE_RANGE.start(),
                            VALID_NORMALISATION_KNEE_RANGE.end()
                        );

                        invalid_error_msg(
                            NORMALISATION_KNEE,
                            NORMALISATION_KNEE_SHORT,
                            &knee,
                            valid_values,
                            &player_default_config.normalisation_knee_db.to_string(),
                        );

                        exit(1);
                    }
                })
                .unwrap_or(player_default_config.normalisation_knee_db);
        }

        let ditherer_name = opt_str(DITHER);
        let ditherer = match ditherer_name.as_deref() {
            Some(value) => match value {
                "none" => None,
                _ => match format {
                    AudioFormat::F64 | AudioFormat::F32 => {
                        error!("Dithering is not available with format: {:?}.", format);
                        exit(1);
                    }
                    _ => Some(dither::find_ditherer(ditherer_name).unwrap_or_else(|| {
                        invalid_error_msg(
                            DITHER,
                            DITHER_SHORT,
                            &opt_str(DITHER).unwrap_or_default(),
                            "none, gpdf, tpdf, tpdf_hp for formats S16, S24, S24_3, S32, none for formats F32, F64",
                            "tpdf for formats S16, S24, S24_3 and none for formats S32, F32, F64",
                        );

                        exit(1);
                    })),
                },
            },
            None => match format {
                AudioFormat::S16 | AudioFormat::S24 | AudioFormat::S24_3 => {
                    player_default_config.ditherer
                }
                _ => None,
            },
        };

        #[cfg(feature = "passthrough-decoder")]
        let passthrough = opt_present(PASSTHROUGH);
        #[cfg(not(feature = "passthrough-decoder"))]
        let passthrough = false;

        PlayerConfig {
            bitrate,
            gapless,
            passthrough,
            normalisation,
            normalisation_type,
            normalisation_method,
            normalisation_pregain_db,
            normalisation_threshold_dbfs,
            normalisation_attack_cf,
            normalisation_release_cf,
            normalisation_knee_db,
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
        zeroconf_ip,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    const RUST_BACKTRACE: &str = "RUST_BACKTRACE";
    const RECONNECT_RATE_LIMIT_WINDOW: Duration = Duration::from_secs(600);
    const RECONNECT_RATE_LIMIT: usize = 5;

    if env::var(RUST_BACKTRACE).is_err() {
        env::set_var(RUST_BACKTRACE, "full")
    }

    let setup = get_setup();

    let mut last_credentials = None;
    let mut spirc: Option<Spirc> = None;
    let mut spirc_task: Option<Pin<_>> = None;
    let mut auto_connect_times: Vec<Instant> = vec![];
    let mut discovery = None;
    let mut connecting = false;
    let mut _event_handler: Option<EventHandler> = None;

    let mut session = Session::new(setup.session_config.clone(), setup.cache.clone());

    if setup.enable_discovery {
        let device_id = setup.session_config.device_id.clone();
        let client_id = setup.session_config.client_id.clone();
        match librespot::discovery::Discovery::builder(device_id, client_id)
            .name(setup.connect_config.name.clone())
            .device_type(setup.connect_config.device_type)
            .port(setup.zeroconf_port)
            .zeroconf_ip(setup.zeroconf_ip)
            .launch()
        {
            Ok(d) => discovery = Some(d),
            Err(err) => warn!("Could not initialise discovery: {}.", err),
        };
    }

    if let Some(credentials) = setup.credentials {
        last_credentials = Some(credentials);
        connecting = true;
    } else if discovery.is_none() {
        error!(
            "Discovery is unavailable and no credentials provided. Authentication is not possible."
        );
        exit(1);
    }

    loop {
        tokio::select! {
            credentials = async {
                match discovery.as_mut() {
                    Some(d) => d.next().await,
                    _ => None
                }
            }, if discovery.is_some() => {
                match credentials {
                    Some(credentials) => {
                        last_credentials = Some(credentials.clone());
                        auto_connect_times.clear();

                        if let Some(spirc) = spirc.take() {
                            if let Err(e) = spirc.shutdown() {
                                error!("error sending spirc shutdown message: {}", e);
                            }
                        }
                        if let Some(spirc_task) = spirc_task.take() {
                            // Continue shutdown in its own task
                            tokio::spawn(spirc_task);
                        }

                        connecting = true;
                    },
                    None => {
                        error!("Discovery stopped unexpectedly");
                        exit(1);
                    }
                }
            },
            _ = async {}, if connecting && last_credentials.is_some() => {
                if session.is_invalid() {
                    session = Session::new(setup.session_config.clone(), setup.cache.clone());
                }

                let mixer_config = setup.mixer_config.clone();
                let mixer = (setup.mixer)(mixer_config);
                let player_config = setup.player_config.clone();
                let connect_config = setup.connect_config.clone();

                let soft_volume = mixer.get_soft_volume();
                let format = setup.format;
                let backend = setup.backend;
                let device = setup.device.clone();
                let player = Player::new(player_config, session.clone(), soft_volume, move || {
                    (backend)(device, format)
                });

                if let Some(player_event_program) = setup.player_event_program.clone() {
                    _event_handler = Some(EventHandler::new(player.get_player_event_channel(), &player_event_program));

                    if setup.emit_sink_events {
                        player.set_sink_event_callback(Some(Box::new(move |sink_status| {
                            run_program_on_sink_events(sink_status, &player_event_program)
                        })));
                    }
                };

                let (spirc_, spirc_task_) = match Spirc::new(connect_config, session.clone(), last_credentials.clone().unwrap_or_default(), player, mixer).await {
                    Ok((spirc_, spirc_task_)) => (spirc_, spirc_task_),
                    Err(e) => {
                        error!("could not initialize spirc: {}", e);
                        exit(1);
                    }
                };
                spirc = Some(spirc_);
                spirc_task = Some(Box::pin(spirc_task_));

                connecting = false;
            },
            _ = async {
                if let Some(task) = spirc_task.as_mut() {
                    task.await;
                }
            }, if spirc_task.is_some() && !connecting => {
                spirc_task = None;

                warn!("Spirc shut down unexpectedly");

                let mut reconnect_exceeds_rate_limit = || {
                    auto_connect_times.retain(|&t| t.elapsed() < RECONNECT_RATE_LIMIT_WINDOW);
                    auto_connect_times.len() > RECONNECT_RATE_LIMIT
                };

                if last_credentials.is_some() && !reconnect_exceeds_rate_limit() {
                    auto_connect_times.push(Instant::now());
                    connecting = true;
                } else {
                    error!("Spirc shut down too often. Not reconnecting automatically.");
                    exit(1);
                }
            },
            _ = tokio::signal::ctrl_c() => {
                break;
            },
            else => break,
        }
    }

    info!("Gracefully shutting down");

    // Shutdown spirc if necessary
    if let Some(spirc) = spirc {
        if let Err(e) = spirc.shutdown() {
            error!("error sending spirc shutdown message: {}", e);
        }

        if let Some(mut spirc_task) = spirc_task {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => (),
                _ = spirc_task.as_mut() => (),
                else => (),
            }
        }
    }
}
