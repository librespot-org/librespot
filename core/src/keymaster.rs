use serde::Deserialize;

use crate::{mercury::MercuryError, session::Session};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
    pub scope: Vec<String>,
}

pub async fn get_token(
    session: &Session,
    client_id: &str,
    scopes: &str,
) -> Result<Token, MercuryError> {
    let url = format!(
        "hm://keymaster/token/authenticated?client_id={}&scope={}",
        client_id, scopes
    );
    let response = session.mercury().get(url).await?;
    let data = response.payload.first().expect("Empty payload");
    serde_json::from_slice(data.as_ref()).map_err(|_| MercuryError)
}
