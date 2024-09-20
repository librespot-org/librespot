use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use librespot_protocol::player::TransferState;
use protobuf::MessageFull;
use serde::{Deserialize, Deserializer};

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
