use alloy::sol;


// Uniswap V2 Router
// Uni V3 Router: 0xE592427A0AEce92De3Edee1F18E0157C05861564
// Uni V2 Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    contract IUniswapV2Router {
        function swapExactETHForTokens(uint amountOutMin, address[] calldata path, address to, uint deadline) external payable returns (uint[] memory amounts);
        function swapExactTokensForETH(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts);
    }
}

// ETH / USDC pair contract
// ETH - USDC pair 0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc
sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    contract IUniswapV2pair {
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
        function token0() external view returns (address);
    }
}

// ERC-20 contract
// USDC contract 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    contract IERC20 {
        function balanceOf(address owner) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
    }
}