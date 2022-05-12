use brush::modifier_definition;
use ink_env::DefaultEnvironment;
use ink_storage::traits::{SpreadAllocate, SpreadLayout};
use primitives::{Address, Int24, Uint24, Uint256, Uint96, U256};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use scale::{Decode, Encode};

#[brush::wrapper]
pub type PositionManagerRef = dyn PositionManager;

#[modifier_definition]
pub fn isAuthorizedForToken<T, F, R>(instance: &mut T, body: F, tokenId: u128) -> R
where
    T: PositionManager,
    F: FnOnce(&mut T) -> R,
{
    let spender = ink_env::caller::<DefaultEnvironment>();
    assert!(
        !instance._isApprovedOrOwner(spender, tokenId),
        "Not approved"
    );
    body(instance)
}

#[derive(Default, Debug, Decode, Encode, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct MintParams {
    pub token0: Address,
    pub token1: Address,
    pub fee: Uint24,
    pub tickLower: Int24,
    pub tickUpper: Int24,
    pub amount0Desired: Uint256,
    pub amount1Desired: Uint256,
    pub amount0Min: Uint256,
    pub amount1Min: Uint256,
    pub recipient: Address,
    pub deadline: Uint256,
}
/// @title Non-fungible token for positions
/// @notice Wraps CrabSwap V3 positions in a non-fungible token interface which allows for them to be transferred
/// and authorized.
#[brush::trait_definition]
pub trait PositionManager {
    fn _isApprovedOrOwner(&self,spender:Address, tokenId:u128) -> bool;
    /// @notice Returns the position information associated with a given token ID.
    /// @dev Throws if the token ID is not valid.
    /// @param tokenId The ID of the token that represents the position
    /// @return nonce The nonce for permits
    /// @return operator The address that is approved for spending
    /// @return token0 The address of the token0 for a specific pool
    /// @return token1 The address of the token1 for a specific pool
    /// @return fee The fee associated with the pool
    /// @return tickLower The lower end of the tick range for the position
    /// @return tickUpper The higher end of the tick range for the position
    /// @return liquidity The liquidity of the position
    /// @return feeGrowthInside0LastX128 The fee growth of token0 as of the last action on the individual position
    /// @return feeGrowthInside1LastX128 The fee growth of token1 as of the last action on the individual position
    /// @return tokensOwed0 The uncollected amount of token0 owed to the position as of the last computation
    /// @return tokensOwed1 The uncollected amount of token1 owed to the position as of the last computation
    #[ink(message)]
    fn positions(&self,tokenId: u128,) -> (
        Uint96,
        Address,
        Address,
        Address,
        Uint24,
        Int24,
        Int24,
        u128,
        U256,
        U256,
        u128,
        u128,
    );

    /// @notice Creates a new position wrapped in a NFT
    /// @dev Call this when the pool does exist and is initialized. Note that if the pool is created but not initialized
    /// a method does not exist, i.e. the pool is assumed to be initialized.
    /// @param params The params necessary to mint a position, encoded as `MintParams` in calldata
    /// @return tokenId The ID of the token that represents the minted position
    /// @return liquidity The amount of liquidity for this position
    /// @return amount0 The amount of token0
    /// @return amount1 The amount of token1
    #[ink(message, payable)]
    fn mint(
        &mut self,
        token0: Address,
        token1: Address,
        fee: Uint24,
        tickLower: Int24,
        tickUpper: Int24,
        amount0Desired: U256,
        amount1Desired: U256,
        amount0Min: U256,
        amount1Min: U256,
        recipient: Address,
        deadline: U256,
    ) -> (
        u128, //tokenId
        u128, //liquidity
        U256, //amount0
        U256, //amount1
    );
}
