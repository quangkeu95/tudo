use alloy_dyn_abi::DynSolValue;
use serde::{Deserialize, Deserializer};

use super::DynSolTypeWrapper;

/// Solidity function argument, which contains a Solidity type specifier and a Solidity value
#[derive(Debug)]
pub struct FunctionArgument {
    pub solidity_type: DynSolTypeWrapper,
    pub solidity_value: DynSolValue,
}

impl<'de> Deserialize<'de> for FunctionArgument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_yaml::Value::deserialize(deserializer)?;

        let solidity_type = value
            .get("type")
            .ok_or(serde::de::Error::missing_field("type"))?;
        let solidity_value = value
            .get("value")
            .ok_or(serde::de::Error::missing_field("value"))?;

        let solidity_type = DynSolTypeWrapper::deserialize(solidity_type).map_err(|e| {
            serde::de::Error::custom(format!("parse Solidity types error {:#?}", e))
        })?;

        let solidity_value: serde_json::Value = serde_yaml::from_value(solidity_value.to_owned())
            .map_err(|e| {
            serde::de::Error::custom(format!(
                "convert Solidity value from yaml to json error {:#?}",
                e
            ))
        })?;

        let solidity_value: DynSolValue = solidity_type.coerce(&solidity_value).map_err(|e| {
            serde::de::Error::custom(format!("parse Solidity value error {:#?}", e))
        })?;

        Ok(Self {
            solidity_type,
            solidity_value,
        })
    }
}

impl FunctionArgument {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy_dyn_abi::DynSolType;
    use alloy_primitives::U256;
    use claims::{assert_matches, assert_some};

    use super::FunctionArgument;

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

    // fn dyn_sol_type_array_strategy() -> BoxedStrategy<DynSolType> {
    //     prop_oneof![
    //         Just(DynSolType::Array(Box::new(DynSolType::Address))),
    //         Just(DynSolType::Array(Box::new(DynSolType::Bool))),
    //         Just(DynSolType::Array(Box::new(DynSolType::String))),
    //         Just(DynSolType::Array(Box::new(DynSolType::Bytes))),
    //         Just(DynSolType::Array(Box::new(DynSolType::Uint(256)))),
    //         Just(DynSolType::Array(Box::new(DynSolType::Int(256)))),
    //         Just(DynSolType::Array(Box::new(DynSolType::FixedBytes(256)))),
    //         Just(DynSolType::Array(Box::new(DynSolType::Array(Box::new(
    //             DynSolType::Address
    //         ))))),
    //     ]
    //     .boxed()
    // }

    // prop_compose! {
    //     fn dyn_sol_array_stategy(sol_type: &DynSolType)(sol_value in DynSolValue::type_strategy(sol_type)) -> String {
    //         let array = sol_value.as_array().unwrap();

    //         format!(
    //             r#"
    //             - type: {:}
    //               value: {:?}
    //         "#,
    //             sol_type.to_string()
    //         )
    //     }
    // }

    // proptest! {
    //     #[test]
    //     fn can_parse_array_to_function_arguments(sol_type in dyn_sol_type_array_strategy())(sol_value in DynSolValue::type_strategy(&sol_type)) {
    //         let function_arguments: Vec<FunctionArgument> = assert_ok!(serde_yaml::from_str(&array_value));
    //         println!("function arguments {:#?}", function_arguments);
    //     }
    // }
}
