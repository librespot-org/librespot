include!(concat!(env!("OUT_DIR"), "/version.rs"));

pub fn version_string() -> String {
    format!("librespot-{}", short_sha())
}
