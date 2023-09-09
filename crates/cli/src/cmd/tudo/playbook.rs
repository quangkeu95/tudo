//! Playbook command
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::path::PathBuf;
use tudo_config::logging::{__tracing as tracing, info, instrument};
use tudo_executor::playbook::PlaybookExecutor;
use tudo_interpreter::playbook::Playbook;

use crate::cmd::utils::AsyncCmd;

mod core;
pub use self::core::*;

/// CLI arguments for `tudo playbook`.
#[derive(Debug, Parser, Serialize)]
#[clap(next_help_heading = "Playbook options", about = None, long_about = None)]
pub struct PlaybookArgs {
    /// Playbook file path.
    #[serde(skip)]
    pub playbook_file: PathBuf,
}

#[async_trait::async_trait]
impl AsyncCmd for PlaybookArgs {
    type Output = ();

    /// Parse and run playbook
    #[instrument(name = "PlaybookCliRun", skip_all)]
    async fn run(self) -> eyre::Result<Self::Output> {
        info!("Running playbook at {:#?}", &self.playbook_file.green());
        let playbook = Playbook::from_file(self.playbook_file)?;

        PlaybookExecutor::run(playbook).await?;
        Ok(())
    }
}
