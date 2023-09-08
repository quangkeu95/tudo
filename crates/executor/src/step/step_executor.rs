// use super::{CallContract, CallContractInputBuilder};
use ethers::providers::Provider;
use thiserror::Error;
use tudo_config::logging::{__tracing as tracing, info, instrument};
use tudo_interpreter::step::{StepConfig, StepConfigError, StepTypes};
use tudo_primitives::{Step, StepError};

use crate::job::{JobContext, JobContextError};

pub struct StepExecutor {}

impl StepExecutor {
    #[instrument(name = "StepExecute", skip_all)]
    pub async fn execute(
        step_config: &StepConfig,
        job_context: &mut JobContext,
    ) -> Result<(), ExecuteStepError> {
        info!("Execute step {:#?}", step_config.name);

        let step = step_config.to_step()?;
        let step_output = step.execute().await?;

        info!("Step output {:#?}", step_output);

        job_context.add_step_output(&step_config.name, step_output)?;

        info!("Finish executing step {:#?}", step_config.name);
        Ok(())
    }
}

/// Error happens during step execution
#[derive(Debug, Error)]
pub enum ExecuteStepError {
    #[error("build step input error {0}")]
    BuildStepInputError(String),
    #[error(transparent)]
    StepConfigError(#[from] StepConfigError),
    #[error(transparent)]
    StepError(#[from] StepError),
    #[error(transparent)]
    JobContextError(#[from] JobContextError),
}
