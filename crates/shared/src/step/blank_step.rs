use crate::{Step, StepError, StepOutput};

/// A blank step take no input and produce nothing
#[derive(Debug, Default)]
pub struct BlankStep {}

#[async_trait::async_trait]
impl Step for BlankStep {
    async fn execute(&self) -> Result<StepOutput, StepError> {
        Ok(StepOutput::None)
    }
}
