use std::time::{Duration, SystemTime, UNIX_EPOCH};

use alloy::primitives::{Address, U256};
use eyre::Result;

use crate::{
    contracts::{IUniswapV2Router, IUniswapV2pair, IERC20},
    provider::ProviderGenerator,
    wallet::EthereumWalletBuilder,
};

pub async fn swap_usdc_for_eth(
    amount: Option<f64>,
    my_address: &str,
    max_slippage: Option<f64>,
) -> Result<()> {
    let my_address: Address = my_address.parse()?;
    let usdc_address: Address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse()?;
    let weth_address: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
    let univ2_router: Address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse()?;
    let pair: Address = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse()?;

    // Instantiate a wallet for signing transactions
    let wallet_builder = EthereumWalletBuilder {
        address: my_address,
    };
    let wallet = wallet_builder.build()?;

    let provider = ProviderGenerator { wallet }.build()?;

    let router = IUniswapV2Router::new(univ2_router, &provider);
    let pair = IUniswapV2pair::new(pair, &provider);
    let usdc_contract = IERC20::new(usdc_address, &provider); // Use the extended interface

    // Token0 must be the lexically smallest
    let token0 = if weth_address < usdc_address {
        weth_address
    } else {
        usdc_address
    };

    // We multiply the amount to match the unit of USDC
    let usdc_amount = amount.unwrap_or(500.0) * 1_000_000.0;

    let usdc_amount_to_swap = U256::from(usdc_amount);

    // Get reserves
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

    // Calculate output ETH amount
    let amount_in_with_fee = usdc_amount_to_swap * U256::from(997);
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in * U256::from(1000) + amount_in_with_fee;
    let amount_out = numerator / denominator;

    // Apply slippage tolerance
    let slippage = max_slippage.unwrap_or(5.0); // default slippage to 5%
    let amount_out_min =
        amount_out * U256::from((1000.0 * (1.0 - slippage)) as u64) / U256::from(1000);

    // Path for swap: USDC -> WETH (ETH)
    let path = vec![usdc_address, weth_address];

    // Approve the Uniswap router to spend USDC
    let allowance = usdc_contract
        .allowance(my_address, univ2_router)
        .call()
        .await?;
    if allowance._0 < usdc_amount_to_swap {
        let _approve_tx = usdc_contract
            .approve(univ2_router, usdc_amount_to_swap)
            .send()
            .await?;
    }

    let deadline = SystemTime::now().duration_since(UNIX_EPOCH)? + Duration::from_secs(60);
    let deadline_timestamp = U256::from(deadline.as_secs());

    let usdc_balance_before = wallet_builder.get_usdc_balance(&provider).await?;
    println!("USDC Balance before: {} usdc", usdc_balance_before);

    let _tx = router
        .swapExactTokensForETH(
            usdc_amount_to_swap,
            amount_out_min,
            path,
            my_address,
            deadline_timestamp,
        )
        .send()
        .await?
        .watch()
        .await?;

    let eth_balance_after = wallet_builder.get_usdc_balance(&provider).await?;

    println!(
        "USDC Balance after: {} usdc\nDifference of {}",
        eth_balance_after,
        eth_balance_after - usdc_balance_before,
    );

    Ok(())
}
