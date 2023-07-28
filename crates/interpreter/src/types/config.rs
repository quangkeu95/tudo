use derive_more::Deref;
use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;
use std::collections::HashMap;
use tudo_common::WorkflowCompose;

/// Workflow file configuration mapping
#[derive(Debug, Deserialize)]
pub struct WorkflowComposeConfig {
    pub version: Version,
    pub setup: Option<Setup>,
    pub jobs: HashMap<JobName, JobConfig>,
    pub workflows: HashMap<WorkflowName, WorkflowConfig>,
}

impl From<WorkflowComposeConfig> for WorkflowCompose {
    fn from(value: WorkflowComposeConfig) -> Self {
        WorkflowCompose {}
    }
}

#[derive(Debug, Deserialize_enum_str)]
pub enum Version {
    #[serde(rename = "1")]
    V1,
}

/// Workflow setup configuration
#[derive(Debug, Deserialize)]
pub struct Setup {}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Deref)]
pub struct JobName(String);

#[derive(Debug, Deserialize)]
pub struct JobConfig {}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Deref)]
pub struct WorkflowName(String);

#[derive(Debug, Deserialize)]
pub struct WorkflowConfig {}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::*;

    #[test]
    fn can_parse_workflow_compose_file() {
        let file_content = r#"
            version: '1'
            setup:
            jobs:
            workflows:
        "#;

        let config: WorkflowComposeConfig = assert_ok!(serde_yaml::from_str(file_content));
        assert_matches!(config.version, Version::V1);
    }
}
