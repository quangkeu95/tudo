use clap::{Parser, Subcommand};

use crate::cmd::tudo::playbook::PlaybookArgs;

#[derive(Debug, Parser)]
#[clap(name = "tudo", author = clap::crate_authors!("\n"), version = crate::utils::VERSION_MESSAGE)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcommands: Subcommands,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "Workflow execution with configuration-as-code",
    after_help = "Find more informations in the Github repository: https://github.com/quangkeu95/tudo",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    /// Playbook commands
    // #[clap(visible_aliases = [""])]
    Playbook(PlaybookArgs),
}
