use primitives::{Address, Uint256};
use ink_prelude::string::String;

#[brush::wrapper]
pub type DescriptorRef = dyn Descriptor;


/// @title Creates and initializes V3 Pools
/// @notice Provides a method for creating and initializing a pool, if necessary, for bundling with other methods that
/// require the pool to exist.
#[brush::trait_definition]
pub trait Descriptor{
    /// @notice Produces the URI describing a particular token ID for a position manager
    /// @dev Note this URI may be a data: URI with the JSON contents directly inlined
    /// @param positionManager The position manager for which to describe the token
    /// @param tokenId The ID of the token for which to produce a description, which may not be valid
    /// @return The URI of the ERC721-compliant metadata
    #[ink(message)]
    // fn tokenURI(&self, tokenId:Uint256) -> String;
    fn tokenURI(&self,positionManager:Address, tokenId:u128) -> String;
}