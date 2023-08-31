use serde::Deserialize;

use crate::step::StepConfig;

mod job_name;
pub use job_name::*;

/// JobConfig is a abstract layer for Job
#[derive(Debug, Deserialize)]
pub struct JobConfig {
    pub steps: Vec<StepConfig>,
}
