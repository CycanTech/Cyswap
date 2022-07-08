use openbrush::{
    declare_storage_trait,
};
use ink_env::{DefaultEnvironment};
use ink_storage::traits::{SpreadAllocate, SpreadLayout,StorageLayout};

use primitives::Address;
use crate::traits::core::no_delegate_call::NoDelegateCall;
pub use swap_project_derive::{NoDelegateCallStorage};

/// define the struct with the data that our smart contract will be using
/// this will isolate the logic of our smart contract from its storage
#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct NoDelegateCallData {
    pub original:Address,
}

declare_storage_trait!(NoDelegateCallStorage);

impl<T:NoDelegateCallStorage<Data = NoDelegateCallData>> NoDelegateCall for T{
    fn checkNotDelegateCall(&self){
        assert!(ink_env::account_id::<DefaultEnvironment>() == self.get().original,"not use delegate!");
    }
}
