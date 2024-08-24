mod provider;
mod swap_et;
mod utils;
mod wallet;

use eyre::Result;
use swap_et::swap_eth_for_usdc;
use utils::get_env_variables;



// Uni V3 Router: 0xE592427A0AEce92De3Edee1F18E0157C05861564
// Uni V2 Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
// weth contract 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2
// usdc contract 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
// eth - usdc pair 0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc

// anvil --fork-url https://eth.llamarpc.com --block-time 10

#[tokio::main]
async fn main() -> Result<()> {
    let address_env_variable = get_env_variables(vec!["MY_PUBLIC_ADDRESS"]);
    let my_address = address_env_variable.get("MY_PUBLIC_ADDRESS").unwrap();

    swap_eth_for_usdc(Some("0.001"), my_address, None).await?;

    Ok(())
}
