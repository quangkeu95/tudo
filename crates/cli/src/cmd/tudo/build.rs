//! Build command
use clap::{Parser, ValueHint};
use owo_colors::OwoColorize;
use serde::Serialize;
use std::path::PathBuf;

use crate::cmd::utils::Cmd;
use tudo_config::Config;
use tudo_interpreter::build_workflow;

mod core;
pub use self::core::*;

/// CLI arguments for `tudo build`.
///
/// CLI arguments take the highest precedence in the Config hierarchy
#[derive(Debug, Parser, Serialize)]
#[clap(next_help_heading = "Build options", about = None, long_about = None)]
pub struct BuildArgs {
    /// Workflow file path. By default it will look for the `workflow.yaml` file at the project root
    #[clap(long = "workflow", short, value_hint = ValueHint::FilePath,
        value_name = "FILE"
    )]
    #[serde(skip)]
    pub workflow_file: Option<PathBuf>,

    #[clap(flatten)]
    #[serde(flatten)]
    pub args: CoreBuildArgs,
}

impl Cmd for BuildArgs {
    type Output = ();

    fn run(self) -> eyre::Result<Self::Output> {
        let workflow_file = Config::workflow_file(self.workflow_file);
        println!("Workflow file path: {:#?}", workflow_file.green());

        let workflow_compose = build_workflow(workflow_file)?;
        Ok(())
    }
}
