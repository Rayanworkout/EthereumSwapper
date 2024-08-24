mod contracts;
mod provider;
mod swap_eth_for_usdc;
mod swap_usdc_for_eth;
mod utils;
mod wallet;

use eyre::Result;
use swap_usdc_for_eth::swap_usdc_for_eth;
use utils::get_env_variables;

// weth contract 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2

// anvil --fork-url https://eth.llamarpc.com --block-time 10

#[tokio::main]
async fn main() -> Result<()> {
    let address_env_variable = get_env_variables(vec!["MY_PUBLIC_ADDRESS"]);
    let my_address = address_env_variable.get("MY_PUBLIC_ADDRESS").unwrap();

    // swap_eth_for_usdc(Some("0.001"), my_address, None).await?;
    swap_usdc_for_eth(Some(1000.0), my_address, None).await?;

    Ok(())
}
