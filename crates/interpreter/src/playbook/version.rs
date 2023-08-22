use serde_enum_str::Deserialize_enum_str;

#[derive(Debug, Deserialize_enum_str)]
pub enum Version {
    #[serde(rename = "1")]
    V1,
}
