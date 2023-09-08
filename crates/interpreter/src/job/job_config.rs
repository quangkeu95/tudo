use derive_builder::Builder;
use serde::Deserialize;

use crate::{step::StepConfig, workflow::JobConfigHelper};

use super::JobName;

/// Job definition
#[derive(Debug, Builder, Deserialize, Clone)]
pub struct JobConfig {
    steps: Vec<StepConfig>,
    /// Prerequisited Job
    #[serde(skip)]
    depends_on: Vec<JobName>,
}

impl JobConfig {
    pub fn with_job_config_helper(&mut self, job_config_helper: &JobConfigHelper) {
        self.depends_on = job_config_helper.depends_on.clone().unwrap_or_default();
    }

    pub fn prerequisited_jobs(&self) -> &[JobName] {
        &self.depends_on
    }

    pub fn steps(&self) -> &[StepConfig] {
        &self.steps
    }
}
