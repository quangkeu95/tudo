use amms::discovery::factory::{discover_factories, DiscoverableFactory};
use anyhow::Result;
use clap::Parser;
use ethers::providers::{Http, Provider};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

use crate::args::Args;

pub async fn run() -> Result<()> {
    init_logging();
    let args = Args::parse();

    info!(?args, "Running collectors...");

    let provider = Arc::new(Provider::<Http>::try_from(args.rpc_url)?);

    let number_of_amms_threshold = 1000;
    let factories = discover_factories(
        vec![
            DiscoverableFactory::UniswapV2Factory,
            DiscoverableFactory::UniswapV3Factory,
        ],
        number_of_amms_threshold,
        provider,
        100000,
    )
    .await?;

    info!(?factories, "Factories");

    Ok(())
}

pub fn init_logging() {
    let filter = filter::Targets::new().with_target("collectors", Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();
}
