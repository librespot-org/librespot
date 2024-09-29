use crate::dealer::protocol::JsonValue;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use librespot_protocol::player::{
    Context, ContextPlayerOptionOverrides, PlayOrigin, TransferState,
};
use protobuf::MessageFull;
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize)]
pub struct Request {
    pub message_id: u32,
    // todo: did only send target_alias_id: null so far, maybe we just ignore it, will see
    // pub target_alias_id: Option<()>,
    pub sent_by_device_id: String,
    pub command: RequestCommand,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "endpoint", rename_all = "snake_case")]
pub enum RequestCommand {
    Transfer(TransferCommand),
    Play(PlayCommand),
    Pause(PauseCommand),
    SeekTo(SeekToCommand),
    // commands that don't send any context
    SkipNext(GenericCommand),
    SkipPrev(GenericCommand),
    Resume(GenericCommand),
    // catch unknown commands, so that we can implement them later
    #[serde(untagged)]
    Unknown(JsonValue),
}

impl Display for RequestCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "endpoint: {}",
            match self {
                RequestCommand::Transfer(_) => "transfer",
                RequestCommand::Play(_) => "play",
                RequestCommand::Pause(_) => "pause",
                RequestCommand::SeekTo(_) => "seek_to",
                RequestCommand::SkipNext(_) => "skip_next",
                RequestCommand::SkipPrev(_) => "skip_prev",
                RequestCommand::Resume(_) => "resume",
                RequestCommand::Unknown(json) => {
                    json.as_object()
                        .and_then(|obj| obj.get("endpoint").map(|v| v.as_str()))
                        .flatten()
                        .unwrap_or("???")
                }
            }
        )
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TransferCommand {
    #[serde(default, deserialize_with = "deserialize_base64_proto")]
    pub data: Option<TransferState>,
    pub options: TransferOptions,
    pub from_device_identifier: String,
    pub logging_params: LoggingParams,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlayCommand {
    #[serde(deserialize_with = "deserialize_json_proto")]
    pub context: Context,
    #[serde(deserialize_with = "deserialize_json_proto")]
    pub play_origin: PlayOrigin,
    pub options: PlayOptions,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PauseCommand {
    // does send options with it, but seems to be empty, investigate which options are send here
    pub logging_params: LoggingParams,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SeekToCommand {
    pub value: u32,
    pub position: u32,
    pub logging_params: LoggingParams,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GenericCommand {
    pub logging_params: LoggingParams,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TransferOptions {
    pub restore_paused: String,
    pub restore_position: String,
    pub restore_track: String,
    pub retain_session: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlayOptions {
    pub skip_to: SkipTo,
    #[serde(deserialize_with = "deserialize_json_proto")]
    pub player_option_overrides: ContextPlayerOptionOverrides,
    pub license: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SkipTo {
    pub track_uid: String,
    pub track_uri: String,
    pub track_index: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoggingParams {
    pub interaction_ids: Option<Vec<String>>,
    pub device_identifier: Option<String>,
    pub command_initiated_time: Option<i64>,
    pub page_instance_ids: Option<Vec<String>>,
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
    protobuf_json_mapping::parse_from_str(&v.to_string()).map_err(Error::custom)
}
