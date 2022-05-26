#![allow(non_snake_case)]
use brush::modifier_definition;
use ink_prelude::vec::Vec;
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};
use libs::core::Position;
use primitives::Int24;
use primitives::Int256;
use primitives::Uint8;
use primitives::{Address, Uint16, Uint160, U160, U256};
use scale::{Decode, Encode};
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
pub fn lock<T, F, R>(instance: &mut T, body: F) -> R
where
    T: PoolAction,
    F: FnOnce(&mut T) -> R,
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
    fn initialize(&mut self, sqrtPriceX96: U160);

    #[ink(message)]
    fn getSlot0(&self) -> Slot0;

    #[ink(message)]
    fn setUnLock(&mut self, unlock: bool);

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
        data: Vec<u8>,
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
    fn burn(&mut self, tickLower: Int24, tickUpper: Int24, amount: u128) -> (U256, U256);

    #[ink(message)]
    fn positions(
        &self,
        position_address: Address,
        tick_lower: Int24,
        tick_upper: Int24,
    ) -> Position::Info;

    /// @notice Collects tokens owed to a position
    /// @dev Does not recompute fees earned, which must be done either via mint or burn of any amount of liquidity.
    /// Collect must be called by the position owner. To withdraw only token0 or only token1, amount0Requested or
    /// amount1Requested may be set to zero. To withdraw all tokens owed, caller may pass any value greater than the
    /// actual tokens owed, e.g. type(uint128).max. Tokens owed may be from accumulated swap fees or burned liquidity.
    /// @param recipient The address which should receive the fees collected
    /// @param tickLower The lower tick of the position for which to collect fees
    /// @param tickUpper The upper tick of the position for which to collect fees
    /// @param amount0Requested How much token0 should be withdrawn from the fees owed
    /// @param amount1Requested How much token1 should be withdrawn from the fees owed
    /// @return amount0 The amount of fees collected in token0
    /// @return amount1 The amount of fees collected in token1
    #[ink(message)]
    fn collect(
        &mut self,
        recipient: Address,
        tickLower: Int24,
        tickUpper: Int24,
        amount0Requested: u128,
        amount1Requested: u128,
    ) -> (u128, u128);

    /// @notice Swap token0 for token1, or token1 for token0
    /// @dev The caller of this method receives a callback in the form of IUniswapV3SwapCallback#uniswapV3SwapCallback
    /// @param recipient The address to receive the output of the swap
    /// @param zeroForOne The direction of the swap, true for token0 to token1, false for token1 to token0
    /// @param amountSpecified The amount of the swap, which implicitly configures the swap as exact input (positive), or exact output (negative)
    /// @param sqrtPriceLimitX96 The Q64.96 sqrt price limit. If zero for one, the price cannot be less than this
    /// value after the swap. If one for zero, the price cannot be greater than this value after the swap
    /// @param data Any data to be passed through to the callback
    /// @return amount0 The delta of the balance of token0 of the pool, exact when negative, minimum when positive
    /// @return amount1 The delta of the balance of token1 of the pool, exact when negative, minimum when positive
    #[ink(message)]
    fn swap(
        &mut self,
        recipient:Address,
        zeroForOne:bool,
        amountSpecified:Int256,
        sqrtPriceLimitX96:U160,
        data:Vec<u8>
    ) -> (Int256,Int256);

    /// @notice Receive token0 and/or token1 and pay it back, plus a fee, in the callback
    /// @dev The caller of this method receives a callback in the form of IUniswapV3FlashCallback#uniswapV3FlashCallback
    /// @dev Can be used to donate underlying tokens pro-rata to currently in-range liquidity providers by calling
    /// with 0 amount{0,1} and sending the donation amount(s) from the callback
    /// @param recipient The address which will receive the token0 and token1 amounts
    /// @param amount0 The amount of token0 to send
    /// @param amount1 The amount of token1 to send
    /// @param data Any data to be passed through to the callback
    #[ink(message)]
    fn flash(&mut self,
         recipient:Address,
         amount0:U256,
         amount1:U256,
         data:Vec<u8>
    ) ;
}
