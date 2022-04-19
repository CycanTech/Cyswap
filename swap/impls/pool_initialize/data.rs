use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
    },
};
use ink_storage::{
    traits::{
        SpreadAllocate,
        SpreadLayout,
    },
};

#[cfg(feature = "pool_initial")]
pub use swap_project_derive::PoolInitializeStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
/// define the struct with the data that our smart contract will be using
/// this will isolate the logic of our smart contract from its storage
pub struct PoolInitializeData {
    /// @inheritdoc IPeripheryImmutableState
    pub factory:AccountId,
    /// @inheritdoc IPeripheryImmutableState
    pub WETH9:AccountId,
}

declare_storage_trait!(PoolInitializeStorage, PoolInitializeData);

