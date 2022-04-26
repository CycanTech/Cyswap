use ink_storage::traits::{SpreadAllocate, SpreadLayout};
use primitives::{Address, Int24, Uint160, Uint24, Uint256, U256};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use scale::{Decode, Encode};

#[brush::wrapper]
pub type PositionManagerRef = dyn PositionManager;

#[derive(Default, Debug,Decode,Encode, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct MintParams {
    token0: Address,
    token1: Address,
    fee: Uint24,
    tickLower: Int24,
    tickUpper: Int24,
    amount0Desired: Uint256,
    amount1Desired: Uint256,
    amount0Min: Uint256,
    amount1Min: Uint256,
    recipient: Address,
    deadline: Uint256,
}
/// @title Non-fungible token for positions
/// @notice Wraps CrabSwap V3 positions in a non-fungible token interface which allows for them to be transferred
/// and authorized.
#[brush::trait_definition]
pub trait PositionManager {
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
        params: MintParams,
    ) -> (
        Uint256, //tokenId
        u128,    //liquidity
        Uint256, //amount0
        Uint256, //amount1
    );

}
