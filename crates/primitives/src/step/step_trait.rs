use crate::{StepError, StepOutput};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

/// A step can be executed individually and can be chained to make a pipeline in a job.
#[async_trait]
#[enum_dispatch]
pub trait Step {
    /// Execute the step and return result
    async fn execute(&self) -> Result<StepOutput, StepError>;
}
