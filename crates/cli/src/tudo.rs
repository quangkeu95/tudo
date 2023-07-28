use clap::Parser;
use tudo_cli::{
    cmd::utils::Cmd,
    opts::tudo::{Opts, Subcommands},
    utils,
};

fn main() -> eyre::Result<()> {
    utils::tracing_subscriber();
    utils::enable_terminal_colors();
    let opts = Opts::parse();

    match opts.subcommands {
        Subcommands::Build(cmd) => cmd.run(),
    }
}
