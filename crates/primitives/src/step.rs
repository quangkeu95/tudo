#[derive(Debug)]
pub enum StepType {}

/// A step can be executed individually and can be chained to make a pipeline in a job.
#[async_trait::async_trait]
pub trait Step {
    type Input;
    type Output;
    type Error;

    async fn execute(&self, input: &Self::Input) -> eyre::Result<Self::Output, Self::Error>;
}
