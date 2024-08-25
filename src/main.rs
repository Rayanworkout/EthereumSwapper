mod app;
mod providers;
mod types;
mod utils;

use eyre::Result;

// anvil --fork-url https://eth.llamarpc.com --block-time 10

#[tokio::main]
async fn main() -> Result<()> {
    const MAX_SLIPPAGE: f64 = 5.0; // %

    app::run(MAX_SLIPPAGE).await?;

    Ok(())
}
