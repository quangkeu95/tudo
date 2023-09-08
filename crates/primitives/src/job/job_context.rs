use std::collections::{BTreeSet, HashMap};

use thiserror::Error;

use crate::StepOutputEnum;

/// Storing Job context in a single place
#[derive(Debug)]
pub struct JobContext {
    pub step_names: BTreeSet<String>,
    pub step_outputs: HashMap<String, StepOutputEnum>,
}

impl JobContext {
    /// Construct a new JobContext
    pub fn new() -> Self {
        JobContext {
            step_names: BTreeSet::new(),
            step_outputs: HashMap::new(),
        }
    }

    pub fn add_step_name<StepName: Into<String>>(
        &mut self,
        step_name: StepName,
    ) -> Result<(), JobContextError> {
        let step_name: String = step_name.into();
        if !self.step_names.insert(step_name.clone()) {
            return Err(JobContextError::StepNameExisted(step_name));
        } else {
            Ok(())
        }
    }
    /// Insert a [`StepOutputEnum`] into a JobContext with unique step name. Return error if step name is existed.
    pub fn add_step_output<T: Into<String>>(
        &mut self,
        step_name: T,
        step_output: StepOutputEnum,
    ) -> Result<(), JobContextError> {
        let key: String = step_name.into();
        if self.step_outputs.contains_key(&key) {
            return Err(JobContextError::StepNameExisted(key));
        } else {
            self.step_outputs.insert(key, step_output);
            Ok(())
        }
    }
}

/// Job context error
#[derive(Debug, Error)]
pub enum JobContextError {
    #[error("step name already existed {0}")]
    StepNameExisted(String),
}
