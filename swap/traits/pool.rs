use brush::{
    traits::{
        AccountId,
    },
};
use ink_storage::traits::{SpreadLayout, PackedLayout};
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use scale::{Encode, Decode};
use primitives::Uint160;
use primitives::Int24;


#[brush::wrapper]
pub type PoolRef = dyn Pool;



#[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
struct Slot0 {
    // the current price
    sqrtPriceX96: Uint160,
    // the current tick
    tick: Int24,
    // the most-recently updated index of the observations array
    observationIndex: u16,
    // the current maximum number of observations that are being stored
    observationCardinality: u16,
    // the next maximum number of observations to store, triggered in observations.write
    observationCardinalityNext: u16,
    // the current protocol fee as a percentage of the swap fee taken on withdrawal
    // represented as an integer denominator (1/x)%
    feeProtocol: u8,
    // whether the pool is locked
    unlocked: bool,
}

#[brush::trait_definition]
pub trait Pool{
    #[ink(message, payable)]
    fn test(
        &mut self        
    ) -> u32;
}