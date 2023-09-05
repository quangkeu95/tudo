use serde::Deserialize;

use crate::types::VariableName;

#[derive(Debug, Clone, Deserialize)]
pub struct StepOutput {
    /// Save step output with a variable name
    pub save_as: VariableName,
}
