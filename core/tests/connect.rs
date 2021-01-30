use librespot_core::*;

#[cfg(test)]
mod tests {
    use super::*;
    // Test AP Resolve
    use apresolve::apresolve_or_fallback;
    #[tokio::test]
    async fn test_ap_resolve() {
        env_logger::init();
        let ap = apresolve_or_fallback(&None, &None).await;
        println!("AP: {:?}", ap);
    }

    // Test connect
    use authentication::Credentials;
    use config::SessionConfig;
    #[tokio::test]
    async fn test_connection() -> Result<(), Box<dyn std::error::Error>> {
        println!("Running connection test");
        let ap = apresolve_or_fallback(&None, &None).await;
        let credentials = Credentials::with_password(String::from("test"), String::from("test"));
        let session_config = SessionConfig::default();
        let proxy = None;

        println!("Connecting to AP \"{}\"", ap);
        let mut connection = connection::connect(ap, &proxy).await?;
        let rc = connection::authenticate(&mut connection, credentials, &session_config.device_id)
            .await?;
        println!("Authenticated as \"{}\"", rc.username);
        Ok(())
    }
}
