/// A step can be executed individually and can be chained to make a pipeline in a job.
#[async_trait::async_trait]
pub trait Step {
    type Input;
    type Output;
    type Error: std::error::Error;

    /// Return step's ID
    fn id(&self) -> Option<String> {
        None
    }

    /// Execute the step and return result
    async fn execute(&self, input: &Self::Input) -> Result<Self::Output, Self::Error>;
}
