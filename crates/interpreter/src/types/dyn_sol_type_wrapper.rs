use alloy_dyn_abi::DynSolType;
use derive_more::{Deref, From};
use ethers::abi::ParamType;
use serde::{Deserialize, Deserializer};
use serde_value::Value;

/// Wrapper type which implements deserialization for [`DynSolType`]
#[derive(Debug, Clone, Deref, From)]
pub struct DynSolTypeWrapper(DynSolType);

/// By default the DynSolTypeWrapper is a tuple
impl Default for DynSolTypeWrapper {
    fn default() -> Self {
        Self(DynSolType::Tuple(vec![]))
    }
}

impl<'de> Deserialize<'de> for DynSolTypeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        if let Value::String(value) = value {
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
    /// Return inner [`DynSolType`] reference
    pub fn inner_ref(&self) -> &DynSolType {
        &self.0
    }

    fn to_ethers_param_type(sol_type: &DynSolType) -> ParamType {
        match sol_type {
            DynSolType::Address => ParamType::Address,
            DynSolType::Bool => ParamType::Bool,
            DynSolType::Int(size) => ParamType::Int(*size),
            DynSolType::Uint(size) => ParamType::Uint(*size),
            DynSolType::FixedBytes(size) => ParamType::FixedBytes(*size),
            DynSolType::Bytes => ParamType::Bytes,
            DynSolType::String => ParamType::String,
            DynSolType::Array(root_type) => {
                ParamType::Array(Box::new(Self::to_ethers_param_type(root_type)))
            }
            DynSolType::FixedArray(root_type, size) => {
                ParamType::FixedArray(Box::new(Self::to_ethers_param_type(root_type)), *size)
            }
            DynSolType::Tuple(vec_param_type) => ParamType::Tuple(
                vec_param_type
                    .iter()
                    .map(Self::to_ethers_param_type)
                    .collect::<Vec<ParamType>>(),
            ),
            DynSolType::CustomStruct {
                name: _,
                prop_names: _,
                tuple: _,
            } => panic!("cannot convert DynSolType::CustomSutrct into ethers ParamType"),
            #[allow(unreachable_patterns)]
            _ => {
                panic!("unsupported DynSolType")
            }
        }
    }

    /// Convert inner [`DynSolType`] into [`ParamType`]
    pub fn as_ethers_param_type(&self) -> ParamType {
        Self::to_ethers_param_type(&self.0)
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
