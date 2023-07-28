//! Tudo configuration

pub mod utils;
use std::path::PathBuf;

pub use crate::utils::*;

/// Tudo configuration
#[derive(Debug)]
pub struct Config {}

impl Config {
    /// Default config file
    pub const FILE_NAME: &'static str = "tudo.toml";

    /// Default workflow file
    pub const WORKFLOW_FILE_NAME: &'static str = "workflow.yaml";

    /// By default workflow file will be `workflow.yaml` at project root.
    pub fn workflow_file(workflow_file: Option<PathBuf>) -> PathBuf {
        if let Some(workflow_file) = workflow_file {
            workflow_file
        } else {
            let project_root = find_project_root_path(None).unwrap();
            project_root.join(Self::WORKFLOW_FILE_NAME)
        }
    }
}
