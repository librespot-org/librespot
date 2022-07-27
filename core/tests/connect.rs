use std::time::Duration;

use tokio::time::timeout;

use librespot_core::{authentication::Credentials, config::SessionConfig, session::Session};

#[tokio::test]
async fn test_connection() {
    timeout(Duration::from_secs(30), async {
        let result = Session::new(SessionConfig::default(), None, false)
            .connect(Credentials::with_password("test", "test"))
            .await;

        match result {
            Ok(_) => panic!("Authentication succeeded despite of bad credentials."),
            Err(e) => assert!(!e.to_string().is_empty()), // there should be some error message
        }
    })
    .await
    .unwrap();
}
