use std::collections::BTreeSet;

use derive_more::Deref;
use serde::Deserialize;
use serde_valid::Validate;

use crate::job::JobName;

/// WorkflowName can only contains alphanumeric, `_` or `-` characters, up to a maximum of 200 characters.
#[derive(Debug, Deref, Deserialize, Validate, Eq, PartialEq, Hash, Clone)]
pub struct WorkflowName(#[validate(pattern = r#"^[a-zA-Z0-9_-]{1,200}$"#)] String);

#[derive(Debug, Deserialize, Clone)]
pub struct WorkflowConfig {
    pub jobs: Vec<JobName>,
}
