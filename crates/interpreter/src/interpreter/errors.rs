use thiserror::Error;

/// Interpreter errors
#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Read workflow file error: {0}")]
    WorkflowFileError(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] serde_yaml::Error),
}
