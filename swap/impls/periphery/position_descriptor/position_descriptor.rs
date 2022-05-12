use ink_storage::traits::{SpreadAllocate, SpreadLayout};
use primitives::{Address, Int24, Uint128, Uint256, Uint80, Uint96};
use ink_prelude::string::String;
use brush::{
    declare_storage_trait,
};
use crate::traits::periphery::position_descriptor::*;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
// details about the uniswap position
#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PositionDescriptor {
    
}

declare_storage_trait!(PositionDescriptorStorage, PositionDescriptor);

impl<T:PositionDescriptorStorage>  Descriptor for T{
    default fn tokenURI(&self,positionManager:Address, tokenId:u128) -> String{
        String::from("")
    }
}