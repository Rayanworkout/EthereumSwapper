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
    let mut swapper = Swapper::new().await?;
    let balances_getter = swapper.balances.take().unwrap();

    let eth_balance_before = balances_getter.get_eth_balance().await?;
    let usdc_balance_before = balances_getter.get_usdc_balance().await?;

    println!("ETH Balance: {:.2} ETH", eth_balance_before);
    println!("USDC Balance: {:.2} $", usdc_balance_before);

    swapper.eth_for_usdc(Some(0.05), None).await?;

    let eth_balance_after = balances_getter.get_eth_balance().await?;
    let usdc_balance_after = balances_getter.get_usdc_balance().await?;

    println!("ETH Balance: {:.2} ETH", eth_balance_after);
    println!("USDC Balance: {:.2} $", usdc_balance_after);

    println!("{:.2}", eth_balance_before - eth_balance_after);

    Ok(())
}
