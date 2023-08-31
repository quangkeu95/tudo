use clap::Parser;
use tudo_cli::{
    cli::tudo::{Cli, Subcommands},
    cmd::utils::AsyncCmd,
    utils,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    utils::tracing_subscriber();
    utils::enable_terminal_colors();
    let cli = Cli::parse();

    match cli.subcommands {
        Subcommands::Playbook(cmd) => cmd.run().await,
    }
}
