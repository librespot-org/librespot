pub fn version_string() -> String {
    format!("librespot-{}", short_sha())
}

// Generate a timestamp representing now (UTC) in RFC3339 format.
pub fn now() -> &'static str {
    env!("VERGEN_BUILD_TIMESTAMP")
}

// Generate a timstamp string representing now (UTC).
pub fn short_now() -> &'static str {
    env!("VERGEN_BUILD_DATE")
}

// Generate a SHA string
pub fn sha() -> &'static str {
    env!("VERGEN_SHA")
}

// Generate a short SHA string
pub fn short_sha() -> &'static str {
    env!("VERGEN_SHA_SHORT")
}

// Generate the commit date string
pub fn commit_date() -> &'static str {
    env!("VERGEN_COMMIT_DATE")
}

// Generate the target triple string
pub fn target() -> &'static str {
    env!("VERGEN_TARGET_TRIPLE")
}

// Generate a semver string
pub fn semver() -> &'static str {
    // env!("VERGEN_SEMVER")
    env!("CARGO_PKG_VERSION")
}

// Generate a random build id.
pub fn build_id() -> &'static str {
    env!("VERGEN_BUILD_ID")
}
