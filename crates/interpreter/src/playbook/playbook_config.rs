use crate::{
    job::{JobConfig, JobName},
    workflow::{
        JobConfigInWorkflowEnum, WorkflowConfig, WorkflowConfigBuilder, WorkflowConfigBuilderError,
        WorkflowConfigHelper, WorkflowName,
    },
};
use derive_builder::Builder;
use handlebars::Handlebars;
use serde::Deserialize;
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

impl TryFrom<PlaybookDeserializeHelper> for Playbook {
    type Error = PlaybookDeserializeHelperError;
    fn try_from(value: PlaybookDeserializeHelper) -> Result<Self, Self::Error> {
        let setup = value.setup.map(Arc::new);
        let jobs = Arc::new(value.jobs);

        let mut workflows = HashMap::new();

        for (workflow_name, workflow_config_helper) in value.workflows {
            let job_config_mapping = workflow_config_helper
                .jobs
                .iter()
                .map(|item| match item {
                    JobConfigInWorkflowEnum::JobName(job_name) => {
                        let job_config = jobs
                            .get(job_name)
                            .ok_or(PlaybookDeserializeHelperError::JobNotDefined(
                                job_name.clone(),
                            ))?
                            .clone();

                        Ok((job_name.clone(), job_config))
                    }

                    JobConfigInWorkflowEnum::JobConfig(job_config_helper) => {
                        let job_name = job_config_helper.name.clone();
                        let mut job_config = jobs
                            .get(&job_name)
                            .ok_or(PlaybookDeserializeHelperError::JobNotDefined(
                                job_name.clone(),
                            ))?
                            .clone();

                        job_config.with_job_config_helper(job_config_helper);

                        Ok((job_name, job_config))
                    }
                })
                .collect::<Result<HashMap<JobName, JobConfig>, Self::Error>>()?;

            let workflow_config = WorkflowConfigBuilder::default()
                .jobs(job_config_mapping)
                .build()?;

            workflows.insert(workflow_name, workflow_config);
        }

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

        // preprocess
        let content = Self::preprocess(&content)?;

        let playbook: Playbook = serde_yaml::from_str(&content)?;

        Ok(playbook)
    }

    /// Preprocess will replace variable references with handlebars syntax with pre-defined variables in the config setup
    fn preprocess<S>(content: S) -> Result<String, PlaybookError>
    where
        S: AsRef<str>,
    {
        let content = content.as_ref();

        #[derive(Debug, Deserialize)]
        struct SetupHelper {
            setup: Option<Setup>,
        }
        let helper: SetupHelper = serde_yaml::from_str(content)?;
        dbg!(&helper);
        if let Some(setup) = helper.setup {
            let mut handlebars = Handlebars::new();

            handlebars.set_strict_mode(true);

            if let Some(variables) = &setup.variables {
                handlebars.register_template_string("variables", content)?;
                let rendered = handlebars.render("variables", variables)?;
                return Ok(rendered);
            }
        }

        Ok(content.to_string())
    }

    /// Get playbook version
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Get shared setup config
    pub fn shared_setup(&self) -> Option<Arc<Setup>> {
        self.setup.as_ref().cloned()
    }

    /// Get shared workflows HashMap
    pub fn shared_workflows(&self) -> Arc<HashMap<WorkflowName, WorkflowConfig>> {
        self.workflows.clone()
    }

    /// Get shared jobs HashMap
    pub fn shared_jobs(&self) -> Arc<HashMap<JobName, JobConfig>> {
        self.jobs.clone()
    }
}

#[derive(Debug, Error)]
pub enum PlaybookError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerdeYamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    HandlebarsTemplateError(#[from] handlebars::TemplateError),
    #[error(transparent)]
    HandlebarsRenderError(#[from] handlebars::RenderError),
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
                uniswap_v3_eth_usdc_3000_pool_address:
                    steps:
                      - type: CallContract
                        name: "Get ETH/USDC 0.3% fee pool address"
                        arguments:
                            chain_rpc_url: "https://eth.llamarpc.com"
                            contract_address: "0x1F98431c8aD98523631AE4a59f267346ea31F984"
                            function_signature: "getPool(address,address,uint24)"
                            function_arguments:
                                - type: address
                                  value: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
                                - type: address
                                  value: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
                                - type: uint24
                                  value: 3000
                            function_return_types: [address]
                        output:
                            save_as: ETH_USDC_3000_BPS_POOL_ADDRESS
            workflows:
                workflow_1:
                    jobs:
                    - uniswap_v3_eth_usdc_3000_pool_address
        "#;

        let playbook: Playbook = serde_yaml::from_str(yaml).unwrap();
        assert_matches!(playbook.version, Version::V1);
    }

    #[test]
    fn can_preprocess_playbook() {
        let yaml = r#"
            version: "1"
            setup:
                variables:
                    ETH_RPC_URL: https://eth.llamarpc.com
                    UNISWAP_V3_FACTORY: "0x1F98431c8aD98523631AE4a59f267346ea31F984"
                    WETH_ADDRESS: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
                    USDC_ADDRESS: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
            jobs:
                uniswap_v3_eth_usdc_3000_pool_address:
                    steps:
                      - type: CallContract
                        name: "Get ETH/USDC 0.3% fee pool address"
                        arguments:
                            chain_rpc_url: {{ETH_RPC_URL}}
                            contract_address: {{UNISWAP_V3_FACTORY}}
                            function_signature: "getPool(address,address,uint24)"
                            function_arguments:
                                - type: address
                                  value: {{WETH_ADDRESS}}
                                - type: address
                                  value: {{USDC_ADDRESS}}
                                - type: uint24
                                  value: 3000
                            function_return_types: [address]
                        output:
                            save_as: ETH_USDC_3000_BPS_POOL_ADDRESS
            workflows:
                workflow_1:
                    jobs:
                    - uniswap_v3_eth_usdc_3000_pool_address
        "#;

        let _processed_content = Playbook::preprocess(yaml).unwrap();
    }
}
