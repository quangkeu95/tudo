use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

pub async fn run() -> Result<()> {
    let filter = filter::Targets::new().with_target("collectors", Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    info!("Running collectors...");

    Ok(())
}
