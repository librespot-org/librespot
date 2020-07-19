use futures::Future;

use crate::mercury::MercuryError;
use crate::session::Session;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
    pub scope: Vec<String>,
}

pub fn get_token(
    session: &Session,
    client_id: &str,
    scopes: &str,
) -> Box<dyn Future<Item = Token, Error = MercuryError>> {
    let url = format!(
        "hm://keymaster/token/authenticated?client_id={}&scope={}",
        client_id, scopes
    );
    Box::new(session.mercury().get(url).map(move |response| {
        let data = response.payload.first().expect("Empty payload");
        let data = String::from_utf8(data.clone()).unwrap();
        let token: Token = serde_json::from_str(&data).unwrap();

        token
    }))
}
