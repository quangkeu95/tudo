use derive_more::{Deref, FromStr};
use serde::Deserialize;
use serde_valid::Validate;

/// WorkflowName can only contains alphanumeric, `_` or `-` characters, up to a maximum of 200 characters.
#[derive(Debug, Deref, Deserialize, Validate, Eq, PartialEq, Hash, Clone, FromStr)]
pub struct WorkflowName(#[validate(pattern = r#"^[a-zA-Z0-9][a-zA-Z0-9_-]{1,199}$"#)] String);
