use serde::Deserialize;
use serde_valid::Validate;
use thiserror::Error;
use tudo_primitives::Step;

use crate::step::CallContract;

use super::{
    StepArgumentTrait, StepArguments, StepArgumentsError, StepName, StepOutput, StepTypes,
};

/// Step definition
#[derive(Debug, Validate, Clone)]
pub struct StepConfig {
    pub step_type: StepTypes,
    pub name: StepName,
    pub description: Option<String>,
    pub arguments: StepArguments,
    pub output: Option<StepOutput>,
}

impl StepConfig {
    pub fn to_step(&self) -> Result<Box<dyn Step>, StepConfigError> {
        self.arguments.to_step().map_err(StepConfigError::from)
    }
}

impl<'de> Deserialize<'de> for StepConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct StepConfigHelper {
            #[serde(alias = "type")]
            step_type: StepTypes,
            name: Option<StepName>,
            description: Option<String>,
            arguments: Option<serde_value::Value>,
            output: Option<serde_value::Value>,
        }

        let helper = StepConfigHelper::deserialize(deserializer)?;

        // give step a randomize name if user doesn't specific the step name
        let name = helper
            .name
            .unwrap_or(StepName::random_with_prefix(helper.step_type.to_string()));

        let step_arguments = match helper.step_type {
            StepTypes::BlankStep => StepArguments::BlankStep,
            StepTypes::CallContract => {
                let arguments = helper
                    .arguments
                    .ok_or(serde::de::Error::custom("missing field `arguments`"))?;

                CallContract::deserialize(arguments)
                    .map(StepArguments::CallContract)
                    .map_err(serde::de::Error::custom)?
            }
        };

        let step_output = match helper.step_type {
            StepTypes::CallContract => {
                let output = helper
                    .output
                    .ok_or(serde::de::Error::custom("missing field `output`"))?;
                Some(StepOutput::deserialize(output).map_err(serde::de::Error::custom)?)
            }
            _ => None,
        };

        Ok(Self {
            step_type: helper.step_type,
            name,
            description: helper.description,
            arguments: step_arguments,
            output: step_output,
        })
    }
}

#[derive(Debug, Error)]
pub enum StepConfigError {
    #[error(transparent)]
    StepArgumentsError(#[from] StepArgumentsError),
}

#[cfg(test)]
mod tests {
    use claims::assert_err;

    use super::*;

    #[test]
    fn can_parse_call_contract_step_config() {
        let content = r#"
            type: CallContract
            name: "Valid_Step_Name_Foo"
            description: "Here is the valid description Foo"
            arguments:
                chain_rpc_url: "https://eth.llamarpc.com"
                contract_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
                function_signature: "balanceOf()"
                function_arguments:
                    - type: address
                      value: "0x95Ba4cF87D6723ad9C0Db21737D862bE80e93911"
                function_return_types: uint256
            output:
                save_as: BALANCE_OF_0x95Ba4cF87D6723ad9C0Db21737D862bE80e93911
        "#;

        let step_configs: StepConfig = serde_yaml::from_str(content).unwrap();
    }

    #[test]
    fn should_return_error_when_parse_invalid_step_config() {
        let yaml = r#"
            type: CallContract
            name: "Valid_Step_Name_Foo"
            description: "Here is the valid description Foo"
        "#;

        assert_err!(serde_yaml::from_str::<StepConfig>(yaml));
    }
}
