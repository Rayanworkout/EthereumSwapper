# Ethereum Swapper

Some Rust 🦀 code to perform swaps on Ethereum through the Uniswap V2 router using [Alloy](https://alloy.rs/index.html).

Disclaimer /!\ This is just a simple script I wrote to deepen my understanding of Rust and the Ethereum ecosystem. It is not production-ready and should not be used as such.
Feel free to customize it to your needs.

## Installation (Linux)

_You need to have `Rust` and `Cargo` installed on your machine to run this tool. Official installation steps [here.](https://www.rust-lang.org/tools/install)_

```bash
git clone https://github.com/Rayanworkout/EthereumSwapper.git
cd EthereumSwapper
cargo build --release

sudo mv target/release/swapper /usr/local/bin

```

You can now call the binary from anywhere in your terminal.

## Usage

First, you need to fill the required env variables. A `.env.example` file is provided in the repository. Rename it to `.env` and fill the required fields.

```bash
ETHEREUM_NETWORK=sepolia
INFURA_API_KEY=API_KEY
WALLET_PRIVATE_KEY=MY_PRIVATE_KEY
MY_PUBLIC_ADDRESS=MY_PUBLIC_ADDRESS
```

Swaps are executed at the current price on UniswapV2 with a default slippage of 5%.
You can easily modify it in `src/main.rs` by changing the `SLIPPAGE` constant.

```bash
# Buy 0.05 ETH in exchange for USDC
swapper buy_eth 0.05
```

```bash
# Buy 1000 USDC in exchange for ETH
swapper buy_usdc 1000
```