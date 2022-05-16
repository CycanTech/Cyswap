use brush::modifier_definition;
use ink_env::DefaultEnvironment;
use ink_prelude::string::String;
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
        instance._isApprovedOrOwner(spender, tokenId),
        "Not approved"
    );
    body(instance)
}

// modifier checkDeadline(uint256 deadline) {
//     require(_blockTimestamp() <= deadline, 'Transaction too old');
//     _;
// }

#[modifier_definition]
pub fn checkDeadline<T, F, R>(instance: &mut T, body: F, deadline: u64) -> R
where
    F: FnOnce(&mut T) -> R,
{
    assert!(
        ink_env::block_timestamp::<DefaultEnvironment>() <= deadline,
        "Transaction too old"
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

// #[derive(Default, Debug, Decode, Encode, SpreadAllocate, SpreadLayout)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct IncreaseLiquidityParams {
    pub tokenId: u128,
    pub amount0Desired: U256,
    pub amount1Desired: U256,
    pub amount0Min: U256,
    pub amount1Min: U256,
    pub deadline: u64,
}

// #[derive(Default, Debug, Decode, Encode, SpreadAllocate, SpreadLayout)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct DecreaseLiquidityParams {
    pub tokenId: u128,
    pub liquidity: u128,
    pub amount0Min: U256,
    pub amount1Min: U256,
    pub deadline: u64,
}

pub struct CollectParams {
    pub tokenId: u128,
    pub recipient: Address,
    pub amount0Max: u128,
    pub amount1Max: u128,
}
/// @title Non-fungible token for positions
/// @notice Wraps CrabSwap V3 positions in a non-fungible token interface which allows for them to be transferred
/// and authorized.
#[brush::trait_definition]
pub trait PositionManager {
    #[ink(message)]
    fn tokenURI(&self, tokenId: u128) -> String;

    #[ink(message)]
    fn setFactory(&mut self, factory: Address);

    /// @notice Increases the amount of liquidity in a position, with tokens paid by the `msg.sender`
    /// @param params tokenId The ID of the token for which liquidity is being increased,
    /// amount0Desired The desired amount of token0 to be spent,
    /// amount1Desired The desired amount of token1 to be spent,
    /// amount0Min The minimum amount of token0 to spend, which serves as a slippage check,
    /// amount1Min The minimum amount of token1 to spend, which serves as a slippage check,
    /// deadline The time by which the transaction must be included to effect the change
    /// @return liquidity The new liquidity amount as a result of the increase
    /// @return amount0 The amount of token0 to acheive resulting liquidity
    /// @return amount1 The amount of token1 to acheive resulting liquidity
    #[ink(message, payable)]
    fn increaseLiquidity(
        &mut self,
        tokenId: u128,
        amount0Desired: U256,
        amount1Desired: U256,
        amount0Min: U256,
        amount1Min: U256,
        deadline: u64,
    ) -> (u128, U256, U256);

    /// @notice Decreases the amount of liquidity in a position and accounts it to the position
    /// @param params tokenId The ID of the token for which liquidity is being decreased,
    /// amount The amount by which liquidity will be decreased,
    /// amount0Min The minimum amount of token0 that should be accounted for the burned liquidity,
    /// amount1Min The minimum amount of token1 that should be accounted for the burned liquidity,
    /// deadline The time by which the transaction must be included to effect the change
    /// @return amount0 The amount of token0 accounted to the position's tokens owed
    /// @return amount1 The amount of token1 accounted to the position's tokens owed
    #[ink(message, payable)]
    fn decreaseLiquidity(
        &mut self,
        tokenId: u128,
        liquidity: u128,
        amount0Min: U256,
        amount1Min: U256,
        deadline: u64,
    ) -> (U256, U256);

    fn _isApprovedOrOwner(&self, spender: Address, tokenId: u128) -> bool;
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
    fn positions(
        &self,
        tokenId: u128,
    ) -> (
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

    /// @notice Collects up to a maximum amount of fees owed to a specific position to the recipient
    /// @param params tokenId The ID of the NFT for which tokens are being collected,
    /// recipient The account that should receive the tokens,
    /// amount0Max The maximum amount of token0 to collect,
    /// amount1Max The maximum amount of token1 to collect
    /// @return amount0 The amount of fees collected in token0
    /// @return amount1 The amount of fees collected in token1
    #[ink(message, payable)]
    fn collect(
        &mut self,
        tokenId: u128,
        recipient: Address,
        amount0Max: u128,
        amount1Max: u128,
    ) -> (U256, U256);

    /// @notice Burns a token ID, which deletes it from the NFT contract. The token must have 0 liquidity and all tokens
    /// must be collected first.
    /// @param tokenId The ID of the token that is being burned
    #[ink(message, payable)]
    fn burn(&mut self,tokenId:u128);
}
