use std::collections::HashMap;
use std::io::{Error as IoError, Read};

use crate::Error;
use base64::prelude::BASE64_STANDARD;
use base64::{DecodeError, Engine};
use flate2::read::GzDecoder;
use librespot_protocol::player::TransferState;
use protobuf::MessageFull;
use serde::{Deserialize, Deserializer};
use serde_json::Error as SerdeError;
use thiserror::Error;

pub type JsonValue = serde_json::Value;

#[derive(Debug, Error)]
enum ProtocolError {
    #[error("base64 decoding failed: {0}")]
    Base64(DecodeError),
    #[error("gzip decoding failed: {0}")]
    GZip(IoError),
    #[error("Deserialization failed: {0}")]
    Deserialization(SerdeError),
}

impl From<ProtocolError> for Error {
    fn from(err: ProtocolError) -> Self {
        Error::failed_precondition(err)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Payload {
    pub compressed: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebsocketRequest {
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
    Request(WebsocketRequest),
}

#[derive(Clone, Debug)]
pub struct Message {
    pub headers: HashMap<String, String>,
    pub payload: Vec<u8>,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Request {
    pub message_id: u32,
    // todo: did only send target_alias_id: null so far, maybe we just ignore it, will see
    // pub target_alias_id: Option<()>,
    pub sent_by_device_id: String,
    pub command: RequestCommand,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestCommand {
    pub endpoint: RequestEndpoint,
    #[serde(default, deserialize_with = "deserialize_base64_proto")]
    pub data: Option<TransferState>,
    pub options: Option<Options>,
    pub from_device_identifier: String,
    pub logging_params: LoggingParams,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestEndpoint {
    Transfer,
    Unknown(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Options {
    pub restore_paused: String,
    pub restore_position: String,
    pub restore_track: String,
    pub retain_session: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoggingParams {
    interaction_ids: Vec<String>,
    device_identifier: Option<String>,
    command_initiated_time: Option<i64>,
    page_instance_ids: Option<Vec<String>>,
}

impl WebsocketMessage {
    pub fn handle_payload(&self) -> Result<Vec<u8>, Error> {
        let payload = match self.payloads.first() {
            None => return Ok(Vec::new()),
            Some(p) => p,
        };

        let bytes = match payload {
            PayloadValue::String(string) => BASE64_STANDARD
                .decode(string)
                .map_err(ProtocolError::Base64)?,
            PayloadValue::Bytes(bytes) => bytes.clone(),
            PayloadValue::Others(others) => {
                return Err(Error::unimplemented(format!(
                    "Received unknown data from websocket message: {others:?}"
                )))
            }
        };

        handle_transfer_encoding(&self.headers, bytes)
    }
}

impl WebsocketRequest {
    pub fn handle_payload(&self) -> Result<Request, Error> {
        let payload_bytes = BASE64_STANDARD
            .decode(&self.payload.compressed)
            .map_err(ProtocolError::Base64)?;

        let payload = handle_transfer_encoding(&self.headers, payload_bytes)?;
        let payload = String::from_utf8(payload)?;
        debug!("request payload: {payload}");

        let request = serde_json::from_str(&payload).map_err(ProtocolError::Deserialization)?;
        Ok(request)
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

pub fn deserialize_base64_proto<'de, T, D>(de: D) -> Result<Option<T>, D::Error>
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
