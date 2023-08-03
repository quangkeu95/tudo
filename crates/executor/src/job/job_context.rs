use std::collections::BTreeMap;
use thiserror::Error;

use crate::step::StepOutput;

#[derive(Debug)]
pub struct JobContext {
    pub step_outputs: BTreeMap<String, StepOutput>,
}

impl JobContext {
    pub fn new() -> Self {
        Self {
            step_outputs: BTreeMap::new(),
        }
    }

    pub fn add_step_output(
        &mut self,
        step_id: String,
        step_output: StepOutput,
    ) -> Result<(), JobContextError> {
        if self.step_outputs.contains_key(&step_id) {
            return Err(JobContextError::StepIdExisted(step_id));
        }
        self.step_outputs.insert(step_id, step_output);
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum JobContextError {
    #[error("Step ID is already existed: {0}")]
    StepIdExisted(String),
}
