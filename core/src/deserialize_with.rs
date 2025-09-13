use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use protobuf::MessageFull;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

const IGNORE_UNKNOWN: protobuf_json_mapping::ParseOptions = protobuf_json_mapping::ParseOptions {
    ignore_unknown_fields: true,
    _future_options: (),
};

fn parse_value_to_msg<T: MessageFull>(
    value: &Value,
) -> Result<T, protobuf_json_mapping::ParseError> {
    protobuf_json_mapping::parse_from_str_with_options::<T>(&value.to_string(), &IGNORE_UNKNOWN)
}

pub fn base64_proto<'de, T, D>(de: D) -> Result<Option<T>, D::Error>
where
    T: MessageFull,
    D: Deserializer<'de>,
{
    let v: String = Deserialize::deserialize(de)?;
    let bytes = BASE64_STANDARD
        .decode(v)
        .map_err(|e| Error::custom(e.to_string()))?;

    T::parse_from_bytes(&bytes).map(Some).map_err(Error::custom)
}

pub fn json_proto<'de, T, D>(de: D) -> Result<T, D::Error>
where
    T: MessageFull,
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(de)?;
    parse_value_to_msg(&v).map_err(Error::custom)
}

pub fn option_json_proto<'de, T, D>(de: D) -> Result<Option<T>, D::Error>
where
    T: MessageFull,
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(de)?;
    parse_value_to_msg(&v).map(Some).map_err(Error::custom)
}

pub fn vec_json_proto<'de, T, D>(de: D) -> Result<Vec<T>, D::Error>
where
    T: MessageFull,
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(de)?;
    let array = match v {
        Value::Array(array) => array,
        _ => return Err(Error::custom("the value wasn't an array")),
    };

    let res = array
        .iter()
        .flat_map(parse_value_to_msg)
        .collect::<Vec<T>>();

    Ok(res)
}

pub fn boxed<'de, T, D>(de: D) -> Result<Box<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    let v: T = Deserialize::deserialize(de)?;
    Ok(Box::new(v))
}

pub fn bool_from_string<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(de)?.as_ref() {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(Error::invalid_value(
            Unexpected::Str(other),
            &"true or false",
        )),
    }
}
