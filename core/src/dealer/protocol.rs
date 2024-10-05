pub mod request;

pub use request::*;

use std::collections::HashMap;
use std::io::{Error as IoError, Read};

use crate::Error;
use base64::prelude::BASE64_STANDARD;
use base64::{DecodeError, Engine};
use flate2::read::GzDecoder;
use protobuf::MessageFull;
use serde::{Deserialize, Deserializer};
use serde_json::Error as SerdeError;
use thiserror::Error;

type JsonValue = serde_json::Value;

#[derive(Debug, Error)]
enum ProtocolError {
    #[error("base64 decoding failed: {0}")]
    Base64(DecodeError),
    #[error("gzip decoding failed: {0}")]
    GZip(IoError),
    #[error("deserialization failed: {0}")]
    Deserialization(SerdeError),
    #[error("payload had more then one value. had {0} values")]
    MoreThenOneValue(usize),
    #[error("payload was empty")]
    Empty,
}

impl From<ProtocolError> for Error {
    fn from(err: ProtocolError) -> Self {
        Error::failed_precondition(err)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct Payload {
    pub compressed: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct WebsocketRequest {
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub message_ident: String,
    pub key: String,
    pub payload: Payload,
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct WebsocketMessage {
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub method: Option<String>,
    #[serde(default)]
    pub payloads: Vec<MessagePayloadValue>,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum MessagePayloadValue {
    String(String),
    Bytes(Vec<u8>),
    Json(JsonValue),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(super) enum MessageOrRequest {
    Message(WebsocketMessage),
    Request(WebsocketRequest),
}

#[derive(Clone, Debug)]
pub enum PayloadValue {
    Empty,
    Raw(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct Message {
    pub headers: HashMap<String, String>,
    pub payload: PayloadValue,
    pub uri: String,
}

impl WebsocketMessage {
    pub fn handle_payload(&mut self) -> Result<PayloadValue, Error> {
        if self.payloads.is_empty() {
            return Ok(PayloadValue::Empty);
        } else if self.payloads.len() > 1 {
            return Err(ProtocolError::MoreThenOneValue(self.payloads.len()).into());
        }

        let payload = self.payloads.pop().ok_or(ProtocolError::Empty)?;
        let bytes = match payload {
            MessagePayloadValue::String(string) => BASE64_STANDARD
                .decode(string)
                .map_err(ProtocolError::Base64)?,
            MessagePayloadValue::Bytes(bytes) => bytes,
            MessagePayloadValue::Json(json) => {
                return Err(Error::unimplemented(format!(
                    "Received unknown data from websocket message: {json:?}"
                )))
            }
        };

        handle_transfer_encoding(&self.headers, bytes).map(PayloadValue::Raw)
    }
}

impl WebsocketRequest {
    pub fn handle_payload(&self) -> Result<Request, Error> {
        let payload_bytes = BASE64_STANDARD
            .decode(&self.payload.compressed)
            .map_err(ProtocolError::Base64)?;

        let payload = handle_transfer_encoding(&self.headers, payload_bytes)?;
        let payload = String::from_utf8(payload)?;
        debug!("request: {payload}");

        serde_json::from_str(&payload)
            .map_err(ProtocolError::Deserialization)
            .map_err(Into::into)
    }
}

fn handle_transfer_encoding(
    headers: &HashMap<String, String>,
    data: Vec<u8>,
) -> Result<Vec<u8>, Error> {
    let encoding = headers.get("Transfer-Encoding").map(String::as_str);
    if let Some(encoding) = encoding {
        trace!("message was send with {encoding} encoding ");
    }

    if !matches!(encoding, Some("gzip")) {
        return Ok(data);
    }

    let mut gz = GzDecoder::new(&data[..]);
    let mut bytes = vec![];
    match gz.read_to_end(&mut bytes) {
        Ok(i) if i == bytes.len() => Ok(bytes),
        Ok(_) => Err(Error::failed_precondition(
            "read bytes mismatched with expected bytes",
        )),
        Err(why) => Err(ProtocolError::GZip(why).into()),
    }
}

fn deserialize_base64_proto<'de, T, D>(de: D) -> Result<Option<T>, D::Error>
where
    T: MessageFull,
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let v: String = serde::Deserialize::deserialize(de)?;
    let bytes = BASE64_STANDARD
        .decode(v)
        .map_err(|e| Error::custom(e.to_string()))?;

    T::parse_from_bytes(&bytes).map(Some).map_err(Error::custom)
}

fn deserialize_json_proto<'de, T, D>(de: D) -> Result<T, D::Error>
where
    T: MessageFull,
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let v: serde_json::Value = serde::Deserialize::deserialize(de)?;
    protobuf_json_mapping::parse_from_str(&v.to_string()).map_err(|why| {
        warn!("deserialize_json_proto: {v}");
        error!("deserialize_json_proto: {why}");
        Error::custom(why)
    })
}
