mod call_contract;
pub use call_contract::*;

use serde::Deserialize;
use shared::{BlankStep, CallContractBuilderError, Step};
use thiserror::Error;

use crate::types::FunctionArgumentError;

use super::StepArgumentTrait;

/// An enum represents all possible step definition
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum StepArguments {
    BlankStep,
    CallContract(CallContract),
}

impl StepArgumentTrait for StepArguments {
    fn to_step(&self) -> Result<Box<dyn Step>, StepArgumentsError> {
        match self {
            StepArguments::BlankStep => Ok(Box::<BlankStep>::default()),
            StepArguments::CallContract(inner) => inner.to_step(),
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
