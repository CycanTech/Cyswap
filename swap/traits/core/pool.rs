#![allow(non_snake_case)]
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};
use primitives::Int24;
use primitives::Uint8;
use primitives::{Address, Uint16, Uint160, Uint24, U160, U256};
use scale::{Decode, Encode};
use ink_prelude::vec::Vec;
//this interface is PoolActions

#[brush::wrapper]
pub type PoolActionRef = dyn PoolAction;

#[derive(
    Default, Debug, Clone, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout, SpreadAllocate,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Slot0 {
    // the current price
    pub sqrtPriceX96: Uint160,
    // the current tick
    pub tick: Int24,
    // the most-recently updated index of the observations array
    pub observationIndex: Uint16,
    // the current maximum number of observations that are being stored
    pub observationCardinality: Uint16,
    // the next maximum number of observations to store, triggered in observations.write
    pub observationCardinalityNext: Uint16,
    // the current protocol fee as a percentage of the swap fee taken on withdrawal
    // represented as an integer denominator (1/x)%
    pub feeProtocol: Uint8,
    // whether the pool is locked
    pub unlocked: bool,
}

#[brush::trait_definition]
pub trait PoolAction {
    // fn new(factory:Address,token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24) -> Self;
    /// @inheritdoc IUniswapV3PoolActions
    /// @dev not locked because it initializes unlocked
    #[ink(message, payable)]
    fn initialize(&mut self,sqrtPriceX96:U160);

    #[ink(message)]
    fn getSlot0(&self) -> Slot0;

    /// @notice Adds liquidity for the given recipient/tickLower/tickUpper position
    /// @dev The caller of this method receives a callback in the form of IUniswapV3MintCallback#uniswapV3MintCallback
    /// in which they must pay any token0 or token1 owed for the liquidity. The amount of token0/token1 due depends
    /// on tickLower, tickUpper, the amount of liquidity, and the current price.
    /// @param recipient The address for which the liquidity will be created
    /// @param tickLower The lower tick of the position in which to add liquidity
    /// @param tickUpper The upper tick of the position in which to add liquidity
    /// @param amount The amount of liquidity to mint
    /// @param data Any data that should be passed through to the callback
    /// @return amount0 The amount of token0 that was paid to mint the given amount of liquidity. Matches the value in the callback
    /// @return amount1 The amount of token1 that was paid to mint the given amount of liquidity. Matches the value in the callback
    #[ink(message)]
    fn mint(
        &mut self,
        recipient: Address,
        tickLower: Int24,
        tickUpper: Int24,
        amount: u128,
        data:Vec<u8>,
    ) -> (U256, U256);
}
