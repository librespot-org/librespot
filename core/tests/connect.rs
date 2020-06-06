use futures::future::TryFutureExt;
use librespot_core::*;
use tokio::runtime;

#[cfg(test)]
mod tests {
    use super::*;
    // Test AP Resolve
    use apresolve::apresolve_or_fallback;
    #[test]
    fn test_ap_resolve() {
        let mut rt = runtime::Runtime::new().unwrap();
        let ap = rt.block_on(apresolve_or_fallback(&None, &Some(80)));
        println!("AP: {:?}", ap);
    }

    // Test connect
    use authentication::Credentials;
    use config::SessionConfig;
    use connection;
    #[test]
    fn test_connection() {
        println!("Running connection test");
        let mut rt = runtime::Runtime::new().unwrap();
        let access_point_addr = rt.block_on(apresolve_or_fallback(&None, &None)).unwrap();
        let credentials = Credentials::with_password(String::from("test"), String::from("test"));
        let session_config = SessionConfig::default();
        let proxy = None;

        println!("Connecting to AP \"{}\"", access_point_addr);
        let connection = connection::connect(access_point_addr, &proxy);

        let device_id = session_config.device_id.clone();
        let authentication = connection.and_then(move |connection| {
            connection::authenticate(connection, credentials, device_id)
        });
        match rt.block_on(authentication) {
            Ok((_transport, reusable_credentials)) => {
                println!("Authenticated as \"{}\" !", reusable_credentials.username)
            }
            // TODO assert that we get BadCredentials once we don't panic
            Err(e) => println!("ConnectError: {:?}", e),
        }
    }
}
