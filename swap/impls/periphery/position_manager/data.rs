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

use primitives::{Address, Int24, Uint128, Uint256, Uint80, Uint96};

#[cfg(feature = "position_manager")]
pub use swap_project_derive::PositionStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PositionData {
    // the nonce for permits
    nonce:Uint96,
    // the address that is approved for spending this token
    operator:Address,
    // the ID of the pool with which this token is connected
    poolId:Uint80,
    // the tick range of the position
    tickLower:Int24,
    tickUpper:Int24,
    // the liquidity of the position
    liquidity:Uint128,
    // the fee growth of the aggregate position as of the last action on the individual position
    feeGrowthInside0LastX128:Uint256,
    feeGrowthInside1LastX128:Uint256,
    // how many uncollected tokens are owed to the position, as of the last computation
    tokensOwed0:Uint128,
    tokensOwed1:Uint128,
}

declare_storage_trait!(PositionStorage, PositionData);

