use crate::contracts::IERC20;
use crate::utils::get_env_variables;
use alloy::primitives::utils::format_units;
use alloy::primitives::U256;
use alloy::providers::fillers::{
    ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};

use alloy::network::{Ethereum, EthereumWallet};
use alloy::providers::{Identity, Provider, RootProvider};
use alloy::transports::http::{Client, Http};
use alloy::{primitives::Address, signers::local::PrivateKeySigner};

use eyre::Result;

type MyProvider = FillProvider<
    JoinFill<
        JoinFill<JoinFill<JoinFill<Identity, GasFiller>, NonceFiller>, ChainIdFiller>,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

pub struct EthereumWalletBuilder {
    pub address: Address,
}

impl EthereumWalletBuilder {
    pub fn build(&self) -> Result<EthereumWallet> {
        let pkey_env_variable = get_env_variables(vec!["WALLET_PRIVATE_KEY"]);

        let pkey = pkey_env_variable.get("WALLET_PRIVATE_KEY").unwrap();

        let signer: PrivateKeySigner = pkey.parse().expect("Could not parse private key.");

        Ok(EthereumWallet::from(signer))
    }

    pub async fn get_eth_balance(&self, provider: &MyProvider) -> Result<f64> {
        let eth_balance_gwei = provider.get_balance(self.address).await?;
        let eth_balance_ether: f64 = format_units(eth_balance_gwei, "eth")?.parse()?;

        Ok(eth_balance_ether)
    }

    pub async fn get_usdc_balance(&self, provider: &MyProvider) -> Result<f64> {
        let usdc_contract_address: Address =
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse()?;

        let usdc_contract = IERC20::new(usdc_contract_address, &provider);

        let balance: U256 = usdc_contract.balanceOf(self.address).call().await?._0; // result looks like this: balanceOfReturn { _0: 10981618907 }

        let balance_in_usdc = format_units(balance, 6)?;

        let balance_as_f64: f64 = balance_in_usdc.parse()?;

        Ok(balance_as_f64)
    }
}
