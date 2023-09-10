use alloy_dyn_abi::{DynSolType, DynSolValue};
use alloy_primitives::Bytes;
use serde::{Deserialize, Deserializer};
use serde_value::Value;
use thiserror::Error;

use crate::alloy_converter::AlloyConverter;

use super::DynSolTypeWrapper;

/// Solidity function argument, which contains a Solidity type specifier and a Solidity value
#[derive(Debug, Clone)]
pub struct FunctionArgument {
    pub solidity_type: DynSolTypeWrapper,
    pub solidity_value: DynSolValue,
}

impl<'de> Deserialize<'de> for FunctionArgument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct FunctionArgumentHelper {
            #[serde(alias = "type")]
            solidity_type: DynSolTypeWrapper,
            #[serde(alias = "value")]
            solidity_value: Value,
        }

        let helper = FunctionArgumentHelper::deserialize(deserializer)?;

        let solidity_value =
            serde_json::to_value(helper.solidity_value).map_err(serde::de::Error::custom)?;

        let solidity_value: DynSolValue = helper
            .solidity_type
            .coerce(&solidity_value)
            .map_err(serde::de::Error::custom)?;

        Ok(Self {
            solidity_type: helper.solidity_type,
            solidity_value,
        })
    }
}

impl FunctionArgument {
    /// Return Solidity type [`DynSolType`]
    pub fn get_sol_type(&self) -> &DynSolType {
        self.solidity_type.inner_ref()
    }

    /// Convert the inner alloy Solidity value into the equivalent [`ethers::abi::Token`]
    pub fn to_ethers_abi_token(&self) -> Result<ethers::abi::Token, FunctionArgumentError> {
        Self::dyn_sol_value_to_ethers_abi_token(&self.solidity_value)
    }

    /// Convert [`DynSolValue`] into [`ethers::abi::Token`]
    fn dyn_sol_value_to_ethers_abi_token(
        solidity_value: &DynSolValue,
    ) -> Result<ethers::abi::Token, FunctionArgumentError> {
        match solidity_value {
            DynSolValue::Address(address) => Ok(ethers::abi::Token::Address(
                AlloyConverter::from_alloy_address(address),
            )),
            DynSolValue::Bool(value) => Ok(ethers::abi::Token::Bool(*value)),
            DynSolValue::Int(value, size) => {
                let int_value = AlloyConverter::from_alloy_int(value, *size);
                let uint_value: ethers::types::U256 = int_value
                    .try_into()
                    .map_err(|_e| FunctionArgumentError::ConvertFromUintToIntError(int_value))?;

                Ok(ethers::abi::Token::Int(uint_value))
            }
            DynSolValue::Uint(value, size) => Ok(ethers::abi::Token::Uint(
                AlloyConverter::from_alloy_uint(value, *size),
            )),
            DynSolValue::FixedBytes(value, _size) => Ok(ethers::abi::Token::FixedBytes(
                AlloyConverter::from_alloy_fixed_bytes(value).to_vec(),
            )),
            DynSolValue::Bytes(value) => Ok(ethers::abi::Token::Bytes(
                AlloyConverter::from_alloy_bytes(&Bytes::from(value.clone())).to_vec(),
            )),
            DynSolValue::String(value) => Ok(ethers::abi::Token::String(value.clone())),
            DynSolValue::Array(value_array) => {
                let converted_array: Result<Vec<ethers::abi::Token>, FunctionArgumentError> =
                    value_array
                        .iter()
                        .map(Self::dyn_sol_value_to_ethers_abi_token)
                        .collect();
                Ok(ethers::abi::Token::Array(converted_array?))
            }
            DynSolValue::FixedArray(value_array) => {
                let converted_array: Result<Vec<ethers::abi::Token>, FunctionArgumentError> =
                    value_array
                        .iter()
                        .map(Self::dyn_sol_value_to_ethers_abi_token)
                        .collect();
                Ok(ethers::abi::Token::FixedArray(converted_array?))
            }
            DynSolValue::Tuple(value_tuple) => {
                let converted_array: Result<Vec<ethers::abi::Token>, FunctionArgumentError> =
                    value_tuple
                        .iter()
                        .map(Self::dyn_sol_value_to_ethers_abi_token)
                        .collect();
                Ok(ethers::abi::Token::Tuple(converted_array?))
            }
            custom_struct @ DynSolValue::CustomStruct {
                name: _,
                prop_names: _,
                tuple: _,
            } => Err(FunctionArgumentError::NotSupportedType(
                custom_struct.clone(),
            )),
            #[allow(unreachable_patterns)]
            _ => {
                panic!("unsupported DynSolValue");
            }
        }
    }

    // pub fn encode_params(&self) -> Vec<u8> {
    //     self.solidity_value.encode_params()
    // }

    /// Encode the inner Solidity value into a packed byte array.
    pub fn encode_packed(&self) -> Vec<u8> {
        self.solidity_value.encode_packed()
    }

    /// Encode the inner Solidity value into a byte array by wrapping it into a 1-element sequence. This method expected to produce similar output as the [`ethers::abi::encode`] method.
    pub fn encode_single(&self) -> Vec<u8> {
        self.solidity_value.encode_single()
    }

    // pub fn encode(&self) -> Option<Vec<u8>> {
    //     self.solidity_value.encode()
    // }
}

#[derive(Debug, Error)]
pub enum FunctionArgumentError {
    #[error("convert DynSolValue to ethers::abi::Token error {:#?}", .0)]
    ConvertToEthersAbiTokenError(DynSolValue),
    #[error("convert from I256 value {:#?} to U256 error", .0)]
    ConvertFromUintToIntError(ethers::types::I256),
    #[error("not supported type: {:#?}", .0)]
    NotSupportedType(DynSolValue),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy_dyn_abi::{DynSolType, DynSolValue};
    use alloy_primitives::U256;
    use claims::{assert_matches, assert_some};
    use ethers::{abi::Token, types::H160};

    use super::FunctionArgument;
    use proptest::prelude::*;

    #[test]
    fn can_parse_uint_to_function_arguments() {
        let content = r#"
            - type: uint256
              value: 1000
            - type: uint256 
              value: "96000000000000000000"
            - type: uint8
              value: 10
        "#;

        let function_arguments: Vec<FunctionArgument> = serde_yaml::from_str(content).unwrap();

        assert_matches!(
            function_arguments[0].solidity_type.inner_ref(),
            DynSolType::Uint(256)
        );
        assert_matches!(
            function_arguments[1].solidity_type.inner_ref(),
            DynSolType::Uint(256)
        );
        assert_matches!(
            function_arguments[2].solidity_type.inner_ref(),
            DynSolType::Uint(8)
        );

        {
            let (value, size) = assert_some!(function_arguments[0].solidity_value.as_uint());
            assert_eq!(value, U256::from_str("1000").unwrap());
            assert_eq!(size, 256);
        }
        {
            let (value, size) = assert_some!(function_arguments[1].solidity_value.as_uint());
            assert_eq!(value, U256::from_str("96000000000000000000").unwrap());
            assert_eq!(size, 256);
        }
        {
            let (value, size) = assert_some!(function_arguments[2].solidity_value.as_uint());
            assert_eq!(value, U256::from_str("10").unwrap());
            assert_eq!(size, 8);
        }
    }

    macro_rules! match_array {
        ($actual:expr, DynSolType::Array(Box::new($expected:pat))) => {
            match $actual {
                DynSolType::Array(inner) => {
                    let inner = inner.clone();
                    match_array!(*inner, $expected);
                }
                _ => {
                    panic!("Parse array to function arguments error");
                }
            }
        };

        ($actual:expr, $expected:pat) => {
            match $actual {
                DynSolType::Array(inner) => {
                    let inner = inner.clone();
                    assert_matches!(*inner, $expected);
                }
                _ => {
                    panic!("Parse array to function arguments error");
                }
            }
        };
    }

    #[test]
    fn can_parse_array_to_function_arguments() {
        let content = r#"
            - type: address[]
              value: ["0x176Fdfc4fBfb14A4D79BC56b8d4fCC87275b35f8", "0x535901D904E2d996128D14871f5bC8d2d7cA1156"]
            - type: bytes[]
              value: ["0x12", "0x4567"]          
            - type: uint256[][]
              value: [[1, 2, 3], [4, 5, 6, 7, 8]]
        "#;

        let function_arguments: Vec<FunctionArgument> = serde_yaml::from_str(content).unwrap();

        match_array!(function_arguments[0].get_sol_type(), DynSolType::Address);
        match_array!(function_arguments[1].get_sol_type(), DynSolType::Bytes);
        match_array!(
            function_arguments[2].get_sol_type(),
            DynSolType::Array(Box::new(DynSolType::Uint(256)))
        );
    }

    proptest! {
        #[test]
        fn can_encode_address_function_arguments(address in DynSolValue::type_strategy(&DynSolType::Address)) {
            let function_argument = FunctionArgument {
                solidity_type: DynSolType::Address.into(),
                solidity_value: address.clone()
            };

            let inner_address = address.as_address().unwrap();
            let ethers_address = H160::from(inner_address.into_array());

            assert_eq!(function_argument.encode_single(), function_argument.encode_single());
            assert_eq!(function_argument.encode_single(), ethers::abi::encode(&[Token::Address(ethers_address)]));
        }

        #[test]
        fn can_encode_array_address_function_arguments(value_array in DynSolValue::type_strategy(&DynSolType::Array(Box::new(DynSolType::Address)))) {
            let function_argument = FunctionArgument {
                solidity_type: DynSolType::Array(Box::new(DynSolType::Address)).into(),
                solidity_value: value_array.clone(),
            };

            let ethers_abi_token = function_argument.to_ethers_abi_token().unwrap();
            let ethers_abi_encoded = ethers::abi::encode(&[ethers_abi_token]);

            assert_eq!(ethers_abi_encoded, function_argument.encode_single());
        }

        #[test]
        fn can_encode_tuple_function_arguments(value_tuple in DynSolValue::type_strategy(&DynSolType::Tuple(vec![DynSolType::Address, DynSolType::Bytes, DynSolType::String, DynSolType::Uint(8)]))) {
            let function_argument = FunctionArgument {
                solidity_type: DynSolType::Tuple(vec![DynSolType::Address, DynSolType::Bytes, DynSolType::String, DynSolType::Uint(8)]).into(),
                solidity_value: value_tuple.clone()
            };

            let ethers_abi_token = function_argument.to_ethers_abi_token().unwrap();
            let ethers_abi_encoded = ethers::abi::encode(&[ethers_abi_token]);

            assert_eq!(ethers_abi_encoded, function_argument.encode_single());
        }
    }
}
