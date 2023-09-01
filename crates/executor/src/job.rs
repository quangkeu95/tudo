use thiserror::Error;
use tudo_interpreter::job::JobConfig;

pub struct JobExecutor {}

impl JobExecutor {
    pub fn execute(job: &JobConfig) -> Result<(), JobExecuteError> {
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum JobExecuteError {}
