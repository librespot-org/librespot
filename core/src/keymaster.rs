use futures::Future;
use serde_json;

use mercury::MercuryError;
use session::Session;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
    pub scope: Vec<String>,
}

pub fn get_token(session: &Session, client_id: Option<&str>, scopes: &str) -> Box<Future<Item = Token, Error = MercuryError>> {
    let client_key = option_env!("CLIENT_ID").or(client_id).expect("No Client ID available");
    let url = format!("hm://keymaster/token/authenticated?client_id={}&scope={}", client_key, scopes);
    Box::new(session.mercury().get(url).map(move |response| {
        let data = response.payload.first().expect("Empty payload");
        let data = String::from_utf8(data.clone()).unwrap();
        let token : Token = serde_json::from_str(&data).unwrap();

        token
    }))
}
