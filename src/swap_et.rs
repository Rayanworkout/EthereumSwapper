use crate::{provider::ProviderGenerator, wallet::EthereumWalletBuilder};

use alloy::{
    primitives::{utils::parse_ether, Address, U256},
    sol,
};
use eyre::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    contract IUniswapV2Router {
        function swapExactETHForTokens(uint amountOutMin, address[] calldata path, address to, uint deadline) external payable returns (uint[] memory amounts);
        function swapExactTokensForETH(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts);
    }
}

sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    contract IUniswapV2pair {
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
        function token0() external view returns (address);
    }
}

pub async fn swap_eth_for_usdc(
    amount: Option<&str>,
    my_address: &str,
    max_slippage: Option<f64>,
) -> Result<()> {
    let my_address: Address = my_address.parse()?;
    let usdc_address: Address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse()?;
    let weth_address: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
    let univ2_router: Address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse()?;
    let pair: Address = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse()?;

    // We instanciate a wallet in order for our provider to sign our transactions
    let wallet_builder = EthereumWalletBuilder {
        address: my_address,
    };
    let wallet = wallet_builder.build()?;

    let provider = ProviderGenerator { wallet }.build()?;

    let router = IUniswapV2Router::new(univ2_router, &provider);
    let pair = IUniswapV2pair::new(pair, &provider);

    // Token0 must be the lexically smallest
    let token0 = if weth_address < usdc_address {
        weth_address
    } else {
        usdc_address
    };

    let amount_to_buy = match amount {
        Some(value) => value,
        None => "0.01",
    };

    let eth_amount = parse_ether(amount_to_buy)?;

    let (reserve_in, reserve_out) = if token0 == usdc_address {
        let IUniswapV2pair::getReservesReturn {
            reserve0, reserve1, ..
        } = pair.getReserves().call().await?;
        (U256::from(reserve0), U256::from(reserve1))
    } else {
        let IUniswapV2pair::getReservesReturn {
            reserve0, reserve1, ..
        } = pair.getReserves().call().await?;
        (U256::from(reserve1), U256::from(reserve0))
    };

    let amount_in_with_fee = eth_amount * U256::from(997);
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in * U256::from(1000) + amount_in_with_fee;
    let amount_out = numerator / denominator;

    let slippage = match max_slippage {
        Some(value) => value,
        None => 0.5, // default slippage to 0.5%
    };

    let amount_out_min =
        amount_out * U256::from((1000.0 * (1.0 - slippage)) as u64) / U256::from(1000);

    let path = vec![weth_address, usdc_address];

    let deadline = SystemTime::now().duration_since(UNIX_EPOCH)? + Duration::from_secs(60);
    let deadline_timestamp = U256::from(deadline.as_secs());

    let eth_balance_before = wallet_builder.get_eth_balance(&provider).await?;
    println!("Balance before: {} eth", eth_balance_before);

    let _tx = router
        .swapExactETHForTokens(amount_out_min, path, my_address, deadline_timestamp)
        .value(amount_in_with_fee)
        .send()
        .await?
        .watch()
        .await?;

    let eth_balance_after = wallet_builder.get_eth_balance(&provider).await?;

    println!(
        "Balance after: {} eth\nDifference of {}",
        eth_balance_after,
        eth_balance_before - eth_balance_after,
    );

    Ok(())
}
