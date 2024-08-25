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

    // Buying USDC

    let eth_balance_before = balances_getter.get_eth_balance().await?;
    let usdc_balance_before = balances_getter.get_usdc_balance().await?;

    println!("ETH Balance: {:.2} ETH", eth_balance_before);
    println!("USDC Balance: {:.2} $", usdc_balance_before);

    match swapper.eth_for_usdc(Some(0.05), None).await {
        Ok(hash) => println!("First oook !\n{}", hash),
        Err(err) => {
            println!("Error first !\n{}", err);
            panic!("Wow cannot proceed !")
        }
    }

    let eth_balance_after = balances_getter.get_eth_balance().await?;
    let usdc_balance_after = balances_getter.get_usdc_balance().await?;

    println!("ETH Balance: {:.2} ETH", eth_balance_after);
    println!("USDC Balance: {:.2} $", usdc_balance_after);

    println!(
        "Difference of {:.2} ETH",
        eth_balance_before - eth_balance_after
    );

    Ok(())
}
