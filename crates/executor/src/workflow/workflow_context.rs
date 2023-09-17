use std::{collections::HashMap, sync::Arc};

use futures::lock::Mutex;
use thiserror::Error;
use tokio::sync::broadcast::{Receiver, Sender};
use interpreter::job::JobName;

use crate::job::JobContext;

#[derive(Default)]
pub struct WorkflowContext {
    job_contexts: HashMap<JobName, JobContext>,
    pending_jobs: HashMap<JobName, Sender<()>>,
}

pub type SharedMutableWorkflowContext = Arc<Mutex<WorkflowContext>>;

impl WorkflowContext {
    pub fn new_share_mutable() -> SharedMutableWorkflowContext {
        Arc::new(Mutex::new(WorkflowContext::default()))
    }

    pub fn add_job_context(
        &mut self,
        job_name: &JobName,
        job_context: JobContext,
    ) -> Result<(), WorkflowContextError> {
        if self.job_contexts.contains_key(job_name) {
            return Err(WorkflowContextError::JobExisted(job_name.clone()));
        }
        self.job_contexts.insert(job_name.clone(), job_context);

        Ok(())
    }

    pub async fn wait_for_job_to_finish(
        &self,
        job_name: &JobName,
    ) -> Result<Receiver<()>, WorkflowContextError> {
        let sender = self
            .pending_jobs
            .get(job_name)
            .ok_or(WorkflowContextError::NoPendingJob(job_name.clone()))?;

        Ok(sender.subscribe())
    }

    pub fn add_pending_job(
        &mut self,
        job_name: &JobName,
        sender: Sender<()>,
    ) -> Result<(), WorkflowContextError> {
        if self.pending_jobs.contains_key(job_name) {
            return Err(WorkflowContextError::JobExisted(job_name.clone()));
        }
        self.pending_jobs.insert(job_name.clone(), sender);

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum WorkflowContextError {
    #[error("job {:#?} existed", .0)]
    JobExisted(JobName),
    #[error("no pending job {:#?}", .0)]
    NoPendingJob(JobName),
}
