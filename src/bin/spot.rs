#[tokio::main]
async fn main() {
    let response = reqwest::get(
        "
https://open.spotify.com/get_access_token?reason=transport&productType=web_player",
    )
    .await
    // each response is wrapped in a `Result` type
    // we'll unwrap here for simplicity
    .unwrap()
    .text()
    .await;
    println!("{:?}", response);
}
