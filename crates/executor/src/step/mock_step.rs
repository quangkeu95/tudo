use std::marker::PhantomData;
use thiserror::Error;

use tudo_primitives::Step;

/// Mock step stores only an inner type and return the value of that type on executed
#[derive(Debug)]
pub struct MockStep {}

impl MockStep {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Step for MockStep {
    type Input = String;
    type Output = String;
    type Error = MockError;

    async fn execute(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(input.clone())
    }
}

#[derive(Debug, Error)]
pub enum MockError {}
