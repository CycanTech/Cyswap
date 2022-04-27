#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use ink_storage::traits::{SpreadLayout, PackedLayout, SpreadAllocate};
use primitives::{Address, Uint24, Uint256, Int24, U256};
use scale::{Encode, Decode};
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[brush::wrapper]
pub type LiquidityManagementTraitRef = dyn LiquidityManagementTrait;


#[derive(Default, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout,SpreadAllocate)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct AddLiquidityParams {
    pub token0: Address,
    pub token1: Address,
    pub fee: Uint24,
    pub recipient: Address,
    pub tickLower: Int24,
    pub tickUpper: Int24,
    pub amount0Desired: Uint256,
    pub amount1Desired: Uint256,
    pub amount0Min: Uint256,
    pub amount1Min: Uint256,
}
/// @title Creates and initializes V3 Pools
/// @notice Provides a method for creating and initializing a pool, if necessary, for bundling with other methods that
/// require the pool to exist.
#[brush::trait_definition]
pub trait LiquidityManagementTrait{
    /// @notice Creates a new pool if it does not exist, then initializes if not initialized
    /// @dev This method can be bundled with others via IMulticall for the first action (e.g. mint) performed against a pool
    /// @param token0 The contract address of token0 of the pool
    /// @param token1 The contract address of token1 of the pool
    /// @param fee The fee amount of the v3 pool for the specified token pair
    /// @param sqrtPriceX96 The initial square root price of the pool as a Q64.96 value
    /// @return pool Returns the pool address based on the pair of tokens and fee, will return the newly created pool address if necessary
    #[ink(message)]
    fn addLiquidity(&mut self,params:AddLiquidityParams)->(u128,U256, U256,Address);

    /// @notice Called to `msg.sender` after minting liquidity to a position from IUniswapV3Pool#mint.
    /// @dev In the implementation you must pay the pool tokens owed for the minted liquidity.
    /// The caller of this method must be checked to be a UniswapV3Pool deployed by the canonical UniswapV3Factory.
    /// @param amount0Owed The amount of token0 due to the pool for the minted liquidity
    /// @param amount1Owed The amount of token1 due to the pool for the minted liquidity
    /// @param data Any data passed through by the caller via the IUniswapV3PoolActions#mint call
    #[ink(message)]
    fn uniswapV3MintCallback(&mut self,
        amount0Owed:U256,
        amount1Owed:U256,
        data:Vec<u8>
    );
}