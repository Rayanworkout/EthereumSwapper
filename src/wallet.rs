use crate::utils::get_env_variables;
use alloy::primitives::utils::format_units;
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
}
