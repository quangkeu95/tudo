//! Tudo configuration

pub mod utils;
pub use crate::utils::*;
pub mod logging;

/// Tudo configuration
#[derive(Debug)]
pub struct Config {}

impl Config {
    /// Default config file
    pub const FILE_NAME: &'static str = "tudo.toml";

    /// Default workflow file
    pub const PLAYBOOK_FILE_NAME: &'static str = "playbook.yaml";
}
