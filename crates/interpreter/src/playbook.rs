use std::{collections::HashMap, fs::File, io::Read, path::Path};

use serde::Deserialize;
use thiserror::Error;

mod version;
pub use version::*;
mod setup;
pub use setup::*;

use crate::{
    job::{JobConfig, JobName},
    workflow::{WorkflowConfig, WorkflowName},
};

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
