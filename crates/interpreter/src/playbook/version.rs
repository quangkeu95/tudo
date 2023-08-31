use derive_more::Display;
use serde_enum_str::Deserialize_enum_str;

#[derive(Debug, Display, Deserialize_enum_str)]
pub enum Version {
    #[serde(rename = "1")]
    V1,
}
