use std::{collections::HashMap, fs::File, io::Read, path::Path};

use crate::{
    job::{JobConfig, JobName},
    workflow::{WorkflowConfig, WorkflowName},
};
use serde::Deserialize;
use thiserror::Error;

use super::{Setup, Version};

/// Playbook configuration
#[derive(Debug, Deserialize)]
pub struct Playbook {
    pub version: Version,
    pub setup: Option<Setup>,
    pub jobs: HashMap<JobName, JobConfig>,
    pub workflows: HashMap<WorkflowName, WorkflowConfig>,
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
