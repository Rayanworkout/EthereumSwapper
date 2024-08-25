use alloy::{network::EthereumWallet, providers::ProviderBuilder};

use eyre::Result;

use crate::types::FillerProvider;

// use crate::utils::get_env_variables;

pub struct ProviderGenerator {
    pub wallet: EthereumWallet,
}

#[allow(dead_code)]
impl ProviderGenerator {
    pub fn build(self) -> Result<FillerProvider> {
        // We check if environment variables are set
        // let vars = get_env_variables(vec!["ETHEREUM_NETWORK", "INFURA_API_KEY"]);

        // let network = vars.get("ETHEREUM_NETWORK").unwrap();

        // let infura_api_key = vars.get("INFURA_API_KEY").unwrap();

        // let rpc_url = format!("https://{network}.infura.io/v3/{infura_api_key}").parse()?;

        let rpc_url = "http://127.0.0.1:8545".parse()?;

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(self.wallet)
            .on_http(rpc_url);

        Ok(provider)
    }
}
