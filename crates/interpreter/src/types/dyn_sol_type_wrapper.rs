use alloy_dyn_abi::DynSolType;
use derive_more::{Deref, From};
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;

/// Wrapper type which implements deserialization for [`DynSolType`]
#[derive(Debug, Deref, From)]
pub struct DynSolTypeWrapper(DynSolType);

impl<'de> Deserialize<'de> for DynSolTypeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        if let serde_yaml::Value::String(value) = value {
            let sol_type: DynSolType = value.parse().map_err(|e| {
                serde::de::Error::custom(format!("cannot parse Solidity types, error {:#?}", e))
            })?;

            Ok(Self(sol_type))
        } else {
            Err(serde::de::Error::custom(
                "expected a string to respresent Solidity types",
            ))
        }
    }
}

impl DynSolTypeWrapper {
    pub fn inner_ref(&self) -> &DynSolType {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_ok;
    use proptest::prelude::*;

    fn deserialize_dyn_sol_type_strategy() -> impl Strategy<Value = String> {
        let valid_cases = vec![
            "address",
            "bool",
            "string",
            "bytes",
            "bytes8",
            "uint",
            "int",
            "uint8",
            "int8",
            "address[]",
            "string[][6]",
            "(bool, bytes8)",
            "(uint256, (uint8, bool))",
        ];

        any::<prop::sample::Index>()
            .prop_map(move |idx| valid_cases[idx.index(valid_cases.len())].to_string())
    }

    proptest! {
        #[test]
        fn can_deserialize_dyn_sol_type(test_case in deserialize_dyn_sol_type_strategy()) {
            let _dyn_sol_type: DynSolTypeWrapper = assert_ok!(serde_yaml::from_str(&test_case));
        }
    }
}
