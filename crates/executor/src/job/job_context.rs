use std::collections::HashMap;

use thiserror::Error;
use tudo_interpreter::step::StepName;
use tudo_primitives::StepOutput;

#[derive(Clone, Default)]
pub struct JobContext {
    step_outputs: HashMap<StepName, StepOutput>,
}

impl JobContext {
    /// Add step output to job context
    pub fn add_step_output(
        &mut self,
        step_name: &StepName,
        step_output: StepOutput,
    ) -> Result<(), JobContextError> {
        if !self.step_outputs.contains_key(step_name) {
            self.step_outputs.insert(step_name.clone(), step_output);
            Ok(())
        } else {
            Err(JobContextError::StepExisted(step_name.clone()))
        }
    }
}

#[derive(Debug, Error)]
pub enum JobContextError {
    #[error("step with name {:#?} already existed", .0)]
    StepExisted(StepName),
}
