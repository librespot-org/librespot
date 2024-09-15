use crate::Error;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use flate2::read::GzDecoder;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;

pub type JsonValue = serde_json::Value;
pub type JsonObject = serde_json::Map<String, JsonValue>;

#[derive(Clone, Debug, Deserialize)]
pub struct Payload {
    pub message_id: u32,
    pub sent_by_device_id: String,
    pub command: JsonObject,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Request {
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub message_ident: String,
    pub key: String,
    pub payload: Payload,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketMessage {
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub method: Option<String>,
    #[serde(default)]
    pub payloads: Vec<PayloadValue>,
    pub uri: String,
}

pub const PAYLOAD_DEFAULT: PayloadValue = PayloadValue::Bytes(Vec::new());
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum PayloadValue {
    String(String),
    Bytes(Vec<u8>),
    Others(JsonValue),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(super) enum MessageOrRequest {
    Message(WebsocketMessage),
    Request(Request),
}

#[derive(Clone, Debug)]
pub struct Message {
    pub headers: HashMap<String, String>,
    pub payload: Vec<u8>,
    pub uri: String,
}

impl WebsocketMessage {
    pub fn handle_payload(&self) -> Result<Vec<u8>, Error> {
        let payload = match self.payloads.first() {
            None => return Ok(Vec::new()),
            Some(p) => p,
        };

        let encoding = self.headers.get("Transfer-Encoding").map(String::as_str);
        if let Some(encoding) = encoding {
            trace!("message was send with {encoding} encoding ");
        }

        match payload {
            PayloadValue::String(string) => {
                trace!("payload: {string}");

                let bytes = BASE64_STANDARD
                    .decode(string)
                    .map_err(Error::failed_precondition)?;

                if !matches!(encoding, Some("gzip")) {
                    return Ok(bytes);
                }

                let mut gz = GzDecoder::new(&bytes[..]);
                let mut bytes = vec![];
                match gz.read_to_end(&mut bytes) {
                    Ok(i) if i == bytes.len() => Ok(bytes),
                    Ok(_) => Err(Error::failed_precondition(
                        "read bytes mismatched with expected bytes",
                    )),
                    Err(why) => Err(Error::failed_precondition(why)),
                }
            }
            PayloadValue::Bytes(bytes) => Ok(bytes.clone()),
            PayloadValue::Others(others) => Err(Error::unimplemented(format!(
                "Received unknown data from websocket message: {others:?}"
            ))),
        }
    }
}
