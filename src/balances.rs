use alloy::{
    primitives::{utils::format_units, Address, U256},
    providers::Provider,
};

use crate::{contracts::IERC20, types::FillerProvider};

use eyre::Result;

pub struct Balances {
    provider: FillerProvider,
    address: Address,
}

impl Balances {
    pub fn new(provider: FillerProvider, address: Address) -> Result<Self> {
        Ok(Balances { provider, address })
    }

    pub async fn get_eth_balance(&self) -> Result<f64> {
        let eth_balance_gwei = self.provider.get_balance(self.address).await?;
        let eth_balance_ether: f64 = format_units(eth_balance_gwei, "eth")?.parse()?;

        Ok(eth_balance_ether)
    }

    pub async fn get_usdc_balance(&self) -> Result<f64> {
        let usdc_contract_address: Address =
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse()?;

        let usdc_contract = IERC20::new(usdc_contract_address, &self.provider);

        let balance: U256 = usdc_contract.balanceOf(self.address).call().await?._0; // result looks like this: balanceOfReturn { _0: 10981618907 }

        let balance_in_usdc = format_units(balance, 6)?;

        let balance_as_f64: f64 = balance_in_usdc.parse()?;

        Ok(balance_as_f64)
    }
}
