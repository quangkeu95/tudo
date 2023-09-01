// use super::{CallContract, CallContractInputBuilder};
use ethers::providers::Provider;
use thiserror::Error;
use tudo_interpreter::step::{StepConfig, StepTypes};
use tudo_primitives::Step;

pub struct StepExecutor {}

impl StepExecutor {
    pub async fn execute(step: &StepConfig) -> Result<(), StepExecuteError> {
        // match step.step_type {
        // StepTypes::CallContract => {
        // let call_contract_input = CallContractInputBuilder::default()
        //     .build()
        //     .map_err(|e| StepExecuteError::BuildStepInputError(e.to_string()))?;
        // let provider = Provider::<Http>::try_from(step)
        // CallContract::new()
        // CallContract::execute().await;
        // }
        // }
        Ok(())
    }
}

/// Error happens during step execution
#[derive(Debug, Error)]
pub enum StepExecuteError {
    #[error("build step input error {0}")]
    BuildStepInputError(String),
}
