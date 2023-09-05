use crate::{
    job::{JobConfig, JobName},
    workflow::{
        WorkflowConfig, WorkflowConfigBuilder, WorkflowConfigBuilderError, WorkflowConfigHelper,
        WorkflowName,
    },
};
use derive_builder::Builder;
use serde::{Deserialize, Deserializer};
use std::{collections::HashMap, fs::File, io::Read, path::Path, sync::Arc};
use thiserror::Error;

use super::{Setup, Version};

/// Helper struct when deserializing Playbook
#[derive(Debug, Deserialize)]
pub struct PlaybookDeserializeHelper {
    pub version: Version,
    pub setup: Option<Setup>,
    pub jobs: HashMap<JobName, JobConfig>,
    pub workflows: HashMap<WorkflowName, WorkflowConfigHelper>,
}

impl PlaybookDeserializeHelper {
    pub fn validate(&self) -> Result<(), PlaybookDeserializeHelperError> {
        for workflow_config in self.workflows.values() {
            for job_name in workflow_config.jobs.keys() {
                if !self.jobs.contains_key(job_name) {
                    return Err(PlaybookDeserializeHelperError::JobNotDefined(
                        job_name.clone(),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl TryFrom<PlaybookDeserializeHelper> for Playbook {
    type Error = PlaybookDeserializeHelperError;
    fn try_from(value: PlaybookDeserializeHelper) -> Result<Self, Self::Error> {
        // value.validate()?;

        let setup = value.setup.map(Arc::new);
        let jobs = Arc::new(value.jobs);

        let workflows = value
            .workflows
            .into_iter()
            .map(|(workflow_name, workflow_config_helper)| {
                let jobs = workflow_config_helper
                    .jobs
                    .into_iter()
                    .map(|(job_name, _job_config_helper)| {
                        let job_config = jobs.get(&job_name).ok_or(
                            PlaybookDeserializeHelperError::JobNotDefined(job_name.clone()),
                        )?;
                        let job_config = job_config.clone();

                        // TODO: append job_config_helper settings into job_config
                        Ok((job_name, job_config))
                    })
                    .collect::<Result<HashMap<JobName, JobConfig>, PlaybookDeserializeHelperError>>(
                    )?;

                let workflow_config = WorkflowConfigBuilder::default().jobs(jobs).build()?;
                Ok((workflow_name, workflow_config))
            })
            .collect::<Result<HashMap<WorkflowName, WorkflowConfig>, PlaybookDeserializeHelperError>>()?;

        let workflows = Arc::new(workflows);

        let playbook = PlaybookBuilder::default()
            .version(value.version)
            .setup(setup)
            .jobs(jobs)
            .workflows(workflows)
            .build()?;

        Ok(playbook)
    }
}

#[derive(Debug, Error)]
pub enum PlaybookDeserializeHelperError {
    #[error("job is not defined {:#?}", .0)]
    JobNotDefined(JobName),
    #[error(transparent)]
    WorkflowConfigBuidlerError(#[from] WorkflowConfigBuilderError),
    #[error(transparent)]
    PlaybookBuilderError(#[from] PlaybookBuilderError),
}

/// Playbook configuration
#[derive(Debug, Builder, Deserialize)]
#[serde(try_from = "PlaybookDeserializeHelper")]
pub struct Playbook {
    version: Version,
    setup: Option<Arc<Setup>>,
    jobs: Arc<HashMap<JobName, JobConfig>>,
    workflows: Arc<HashMap<WorkflowName, WorkflowConfig>>,
}

impl Playbook {
    /// Parse Playbook from file
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, PlaybookError> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let playbook: Playbook = serde_yaml::from_str(&content)?;

        Ok(playbook)
    }

    /// Deserialize setup into shared setup type
    // fn deserialize_setup<'de, D>(deserializer: D) -> Result<Option<Arc<Setup>>, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     let value = Option::<Setup>::deserialize(deserializer)?;
    //     Ok(value.map(Arc::new))
    // }

    /// Get playbook version
    pub fn get_version(&self) -> &Version {
        &self.version
    }

    /// Get shared setup config
    pub fn get_shared_setup(&self) -> Option<Arc<Setup>> {
        self.setup.as_ref().map(|item| item.clone())
    }

    /// Get shared workflows HashMap
    pub fn get_shared_workflows(&self) -> Arc<HashMap<WorkflowName, WorkflowConfig>> {
        self.workflows.clone()
    }

    /// Get shared jobs HashMap
    pub fn get_shared_jobs(&self) -> Arc<HashMap<JobName, JobConfig>> {
        self.jobs.clone()
    }
}

#[derive(Debug, Error)]
pub enum PlaybookError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerdeYamlError(#[from] serde_yaml::Error),
}

#[cfg(test)]
mod tests {
    use claims::assert_matches;

    use super::*;

    #[test]
    fn can_deserialize_playbook() {
        let yaml = r#"
            version: "1"
            jobs:
                get_uniswap_v2_pair_at_index_0:
                    steps:
                      - type: CallContract
                        name: "Get UniwapV2Pair address from factory at index 0"
                        arguments:
                            chain_rpc_url: "https://eth.llamarpc.com"
                            contract_address: "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc"
                            function_signature: "allPairs(uint256)"
                            function_arguments:
                                - type: uint256
                                  value: 0
                        output:
                            save_as: ALL_PAIRS
            workflows:
                workflow_1:
                    jobs:
                    - get_uniswap_v2_pair_at_index_0
        "#;

        let playbook: Playbook = serde_yaml::from_str(yaml).unwrap();
        assert_matches!(playbook.version, Version::V1);
    }
}
