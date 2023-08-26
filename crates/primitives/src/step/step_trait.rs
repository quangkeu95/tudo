use async_trait::async_trait;

/// A step can be executed individually and can be chained to make a pipeline in a job.
#[async_trait]
pub trait Step: Sized {
    type Input;
    type Output;
    type Error: std::error::Error;

    /// Execute the step and return result
    async fn execute(&self, input: &mut Self::Input) -> Result<Self::Output, Self::Error>;
}
