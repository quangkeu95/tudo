use super::{PlaybookContextBuilder, PlaybookContextBuilderError};
use crate::workflow::WorkflowExecutor;
use thiserror::Error;
use config::logging::{__tracing as tracing, error, instrument};
use interpreter::playbook::{Playbook, Version};

pub struct PlaybookExecutor {}

impl PlaybookExecutor {
    /// Execute the playbook and produce outputs.
    /// Workflows in the playbook are executed in parallel, or sequentially by depending on each others.
    pub async fn run(playbook: Playbook) -> Result<(), PlaybookExecutorError> {
        match playbook.version() {
            Version::V1 => Self::run_v1(playbook).await,
            #[allow(unreachable_patterns)]
            other => Err(PlaybookExecutorError::PlaybookVersionNotSupported(
                other.to_string(),
            )),
        }
    }

    /// Execute the playbook version 1
    #[instrument(name = "PlaybookExecutorV1", skip_all)]
    async fn run_v1(playbook: Playbook) -> Result<(), PlaybookExecutorError> {
        let shared_setup = playbook.shared_setup();
        let playbook_context = PlaybookContextBuilder::default()
            .shared_setup(shared_setup)
            .build()?
            .into_shared_mutex();

        let workflows = playbook.shared_workflows();
        // let workflows_num = workflows.len();

        let tasks = workflows.iter().map(|(workflow_name, workflow_config)| {
            tokio::spawn({
                let workflow_name = workflow_name.clone();
                let workflow_config = workflow_config.clone();
                let playbook_context = playbook_context.clone();

                async move {
                    WorkflowExecutor::execute(workflow_name, workflow_config, playbook_context)
                        .await
                }
            })
        });
        futures::future::join_all(tasks).await;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum PlaybookExecutorError {
    #[error("playbook version is not supported. Got {:#?}", .0)]
    PlaybookVersionNotSupported(String),
    #[error(transparent)]
    PlaybookContextBuilderError(#[from] PlaybookContextBuilderError),
    #[error(transparent)]
    TokioTaskJoinError(#[from] tokio::task::JoinError),
}
