use std::time::Duration;

use librespot_core::authentication::Credentials;
use librespot_core::config::SessionConfig;
use librespot_core::session::Session;

use tokio::time::timeout;

#[tokio::test]
async fn test_connection() {
    timeout(Duration::from_secs(30), async {
        let result = Session::connect(
            SessionConfig::default(),
            Credentials::with_password("test", "test"),
            None,
            false,
        )
        .await;

        match result {
            Ok(_) => panic!("Authentication succeeded despite of bad credentials."),
            Err(e) => assert_eq!(e.to_string(), "Login failed with reason: Bad credentials"),
        }
    })
    .await
    .unwrap();
}
