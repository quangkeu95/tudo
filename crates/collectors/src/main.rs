use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    collectors::run().await
}
