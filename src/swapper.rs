use std::time::{Duration, SystemTime, UNIX_EPOCH};

use alloy::primitives::{utils::parse_ether, Address, Uint, U256};

use crate::{
    balances::Balances,
    contracts::{IUniswapV2Router, IUniswapV2pair, IERC20},
    provider::ProviderGenerator,
    types::{FillerProvider, HttpClient},
    utils::get_env_variables,
    wallet_builder::EthereumWalletBuilder,
};
use eyre::Result;

pub struct Swapper {
    router: Option<IUniswapV2Router::IUniswapV2RouterInstance<HttpClient, FillerProvider>>,
    usdc_contract: Option<IERC20::IERC20Instance<HttpClient, FillerProvider>>,
    reserve_in: Option<Uint<256, 4>>,
    reserve_out: Option<Uint<256, 4>>,
    my_address: Address,
    usdc_address: Address,
    weth_address: Address,
    univ2_router: Address,
    pair: Address,
    pub balances: Option<Balances>,
}

impl Swapper {
    pub async fn new() -> Result<Self> {
        let usdc_address: Address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse()?;
        let weth_address: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
        let univ2_router: Address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse()?;
        let pair: Address = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse()?;

        let address_env_variable = get_env_variables(vec!["MY_PUBLIC_ADDRESS"]);

        let my_address: Address = address_env_variable
            .get("MY_PUBLIC_ADDRESS")
            .unwrap()
            .parse()?;

        let mut swapper = Swapper {
            router: None,
            usdc_contract: None,
            reserve_in: None,
            reserve_out: None,
            my_address,
            usdc_address,
            weth_address,
            univ2_router,
            pair,
            balances: None,
        };

        // Initialize the fields that require complex setup
        swapper.prepare_swap().await?;

        Ok(swapper)
    }
    async fn prepare_swap(&mut self) -> Result<()> {
        // Instantiate a wallet in order for our provider to sign our transactions
        let wallet_builder = EthereumWalletBuilder {
            address: self.my_address,
        };
        let wallet = wallet_builder.build()?;

        let provider = ProviderGenerator { wallet }.build()?;

        let router = IUniswapV2Router::new(self.univ2_router, provider.clone());
        let pair = IUniswapV2pair::new(self.pair, provider.clone());
        let usdc_contract = IERC20::new(self.usdc_address, provider.clone());

        // Token0 must be the lexically smallest
        let token0 = if self.weth_address < self.usdc_address {
            self.weth_address
        } else {
            self.usdc_address
        };

        // Get reserves
        let (reserve_in, reserve_out) = if token0 == self.usdc_address {
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

        self.router = Some(router);
        self.usdc_contract = Some(usdc_contract);
        self.reserve_in = Some(reserve_in);
        self.reserve_out = Some(reserve_out);
        self.balances = Some(Balances::new(provider, self.my_address)?);

        Ok(())
    }

    pub async fn usdc_for_eth(
        &self,
        usdc_amount: Option<f64>,
        max_slippage: Option<f64>,
    ) -> Result<()> {
        // We multiply the amount to match the unit of USDC
        let usdc_amount = usdc_amount.unwrap_or(500.0);

        let usdc_amount_to_swap = U256::from(usdc_amount * 1_000_000.0);

        // Calculate output ETH amount
        let amount_in_with_fee = usdc_amount_to_swap * U256::from(997);
        let numerator = amount_in_with_fee * self.reserve_out.unwrap();
        let denominator = self.reserve_in.unwrap() * U256::from(1000) + amount_in_with_fee;
        let amount_out = numerator / denominator;

        // Apply slippage tolerance
        let slippage = max_slippage.unwrap_or(5.0); // default slippage to 5%
        let amount_out_min =
            amount_out * U256::from((1000.0 * (1.0 - slippage)) as u64) / U256::from(1000);

        // Path for swap: USDC -> WETH (ETH)
        let path = vec![self.usdc_address, self.weth_address];

        // Approve the Uniswap router to spend USDC
        let allowance = self
            .usdc_contract
            .as_ref()
            .unwrap()
            .allowance(self.my_address, self.univ2_router)
            .call()
            .await?;
        if allowance._0 < usdc_amount_to_swap {
            let _approve_tx = self
                .usdc_contract
                .as_ref()
                .unwrap()
                .approve(self.univ2_router, usdc_amount_to_swap)
                .send()
                .await?;
        }

        let deadline = SystemTime::now().duration_since(UNIX_EPOCH)? + Duration::from_secs(60);
        let deadline_timestamp = U256::from(deadline.as_secs());

        let tx = self
            .router
            .as_ref()
            .unwrap()
            .swapExactTokensForETH(
                usdc_amount_to_swap,
                amount_out_min,
                path,
                self.my_address,
                deadline_timestamp,
            )
            .send()
            .await?
            .watch()
            .await?;

        println!("Swapped {:.2} USDC\nHash: {}", usdc_amount, tx);

        Ok(())
    }

    pub async fn eth_for_usdc(&self, eth_amount: Option<f64>, max_slippage: Option<f64>) -> Result<()> {
        let amount_to_buy = eth_amount.unwrap_or(0.01);

        let eth_amount = parse_ether(&amount_to_buy.to_string())?;

        // Reversing reserves because the swap is the other way
        let amount_in_with_fee = eth_amount * U256::from(997);
        let numerator = amount_in_with_fee * self.reserve_in.unwrap();
        let denominator = (self.reserve_out.unwrap() * U256::from(1000)) + amount_in_with_fee;
        let amount_out = numerator / denominator;

        let slippage = max_slippage.unwrap_or(5.0) / 100.0;

        let amount_out_min =
            amount_out * U256::from(((1.0 - slippage) * 1000.0) as u64) / U256::from(1000);

        let path = vec![self.weth_address, self.usdc_address];

        let deadline = SystemTime::now().duration_since(UNIX_EPOCH)? + Duration::from_secs(60);
        let deadline_timestamp = U256::from(deadline.as_secs());

        let tx = self
            .router
            .as_ref()
            .unwrap()
            .swapExactETHForTokens(amount_out_min, path, self.my_address, deadline_timestamp)
            .value(eth_amount)
            .send()
            .await?
            .watch()
            .await?;

        println!("Swapped {:.2} ETH\nHash: {}", amount_to_buy, tx);

        Ok(())
    }
}
