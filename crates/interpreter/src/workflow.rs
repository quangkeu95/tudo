use derive_more::{Deref, From};
use serde::Deserialize;

#[derive(Debug, Deref, Deserialize, From, Eq, PartialEq, Hash)]
pub struct WorkflowName(String);

#[derive(Debug, Deserialize)]
pub struct WorkflowConfig {}
