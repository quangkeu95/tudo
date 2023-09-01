use clap::Parser;
use tudo_cli::{
    cli::tudo::{Cli, Subcommands},
    cmd::utils::AsyncCmd,
    utils,
};
use tudo_config::logging::{info, init_tracing_subscriber};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    init_tracing_subscriber();
    info!("Tracing initialized");
    utils::enable_terminal_colors();
    let cli = Cli::parse();

    match cli.subcommands {
        Subcommands::Playbook(cmd) => cmd.run().await,
    }
}
