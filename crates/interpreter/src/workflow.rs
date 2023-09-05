use derive_builder::Builder;
use std::collections::HashMap;

use derive_more::Deref;
use serde::Deserialize;
use serde_valid::Validate;

use crate::job::{JobConfig, JobName};

/// WorkflowName can only contains alphanumeric, `_` or `-` characters, up to a maximum of 200 characters.
#[derive(Debug, Deref, Deserialize, Validate, Eq, PartialEq, Hash, Clone)]
pub struct WorkflowName(#[validate(pattern = r#"^[a-zA-Z0-9][a-zA-Z0-9_-]{1,199}$"#)] String);

#[derive(Debug, Builder, Deserialize)]
pub struct WorkflowConfig {
    jobs: HashMap<JobName, JobConfig>,
}

impl WorkflowConfig {
    pub fn get_jobs(&self) -> &HashMap<JobName, JobConfig> {
        &self.jobs
    }
}

#[derive(Debug, Deserialize)]
pub struct WorkflowConfigHelper {
    pub jobs: HashMap<JobName, Option<JobConfigHelper>>,
}

#[derive(Debug, Deserialize)]
pub struct JobConfigHelper {}
