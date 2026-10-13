/// Version string of the form "librespot-\<sha\>"
pub const VERSION_STRING: &str = concat!("librespot-", env!("VERGEN_GIT_SHA"));

/// Generate a timestamp string representing the build date (UTC).
pub const BUILD_DATE: &str = env!("VERGEN_BUILD_DATE");

/// Short sha of the latest git commit.
pub const SHA_SHORT: &str = env!("VERGEN_GIT_SHA");

/// Date of the latest git commit.
pub const COMMIT_DATE: &str = env!("VERGEN_GIT_COMMIT_DATE");

/// Librespot crate version.
pub const SEMVER: &str = env!("CARGO_PKG_VERSION");

/// A random build id.
pub const BUILD_ID: &str = env!("LIBRESPOT_BUILD_ID");

/// The protocol version of the Spotify desktop client.
pub const SPOTIFY_VERSION: u64 = 129600518;

/// The semantic version of the Spotify desktop client.
pub const SPOTIFY_SEMANTIC_VERSION: &str = "1.2.96.518.g366879e1";

/// `property_set_id` related to desktop version 1.2.96.518
pub const SPOTIFY_PROPERTY_SET_ID: &str = "0ac90111a046060de984f4395a787adaff7e6539";

/// The protocol version of the Spotify mobile app.
pub const SPOTIFY_MOBILE_VERSION: &str = "8.9.82.620";

/// `property_set_id` related to mobile version 8.9.82.620
pub const SPOTIFY_MOBILE_PROPERTY_SET_ID: &str =
    "5ec87c2cc32e7c509703582cfaaa3c7ad253129d5701127c1f5eab5c9531736c";

/// The general spirc version
pub const SPOTIFY_SPIRC_VERSION: &str = "3.2.6";

/// The user agent to fall back to, if one could not be determined dynamically.
pub const FALLBACK_USER_AGENT: &str = "Spotify/129600518 Linux/0 (librespot)";

pub fn spotify_version() -> String {
    match crate::config::OS {
        "android" | "ios" => SPOTIFY_MOBILE_VERSION.to_owned(),
        _ => SPOTIFY_VERSION.to_string(),
    }
}

pub fn spotify_semantic_version() -> String {
    match crate::config::OS {
        "android" | "ios" => SPOTIFY_MOBILE_VERSION.to_owned(),
        _ => SPOTIFY_SEMANTIC_VERSION.to_string(),
    }
}
