use anyhow::Result;
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

use crate::args::Args;

pub async fn run() -> Result<()> {
    init_logging();
    let args = Args::parse();

    info!(?args, "Running collectors...");

    Ok(())
}

pub fn init_logging() {
    let filter = filter::Targets::new().with_target("collectors", Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();
}
