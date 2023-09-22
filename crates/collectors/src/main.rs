use anyhow::Result;
use collectors::cli;

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await
}
