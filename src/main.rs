mod balances;
mod contracts;
mod provider;
mod swapper;
mod types;
mod utils;
mod wallet_builder;

use eyre::Result;
use swapper::Swapper;

// anvil --fork-url https://eth.llamarpc.com --block-time 10

#[tokio::main]
async fn main() -> Result<()> {
    let swapper = Swapper::new().await?;

    swapper.eth_for_usdc(Some("0.05"), None).await?;

    // let balances_getter = swapper.balances.unwrap();

    Ok(())
}
