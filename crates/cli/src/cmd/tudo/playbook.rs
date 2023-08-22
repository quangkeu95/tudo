//! Playbook command
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::path::PathBuf;
use tudo_executor::playbook::Executor;
use tudo_interpreter::playbook::Playbook;

use crate::cmd::utils::Cmd;

mod core;
pub use self::core::*;

/// CLI arguments for `tudo playbook`.
#[derive(Debug, Parser, Serialize)]
#[clap(next_help_heading = "Playbook options", about = None, long_about = None)]
pub struct PlaybookArgs {
    /// Playbook file path.
    #[serde(skip)]
    pub playbook_file: PathBuf,
    // #[clap(flatten)]
    // #[serde(flatten)]
    // pub args: CorePlaybookArgs,
}

impl Cmd for PlaybookArgs {
    type Output = ();

    /// Parse and run playbook
    fn run(self) -> eyre::Result<Self::Output> {
        println!("Playbook file path: {:#?}", self.playbook_file.green());

        let playbook = Playbook::from_file(self.playbook_file)?;

        Executor::run(&playbook)?;
        Ok(())
    }
}