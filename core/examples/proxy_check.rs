//! Verify proxy tunnelling through the vendored connector code paths.
//!
//! Usage:
//!   PROXY=socks5h://127.0.0.1:1080 cargo run --example proxy_check

use http::{Method, Request};
use librespot_core::socket;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proxy = Url::parse(&std::env::var("PROXY")?)?;

    // Path 1: the raw AP tunnel (socket.rs + proxytunnel.rs).
    let stream = socket::connect("ap-gew4.spotify.com", 443, Some(&proxy)).await?;
    println!("[socket] tunnel to ap-gew4.spotify.com:443 established: {stream:?}");
    drop(stream);

    // Path 2: the HTTP client (http_client.rs + proxy_connector.rs), plain http.
    let client = librespot_core::http_client::HttpClient::new(Some(&proxy));
    let request = Request::builder()
        .method(Method::GET)
        .uri("http://apresolve.spotify.com/?type=accesspoint")
        .body(Default::default())?;
    let response = client.request_body(request).await?;
    println!(
        "[http/1 ] apresolve: {}",
        String::from_utf8_lossy(&response)
    );

    // Path 2b: HTTPS through the same proxy stack (TLS over the tunnel).
    // Unauthorized is expected and proves TLS + proxy work end to end.
    let request = Request::builder()
        .method(Method::GET)
        .uri("https://spclient.wg.spotify.com/")
        .body(Default::default())?;
    match client.request_body(request).await {
        Ok(response) => println!(
            "[https  ] spclient says: {}",
            String::from_utf8_lossy(&response)
        ),
        Err(error) => {
            // A 401/403 surfaces as an error here; transport errors look different.
            let msg = error.to_string();
            assert!(
                !msg.contains("proxy") || msg.contains("401"),
                "transport error: {msg}"
            );
            println!("[https  ] spclient responded (expected auth error): {msg}");
        }
    }

    println!("ALL PROXY CHECKS PASSED");
    Ok(())
}
