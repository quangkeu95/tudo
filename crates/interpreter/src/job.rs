use derive_more::{Deref, From};
use serde::Deserialize;

/// JobName can only contains alphanumeric characters, `_` or `-`
#[derive(Debug, Deref, Deserialize, From, Eq, PartialEq, Hash)]
pub struct JobName(String);

/// JobConfig is a abstract layer for Job
#[derive(Debug, Deserialize)]
pub struct JobConfig {}
