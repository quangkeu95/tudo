use std::{fs::File, io::Read, path::PathBuf};
use tudo_common::WorkflowCompose;

use crate::types::config::WorkflowComposeConfig;

mod errors;
pub use errors::*;

/// Build the workflow and return [WorkflowCompose]
pub fn build_workflow(workflow_file: PathBuf) -> eyre::Result<WorkflowCompose> {
    let workflow_compose_config = Interpreter::from_workflow_file(workflow_file)?;

    Ok(workflow_compose_config.into())
}

#[derive(Debug)]
pub struct Interpreter {}

impl Interpreter {
    /// Parse workflow file into [WorkflowComposeConfig] type
    pub fn from_workflow_file(
        workflow_file: PathBuf,
    ) -> Result<WorkflowComposeConfig, InterpreterError> {
        let mut file = File::open(workflow_file)?;

        // Read the file content into a string
        let mut yaml_content = String::new();
        file.read_to_string(&mut yaml_content)?;

        // Deserialize the YAML content into your Config struct
        let config: WorkflowComposeConfig = serde_yaml::from_str(&yaml_content)?;

        Ok(config)
    }
}
