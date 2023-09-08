use derive_more::{Deref, From};
use serde::Deserialize;
use serde_valid::Validate;
use uuid::Uuid;

/// StepName can only contains alphanumeric, `-`, `_` characters, up to a maximum of 200 characters.
#[derive(Debug, Clone, Deref, Deserialize, From, Validate, Eq, PartialEq, Hash)]
pub struct StepName(#[validate(pattern = r#"^[a-zA-Z0-9][a-zA-Z0-9_-]{1,199}$"#)] String);

impl StepName {
    /// Generate random StepName with prefix
    pub fn random_with_prefix<P: Into<String>>(prefix: P) -> Self {
        let randomize = Uuid::new_v4();
        let randomize_with_prefix = format!("{:}_{:}", prefix.into(), randomize);
        Self(randomize_with_prefix)
    }
}
