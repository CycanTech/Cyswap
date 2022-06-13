use primitives::Int256;
use ink_prelude::vec::Vec;

#[brush::wrapper]
pub type SwapCallbackRef = dyn SwapCallback;

// @title Callback for IUniswapV3PoolActions#swap
// @notice Any contract that calls IUniswapV3PoolActions#swap must implement this interface
// rename UniswapV3SwapCallback to SwapCallback
#[brush::trait_definition]
pub trait SwapCallback {
    // @notice Called to `msg.sender` after executing a swap via IUniswapV3Pool#swap.
    // @dev In the implementation you must pay the pool tokens owed for the swap.
    // The caller of this method must be checked to be a UniswapV3Pool deployed by the canonical UniswapV3Factory.
    // amount0Delta and amount1Delta can both be 0 if no tokens were swapped.
    // @param amount0Delta The amount of token0 that was sent (negative) or must be received (positive) by the pool by
    // the end of the swap. If positive, the callback must send that amount of token0 to the pool.
    // @param amount1Delta The amount of token1 that was sent (negative) or must be received (positive) by the pool by
    // the end of the swap. If positive, the callback must send that amount of token1 to the pool.
    // @param data Any data passed through by the caller via the IUniswapV3PoolActions#swap call
    #[ink(message)]
    fn swapCallback(&mut self, amount0Delta: Int256, amount1Delta: Int256, data: Vec<u8>);
}
