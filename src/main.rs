mod balances;
mod contracts;
mod provider;
mod swapper;
mod types;
mod utils;
mod wallet_builder;

use eyre::Result;
use std::env;
use swapper::Swapper;
use utils::confirm_swap;

// anvil --fork-url https://eth.llamarpc.com --block-time 10

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // $ swapper buy_eth <amount>

    if args.len() < 3 {
        println!("Please provide the command to run and the amount to buy.");
        return Ok(());
    }

    let command = &args[1].to_lowercase();

    let str_amount = args[2].parse();

    let amount: f64 = match str_amount {
        Ok(amount) => amount,
        Err(_) => {
            println!("Invalid amount. Please provide a valid number.");
            return Ok(());
        }
    };

    match command.as_str() {
        "buy_eth" => {
            confirm_swap(command, amount)?;

            let mut swapper = Swapper::new().await?;

            match swapper.eth_for_usdc(Some(amount), None).await {
                Ok(hash) => {
                    println!("Successfully swapped {} ETH.\nHash: {}", amount, hash);
                }
                Err(err) => {
                    panic!("{}", err);
                }
            };
        }
        "buy_usdc" => {
            confirm_swap(command, amount)?;

            let mut swapper = Swapper::new().await?;

            match swapper.usdc_for_eth(Some(amount), None).await {
                Ok(hash) => {
                    println!("Successfully swapped {} USDC.\nHash: {}", amount, hash);
                }
                Err(err) => {
                    panic!("{}", err);
                }
            };
        }
        _ => {
            println!(
                "Invalid command. Please use 'buy_eth' or 'buy_usdc' along with the amount to buy."
            );
            return Ok(());
        }
    }

    Ok(())
}
