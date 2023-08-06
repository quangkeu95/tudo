use alloy_sol_type_parser::{TupleSpecifier, TypeSpecifier, TypeStem};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer};

lazy_static! {
    static ref UINT_REGEX: Regex = Regex::new("^uint(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)?$").unwrap();
    static ref INT_REGEX: Regex = Regex::new("^int(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)?$").unwrap();
    static ref BYTES_REGEX: Regex = Regex::new("^bytes([1-9]|1[0-9]|2[0-9]|30|31|32)$").unwrap();
    static ref ARRAY_REGEX: Regex = Regex::new(r#"^(.*)(\[\d?\])$"#).unwrap();
    static ref TUPLE_REGEX: Regex = Regex::new(r#"^\(.*\)$"#).unwrap();
}

/// Solidity types representations
#[derive(Debug, PartialEq)]
// #[serde(rename_all = "snake_case")]
pub enum SolidityTypes {
    Address,
    Bytes,
    Bool,
    String,
    Uint(usize),
    Int(usize),
    FixedBytes(usize),
    Array(Box<SolidityTypes>),
    FixedArray(Box<SolidityTypes>, usize),
    Tuple(Vec<SolidityTypes>), // Tuple(Vec<SolidityTypes>),
}

impl<'de> Deserialize<'de> for SolidityTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if let serde_json::Value::String(str_value) = value {
            Self::parse_root_type::<D>(&str_value)
        } else {
            Err(serde::de::Error::custom("Expected a string"))
        }
    }
}

impl SolidityTypes {
    fn parse_root_type<'de, D>(s: &str) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match s {
            "address" => Ok(Self::Address),
            "bytes" => Ok(Self::Bytes),
            "bool" => Ok(Self::Bool),
            "string" => Ok(Self::String),
            s if UINT_REGEX.is_match(s) => Self::parse_uint::<D>(s),
            s if INT_REGEX.is_match(s) => Self::parse_int::<D>(s),
            s if BYTES_REGEX.is_match(s) => Self::parse_fixed_bytes::<D>(s),
            s if ARRAY_REGEX.is_match(s) => Self::parse_array::<D>(s),
            s if TUPLE_REGEX.is_match(s) => Self::parse_tuple::<D>(s),
            other => Err(serde::de::Error::custom(format!(
                "Unknown SolidityTypes {:#?}",
                other
            ))),
        }
    }

    /// Parse Solidity uint type of various sizes, from uint8 to uint256 (with steps of 8)
    fn parse_uint<'de, D>(s: &str) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if s == "uint" {
            return Ok(Self::Uint(256));
        }
        if let Ok(size) = s[4..].parse::<usize>() {
            if size > 0 && size <= 256 && size % 8 == 0 {
                return Ok(Self::Uint(size));
            }
        }
        Err(serde::de::Error::custom(format!(
            "error parsing uint, invalid uint: {:#?}",
            s
        )))
    }

    /// Parse Solidity int type of various sizes, from int8 to int256 (with steps of 8)
    fn parse_int<'de, D>(s: &str) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if s == "int" {
            return Ok(Self::Int(256));
        }
        if let Ok(size) = s[3..].parse::<usize>() {
            if size > 0 && size <= 256 && size % 8 == 0 {
                return Ok(Self::Int(size));
            }
        }
        Err(serde::de::Error::custom(format!(
            "error parsing int, invalid int: {:#?}",
            s
        )))
    }

    /// Parse Solidity fixed bytes type of various sizes, from bytes1 to bytes32
    fn parse_fixed_bytes<'de, D>(s: &str) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Ok(size) = s[5..].parse::<usize>() {
            if size >= 1 && size <= 32 {
                return Ok(Self::FixedBytes(size));
            }
        }
        Err(serde::de::Error::custom(format!(
            "error parsing fixed bytes, invalid fixed bytes: {:#?}",
            s
        )))
    }

    /// Parse Solidity fixed size array or dynamic size array
    fn parse_array<'de, D>(s: &str) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let type_specifier = TypeSpecifier::try_from(s)
            .map_err(|e| serde::de::Error::custom(format!("error parsing array {:#?}", e)))?;

        let root_type = match type_specifier.stem() {
            TypeStem::Root(root_type) => Self::parse_root_type::<D>(root_type.span())?,
            TypeStem::Tuple(tuple_type) => Self::parse_tuple::<D>(tuple_type.span())?,
        };

        if type_specifier.sizes.len() == 0 {
            return Err(serde::de::Error::custom("Array length is zero"));
        }

        let mut inner_type = root_type;

        for array_size in type_specifier.sizes {
            if let Some(size) = array_size {
                inner_type = Self::FixedArray(Box::new(inner_type), size.get());
            } else {
                inner_type = Self::Array(Box::new(inner_type));
            }
        }

        Ok(inner_type)
    }

    /// Parse Solidity tuple type
    fn parse_tuple<'de, D>(s: &str) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let tuple_specifier = TupleSpecifier::try_from(s)
            .map_err(|e| serde::de::Error::custom(format!("error parsing tuple {:#?}", e)))?;

        let tuple: Result<Vec<SolidityTypes>, D::Error> = tuple_specifier
            .types
            .into_iter()
            .map(|item| Self::parse_root_type::<D>(item.span()))
            .collect();
        let tuple = tuple?;

        Ok(Self::Tuple(tuple))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::*;

    macro_rules! assert_match_solidity_type {
        ($value:expr, $expected_type:pat) => {
            let parsed = serde_yaml::from_str($value).unwrap();
            assert_matches!(parsed, $expected_type);
        };
        ($value:expr, $expected_type:ident) => {
            let parsed = serde_yaml::from_str($value).unwrap();
            assert_matches!(parsed, $expected_type);
        };
    }

    #[test]
    fn can_parse_primary_solidity_types() {
        assert_match_solidity_type!("address", SolidityTypes::Address);
        assert_match_solidity_type!("bytes", SolidityTypes::Bytes);
        assert_match_solidity_type!("bool", SolidityTypes::Bool);
        assert_match_solidity_type!("string", SolidityTypes::String);
    }

    #[test]
    fn can_parse_uint() {
        assert_match_solidity_type!("uint", SolidityTypes::Uint(256));
        assert_match_solidity_type!("uint256", SolidityTypes::Uint(256));
        assert_match_solidity_type!("uint8", SolidityTypes::Uint(8));
        assert_err!(serde_yaml::from_str::<SolidityTypes>("uint0"));
        assert_err!(serde_yaml::from_str::<SolidityTypes>("uint1"));
        assert_err!(serde_yaml::from_str::<SolidityTypes>("uint257"));
    }

    #[test]
    fn can_parse_int() {
        assert_match_solidity_type!("int", SolidityTypes::Int(256));
        assert_match_solidity_type!("int256", SolidityTypes::Int(256));
        assert_match_solidity_type!("int8", SolidityTypes::Int(8));
        assert_err!(serde_yaml::from_str::<SolidityTypes>("int0"));
        assert_err!(serde_yaml::from_str::<SolidityTypes>("int1"));
        assert_err!(serde_yaml::from_str::<SolidityTypes>("int257"));
    }

    #[test]
    fn can_parse_fixed_bytes() {
        assert_match_solidity_type!("bytes1", SolidityTypes::FixedBytes(1));
        assert_match_solidity_type!("bytes8", SolidityTypes::FixedBytes(8));
        assert_match_solidity_type!("bytes32", SolidityTypes::FixedBytes(32));
        assert_err!(serde_yaml::from_str::<SolidityTypes>("bytes0"));
        assert_err!(serde_yaml::from_str::<SolidityTypes>("bytes33"));
    }

    #[test]
    fn can_parse_array() {
        {
            let array: SolidityTypes = serde_yaml::from_str("address[]").unwrap();
            match array {
                SolidityTypes::Array(root_type) => {
                    assert_eq!(root_type, Box::new(SolidityTypes::Address));
                }
                _ => panic!("cannot parse address[]"),
            }
        }
        {
            let array: SolidityTypes = serde_yaml::from_str("string[]").unwrap();
            match array {
                SolidityTypes::Array(root_type) => {
                    assert_eq!(root_type, Box::new(SolidityTypes::String));
                }
                _ => panic!("cannot parse string[]"),
            }
        }
        {
            let array: SolidityTypes = serde_yaml::from_str("bool[]").unwrap();
            match array {
                SolidityTypes::Array(root_type) => {
                    assert_eq!(root_type, Box::new(SolidityTypes::Bool));
                }
                _ => panic!("cannot parse bool[]"),
            }
        }
        {
            let array: SolidityTypes = serde_yaml::from_str("bytes[]").unwrap();
            match array {
                SolidityTypes::Array(root_type) => {
                    assert_eq!(root_type, Box::new(SolidityTypes::Bytes));
                }
                _ => panic!("cannot parse bytes[]"),
            }
        }
        {
            let array: SolidityTypes = serde_yaml::from_str("uint32[]").unwrap();
            match array {
                SolidityTypes::Array(root_type) => {
                    assert_eq!(root_type, Box::new(SolidityTypes::Uint(32)));
                }
                _ => panic!("cannot parse uint32[]"),
            }
        }
        {
            let array: SolidityTypes = serde_yaml::from_str("bytes8[][5]").unwrap();
            match array {
                SolidityTypes::FixedArray(root_type, size) => {
                    assert_eq!(size, 5);
                    assert_eq!(
                        root_type,
                        Box::new(SolidityTypes::Array(Box::new(SolidityTypes::FixedBytes(8))))
                    );
                }
                _ => panic!("cannot parse bytes8[][5]"),
            }
        }
    }

    #[test]
    fn can_parse_tuple() {
        {
            let tuple: SolidityTypes =
                serde_yaml::from_str("(uint8, address, bool, bytes[])").unwrap();
            match tuple {
                SolidityTypes::Tuple(inner_tuple) => {
                    assert_eq!(
                        inner_tuple,
                        vec![
                            SolidityTypes::Uint(8),
                            SolidityTypes::Address,
                            SolidityTypes::Bool,
                            SolidityTypes::Array(Box::new(SolidityTypes::Bytes)),
                        ]
                    );
                }
                _ => panic!("cannot parse tuple"),
            }
        }
    }
}
