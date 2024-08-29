use librespot_core::SessionConfig;
use librespot_oauth::get_access_token;

fn main() {
    let mut builder = env_logger::Builder::new();
    builder.parse_filters("librespot=trace");
    builder.init();

    match get_access_token(
        &SessionConfig::default().client_id,
        vec!["streaming"],
        Some(1337),
    ) {
        Ok(token) => println!("Success: {token:#?}"),
        Err(e) => println!("Failed: {e}"),
    };
}
