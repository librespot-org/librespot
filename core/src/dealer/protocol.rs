use std::collections::HashMap;

use serde::Deserialize;

pub type JsonValue = serde_json::Value;
pub type JsonObject = serde_json::Map<String, JsonValue>;

#[derive(Clone, Debug, Deserialize)]
pub struct Request<P> {
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub message_ident: String,
    pub key: String,
    pub payload: P,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Payload {
    pub message_id: i32,
    pub sent_by_device_id: String,
    pub command: JsonObject,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Message<P> {
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub method: Option<String>,
    #[serde(default)]
    pub payloads: Vec<P>,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOrRequest {
    Message(Message<JsonValue>),
    Request(Request<Payload>),
}
