use alloy_primitives::Address;
use serde::Deserialize;

use crate::{
    playbook::RpcProvider,
    types::{FunctionArgument, FunctionReturnTypes, FunctionSignature},
};

/// CallContract is a step arguments
#[derive(Debug, Deserialize)]
pub struct CallContract {
    /// Chain rpc url
    // pub chain_rpc_url: Url,
    #[serde(flatten)]
    pub rpc_provider: RpcProvider,
    /// Contract address in hex string
    pub contract_address: Address,
    /// Function signature example: `setOwner(address)`
    pub function_signature: FunctionSignature,
    /// Function arguments
    pub function_arguments: Vec<FunctionArgument>,
    /// Function return types, could be defined as a single Solidity type or a Solidity tuple
    #[serde(default)]
    pub function_return_types: FunctionReturnTypes,
}

// impl StepArgumentsTrait for CallContract {
//     type Step = CallContractStep<M>;

//     fn to_step_input(&self) -> eyre::Result<<Self::Step as tudo_primitives::Step>::Input> {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize_call_contract_as_step_arguments() {
        let yaml = r#"
            chain_rpc_url: "https://eth.llamarpc.com"
            contract_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
            function_signature: "balanceOf()"
            function_arguments:
                - type: address
                  value: "0x95Ba4cF87D6723ad9C0Db21737D862bE80e93911"
            function_return_types: uint256
        "#;

        let _call_contract_step_argument: CallContract = serde_yaml::from_str(yaml).unwrap();

        let yaml = r#"
            chain_rpc_url: "https://eth.llamarpc.com"
            contract_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
            function_signature: "balanceOf()"
            function_arguments:
                - type: address
                  value: "0x95Ba4cF87D6723ad9C0Db21737D862bE80e93911"
            function_return_types: (uint256)
        "#;

        let _call_contract_step_argument: CallContract = serde_yaml::from_str(yaml).unwrap();
    }
}
