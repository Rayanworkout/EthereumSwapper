use crate::utils::get_env_variables;

use alloy::network::EthereumWallet;
use alloy::{primitives::Address, signers::local::PrivateKeySigner};
use eyre::Result;

pub struct EthereumWalletBuilder {
    pub address: Address,
}

impl EthereumWalletBuilder {
    pub fn build(self) -> Result<EthereumWallet> {
        let pkey_env_variable = get_env_variables(vec!["WALLET_PRIVATE_KEY"]);

        let pkey = pkey_env_variable.get("WALLET_PRIVATE_KEY").unwrap();

        let signer: PrivateKeySigner = pkey.parse().expect("Could not parse private key.");

        Ok(EthereumWallet::from(signer))
    }
}
