/// Version string of the form "librespot-<sha>"
pub const VERSION_STRING: &str = concat!("librespot-", env!("VERGEN_SHA_SHORT"));

/// Generate a timestamp string representing the build date (UTC).
pub const BUILD_DATE: &str = env!("VERGEN_BUILD_DATE");

/// Short sha of the latest git commit.
pub const SHA_SHORT: &str = env!("VERGEN_SHA_SHORT");

/// Date of the latest git commit.
pub const COMMIT_DATE: &str = env!("VERGEN_COMMIT_DATE");

/// A random build id.
pub const BUILD_ID: &str = env!("VERGEN_BUILD_ID");
