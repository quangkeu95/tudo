use alloy_primitives::Address;
use ethers::{
    abi::{ParamType, Token},
    types::{BlockId, Bytes},
};
use serde::Deserialize;
use shared::{utils::build_calldata, CallContractBuilder, Step};

use crate::{
    alloy_converter::AlloyConverter,
    playbook::RpcProvider,
    step::StepArgumentTrait,
    types::{FunctionArgument, FunctionArgumentError, FunctionReturnTypes, FunctionSignature},
};

/// CallContract is a step arguments
#[derive(Debug, Deserialize, Clone)]
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
    /// Optional block
    pub block: Option<BlockId>,
    /// Function return types, could be defined as a single Solidity type or a Solidity tuple
    #[serde(default)]
    pub function_return_types: Option<Vec<FunctionReturnTypes>>,
}

impl CallContract {
    /// Return calldata in [`Bytes`]
    pub fn calldata(&self) -> Result<Bytes, FunctionArgumentError> {
        let tokens = self
            .function_arguments
            .iter()
            .map(|arg| arg.to_ethers_abi_token())
            .collect::<Result<Vec<Token>, FunctionArgumentError>>()?;

        let call_data = build_calldata(self.function_signature.as_ref(), &tokens);
        Ok(call_data)
    }

    /// Return function return in [`ParamType`] type
    pub fn as_function_return_param_types(&self) -> Option<Vec<ParamType>> {
        self.function_return_types
            .as_ref()
            .map(|vec_function_return_types| {
                vec_function_return_types
                    .iter()
                    .map(|item| item.as_ethers_param_type())
                    .collect::<Vec<ParamType>>()
            })
    }
}

impl StepArgumentTrait for CallContract {
    fn to_step(&self) -> Result<Box<dyn Step>, super::StepArgumentsError> {
        let contract_address = AlloyConverter::from_alloy_address(&self.contract_address);
        let calldata = self.calldata()?;
        let return_data_types = self.as_function_return_param_types();

        match &self.rpc_provider {
            RpcProvider::Http(provider)
            | RpcProvider::HttpWithBasicAuth(provider)
            | RpcProvider::HttpWithBearerAuth(provider) => {
                let call_contract_step = CallContractBuilder::default()
                    .middleware(provider.clone())
                    .contract_address(contract_address)
                    .calldata(calldata)
                    .block(self.block)
                    .return_data_types(return_data_types)
                    .build()?;
                Ok(Box::new(call_contract_step))
            }
            RpcProvider::Websocket(provider)
            | RpcProvider::WebsocketWithBasicAuth(provider)
            | RpcProvider::WebsocketWithBearerAuth(provider) => {
                let call_contract_step = CallContractBuilder::default()
                    .middleware(provider.clone())
                    .contract_address(contract_address)
                    .calldata(calldata)
                    .block(self.block)
                    .return_data_types(return_data_types)
                    .build()?;
                Ok(Box::new(call_contract_step))
            }
            RpcProvider::Ipc(provider) => {
                let call_contract_step = CallContractBuilder::default()
                    .middleware(provider.clone())
                    .contract_address(contract_address)
                    .calldata(calldata)
                    .block(self.block)
                    .return_data_types(return_data_types)
                    .build()?;
                Ok(Box::new(call_contract_step))
            }
        }
    }
}

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
            function_return_types: [uint256]
        "#;

        let _call_contract_step_argument: CallContract = serde_yaml::from_str(yaml).unwrap();
    }

    #[test]
    fn can_convert_call_contract_step_argument_to_step() {
        let yaml = r#"
            chain_rpc_url: "https://eth.llamarpc.com"
            contract_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
            function_signature: "balanceOf()"
            function_arguments:
                - type: address
                  value: "0x95Ba4cF87D6723ad9C0Db21737D862bE80e93911"
            function_return_types: [uint256]
        "#;

        let call_contract_step_argument: CallContract = serde_yaml::from_str(yaml).unwrap();
        let _step = call_contract_step_argument.to_step().unwrap();
    }
}
