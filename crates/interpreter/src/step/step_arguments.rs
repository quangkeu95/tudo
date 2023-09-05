mod call_contract;
pub use call_contract::*;

use serde::Deserialize;
use thiserror::Error;
use tudo_primitives::CallContractBuilderError;

use crate::types::FunctionArgumentError;

use super::StepArgumentTrait;

/// An enum represents all possible step definition
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum StepArguments {
    CallContract(CallContract),
}

impl StepArgumentTrait for StepArguments {
    fn as_step(&self) -> Result<Box<dyn tudo_primitives::Step>, StepArgumentsError> {
        match self {
            StepArguments::CallContract(inner) => inner.as_step(),
        }
    }
}

#[derive(Error, Debug)]
pub enum StepArgumentsError {
    #[error(transparent)]
    CallContractBuilderError(#[from] CallContractBuilderError),
    #[error(transparent)]
    FunctionArgumentError(#[from] FunctionArgumentError),
}
