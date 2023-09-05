use crate::{StepError, StepOutput};
use async_trait::async_trait;

/// A step can be executed individually and can be chained to make a pipeline in a job.
#[async_trait]
pub trait Step {
    /// Execute the step and return result
    async fn execute(&self) -> Result<StepOutput, StepError>;
}
