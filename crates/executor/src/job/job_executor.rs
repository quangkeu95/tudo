use thiserror::Error;
use tudo_config::logging::{__tracing as tracing, info, instrument};
use tudo_interpreter::job::{JobConfig, JobName};

use crate::{
    job::JobContext,
    step::{ExecuteStepError, StepExecutor},
};

pub struct JobExecutor {}

impl JobExecutor {
    #[instrument(name = "JobExecute", skip(job_config))]
    pub async fn execute(
        job_name: &JobName,
        job_config: &JobConfig,
    ) -> Result<JobContext, ExecuteJobError> {
        info!("Executing job {:#?}", job_name);

        let mut job_context = JobContext::default();

        for step in job_config.steps() {
            StepExecutor::execute(step, &mut job_context).await?;
        }

        info!("Finish executing the job {:#?}", job_name);

        Ok(job_context)
    }
}

#[derive(Debug, Error)]
pub enum ExecuteJobError {
    #[error(transparent)]
    ExecuteStepError(#[from] ExecuteStepError),
}
