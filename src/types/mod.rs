
pub type HttpClient = alloy::transports::http::Http<alloy::transports::http::Client>;
pub type EthereumWallet = alloy::network::EthereumWallet;
pub type Ethereum = alloy::network::Ethereum;


pub type FillerProvider = alloy::providers::fillers::FillProvider<
    alloy::providers::fillers::JoinFill<
        alloy::providers::fillers::JoinFill<
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::JoinFill<
                    alloy::providers::Identity,
                    alloy::providers::fillers::GasFiller,
                >,
                alloy::providers::fillers::NonceFiller,
            >,
            alloy::providers::fillers::ChainIdFiller,
        >,
        alloy::providers::fillers::WalletFiller<EthereumWallet>,
    >,
    alloy::providers::RootProvider<HttpClient>,
    HttpClient,
    Ethereum,
>;