use futures::stream::StreamExt;
use thiserror::Error;
use tokio::task::JoinHandle;
use tudo_interpreter::playbook::{Playbook, Version};

use crate::workflow::{WorkflowExecuteError, WorkflowExecutor};

pub struct Executor {}

impl Executor {
    /// Execute the playbook and produce outputs
    pub async fn run(playbook: Playbook) -> Result<(), ExecutorError> {
        match &playbook.version {
            Version::V1 => Self::run_v1(playbook).await,
            #[allow(unreachable_patterns)]
            other => Err(ExecutorError::PlaybookVersionNotSupported(
                other.to_string(),
            )),
        }
    }

    async fn run_v1(playbook: Playbook) -> Result<(), ExecutorError> {
        if let Some(setup) = &playbook.setup {
            // TODO: execute setup
        };

        // each workflow is executed separately
        // let workflows_handles =
        //     playbook
        //         .workflows
        //         .iter()
        //         .map(|(workflow_name, workflow_config)| {
        //             let workflow_name = workflow_name.clone();
        //             let workflow_config = workflow_config.clone();

        //             tokio::spawn(async move {
        //                 WorkflowExecutor::execute(&workflow_name, &workflow_config)
        //             })
        //         });
        // let buffer_len = workflows_handles.len();

        // let mut workflows_stream =
        //     futures::stream::iter(workflows_handles).buffer_unordered(buffer_len);

        // while let Some(workflow_result) = workflows_stream.next().await {
        //     match workflow_result {
        //         Ok(result) => {}
        //         Err(e) => {}
        //     }
        // }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("playbook version is not supported. Got {:#?}", .0)]
    PlaybookVersionNotSupported(String),
}
