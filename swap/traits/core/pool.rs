#![allow(non_snake_case)]
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};
use libs::core::Position;
use primitives::Int24;
use primitives::Uint8;
use primitives::{Address, Uint16, Uint160, U160, U256};
use scale::{Decode, Encode};
use ink_prelude::vec::Vec;
use brush::modifier_definition;
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

    /// @dev Mutually exclusive reentrancy protection into the pool to/from a method. This method also prevents entrance
    /// to a function before the pool is initialized. The reentrancy guard is required throughout the contract because
    /// we use balance checks to determine the payment status of interactions such as mint, swap and flash.
    // modifier lock() {
    //     require(slot0.unlocked, 'LOK');
    //     slot0.unlocked = false;
    //     _;
    //     slot0.unlocked = true;
    // }
    #[modifier_definition]
    pub fn lock<T, F, R>(instance: &mut T, body: F, ) -> R
    where
        T: PoolAction,
        F: FnOnce(&mut T)->R,
    {
        let unlocked = instance.getSlot0().unlocked;
        assert!(unlocked, "LOK");
        instance.setUnLock(false);
        let result = body(instance);
        instance.setUnLock(true);
        result

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

    #[ink(message)]
    fn setUnLock(&mut self,unlock:bool);

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

    /// @notice Burn liquidity from the sender and account tokens owed for the liquidity to the position
    /// @dev Can be used to trigger a recalculation of fees owed to a position by calling with an amount of 0
    /// @dev Fees must be collected separately via a call to #collect
    /// @param tickLower The lower tick of the position for which to burn liquidity
    /// @param tickUpper The upper tick of the position for which to burn liquidity
    /// @param amount How much liquidity to burn
    /// @return amount0 The amount of token0 sent to the recipient
    /// @return amount1 The amount of token1 sent to the recipient
    #[ink(message)]
    fn burn(&mut self,
        tickLower:Int24,
        tickUpper:Int24,
        amount:u128,
    ) -> (U256,U256);

    #[ink(message)]
    fn positions(&self,position_address:Address,tick_lower:Int24,tick_upper:Int24) -> Position::Info;
}
