use std::collections::HashMap;

use crate::job::{JobConfig, JobName};
use derive_builder::Builder;
use serde::Deserialize;

/// Workflow configuration
#[derive(Debug, Builder, Clone)]
pub struct WorkflowConfig {
    jobs: HashMap<JobName, JobConfig>,
}

impl WorkflowConfig {
    pub fn get_jobs(&self) -> &HashMap<JobName, JobConfig> {
        &self.jobs
    }
}

/// Helper struct to help deserialize [`WorkflowConfig`]
#[derive(Debug)]
pub struct WorkflowConfigHelper {
    pub jobs: Vec<JobConfigInWorkflowEnum>,
}

impl<'de> Deserialize<'de> for WorkflowConfigHelper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct Helper {
            pub jobs: Vec<serde_value::Value>,
        }

        let helper = Helper::deserialize(deserializer)?;

        let jobs = helper
            .jobs
            .into_iter()
            .map(|value| match value.clone() {
                serde_value::Value::String(_str_value) => {
                    let job_name = JobName::deserialize(value).map_err(serde::de::Error::custom)?;
                    Ok(JobConfigInWorkflowEnum::JobName(job_name))
                }
                serde_value::Value::Map(_mapping) => {
                    let job_config_helper =
                        JobConfigHelper::deserialize(value).map_err(serde::de::Error::custom)?;
                    Ok(JobConfigInWorkflowEnum::JobConfig(job_config_helper))
                }
                _ => Err(serde::de::Error::custom(
                    "cannot deserialize workflow config helper",
                )),
            })
            .collect::<Result<Vec<JobConfigInWorkflowEnum>, D::Error>>()?;

        Ok(Self { jobs })
    }
}

#[derive(Debug, Deserialize)]
pub enum JobConfigInWorkflowEnum {
    JobName(JobName),
    JobConfig(JobConfigHelper),
}

#[derive(Debug, Deserialize)]
pub struct JobConfigHelper {
    pub name: JobName,
    pub depends_on: Option<Vec<JobName>>,
}
