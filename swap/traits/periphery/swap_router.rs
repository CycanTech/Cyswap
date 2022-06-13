#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use ink_prelude::vec::Vec;
use primitives::{Address, Uint24, U160, U256};
use scale::{Decode, Encode};

#[brush::wrapper]
pub type SwapRouterRef = dyn SwapRouter;

#[derive(Default, PartialEq, Eq, Encode, Decode)]
pub struct ExactInputSingleParams {
    pub tokenIn: Address,
    pub tokenOut: Address,
    pub fee: Uint24,
    pub recipient: Address,
    pub deadline: U256,
    pub amountIn: U256,
    pub amountOutMinimum: U256,
    pub sqrtPriceLimitX96: U160,
}

pub struct ExactInputParams {
    pub path: Vec<u8>,
    pub recipient: Address,
    pub deadline: u64,
    pub amountIn: U256,
    pub amountOutMinimum: U256,
}

#[derive(Default, PartialEq, Eq, Encode, Decode)]
pub struct ExactOutputSingleParams {
    pub tokenIn: Address,
    pub tokenOut: Address,
    pub fee: Uint24,
    pub recipient: Address,
    pub deadline: u64,
    pub amountOut: U256,
    pub amountInMaximum: U256,
    pub sqrtPriceLimitX96: U160,
}

#[derive(Default, PartialEq, Eq, Encode, Decode)]
pub struct ExactOutputParams {
    pub path: Vec<u8>,
    pub recipient: Address,
    pub deadline: u64,
    pub amountOut: U256,
    pub amountInMaximum: U256,
}

#[brush::trait_definition]
pub trait SwapRouter {
    /// @notice Swaps `amountIn` of one token for as much as possible of another token
    /// @param params The parameters necessary for the swap, encoded as `ExactInputSingleParams` in calldata
    /// @return amountOut The amount of the received token
    #[ink(message, payable)]
    fn exactInputSingle(
        &mut self,
        tokenIn: Address,
        tokenOut: Address,
        fee: Uint24,
        recipient: Address,
        deadline: U256,
        amountIn: U256,
        amountOutMinimum: U256,
        sqrtPriceLimitX96: U160,
    ) -> U256;


    /// @notice Swaps `amountIn` of one token for as much as possible of another along the specified path
    /// @param params The parameters necessary for the multi-hop swap, encoded as `ExactInputParams` in calldata
    /// @return amountOut The amount of the received token
    // function exactInput(ExactInputParams calldata params) external payable returns (uint256 amountOut);
    #[ink(message, payable)]
    fn exactInput(
        &mut self,
        path: Vec<u8>,
        recipient: Address,
        deadline: u64,
        amountIn: U256,
        amountOutMinimum: U256,
    ) -> U256;
    

    // /// @notice Swaps as little as possible of one token for `amountOut` of another token
    // /// @param params The parameters necessary for the swap, encoded as `ExactOutputSingleParams` in calldata
    // /// @return amountIn The amount of the input token
    #[ink(message, payable)]
    fn exactOutputSingle(
        &mut self,
        tokenIn: Address,
        tokenOut: Address,
        fee: Uint24,
        recipient: Address,
        deadline: u64,
        amountOut: U256,
        amountInMaximum: U256,
        sqrtPriceLimitX96: U160,
    ) -> U256;

    // /// @notice Swaps as little as possible of one token for `amountOut` of another along the specified path (reversed)
    // /// @param params The parameters necessary for the multi-hop swap, encoded as `ExactOutputParams` in calldata
    // /// @return amountIn The amount of the input token
    #[ink(message, payable)]
    fn exactOutput(
        &mut self,
        path: Vec<u8>,
        recipient: Address,
        deadline: u64,
        amountOut: U256,
        amountInMaximum: U256,
    ) -> U256;
}
