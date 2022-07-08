use openbrush::{
    declare_storage_trait,
};
use ink_storage::traits::{
    SpreadAllocate,
    SpreadLayout,
};

#[cfg(feature = "erc721_permit")]
pub use swap_project_derive::ERC721PermitStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
/// define the struct with the data that our smart contract will be using
/// this will isolate the logic of our smart contract from its storage
pub struct ERC721PermitData {
    /// @dev The hash of the name used in the permit signature verification
    pub nameHash:[u8;32],

    /// @dev The hash of the version string used in the permit signature verification
    pub versionHash:[u8;32],
    
}

declare_storage_trait!(ERC721PermitStorage);

