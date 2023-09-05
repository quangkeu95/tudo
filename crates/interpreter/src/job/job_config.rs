use serde::Deserialize;

use crate::step::StepConfig;

use super::JobName;

/// Job definition
#[derive(Debug, Deserialize, Clone)]
pub struct JobConfig {
    steps: Vec<StepConfig>,
    /// Prerequisited Job
    depends_on: Vec<JobName>,
}

impl JobConfig {
    pub fn prerequisited_jobs(&self) -> &[JobName] {
        &self.depends_on
    }

    pub fn steps(&self) -> &[StepConfig] {
        &self.steps
    }
}
