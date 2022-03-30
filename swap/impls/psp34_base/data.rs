use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Hash,
        ZERO_ADDRESS,
    },
};
use ink_storage::{
    traits::{
        SpreadAllocate,
        SpreadLayout,
    },
    Mapping,
};
use ink_prelude::string::String;

#[cfg(feature = "psp34_base")]
pub use swap_project_derive::PSP34BaseStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
/// define the struct with the data that our smart contract will be using
/// this will isolate the logic of our smart contract from its storage
pub struct PSP34BaseData {
    /// @dev The hash of the name used in the permit signature verification
    pub name:String,

    /// @dev The hash of the version string used in the permit signature verification
    pub symbol:String,
    
}

declare_storage_trait!(PSP34BaseStorage, PSP34BaseData);

