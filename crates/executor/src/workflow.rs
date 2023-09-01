use thiserror::Error;
use tudo_config::logging::info;
use tudo_interpreter::workflow::{WorkflowConfig, WorkflowName};

pub struct WorkflowExecutor {}

impl WorkflowExecutor {
    pub fn execute(
        workflow_name: &WorkflowName,
        workflow: &WorkflowConfig,
    ) -> Result<(), WorkflowExecuteError> {
        info!("execute workflow {:#?}", workflow_name);
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum WorkflowExecuteError {}
