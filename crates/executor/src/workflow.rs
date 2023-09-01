use thiserror::Error;
use tudo_interpreter::workflow::{WorkflowConfig, WorkflowName};

pub struct WorkflowExecutor {}

impl WorkflowExecutor {
    pub fn execute(
        workflow_name: &WorkflowName,
        workflow: &WorkflowConfig,
    ) -> Result<(), WorkflowExecuteError> {
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum WorkflowExecuteError {}
