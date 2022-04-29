#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]
use ink_storage::{traits::{SpreadLayout, PackedLayout,StorageLayout}, Mapping};
use primitives::{Uint256, U256};
use scale::{Encode, Decode};

use crate::swap::FullMath;

use super::{LiquidityMath, FixedPoint128};


/// @title Position
/// @notice Positions represent an owner address' liquidity between a lower and upper tick boundary
/// @dev Positions store additional state for tracking fees owed to the position

// info stored for each user's position
#[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Info {
    // the amount of liquidity owned by this position
    pub liquidity:u128,
    // fee growth per unit of liquidity as of the last update to liquidity or fees owed
    pub feeGrowthInside0LastX128:Uint256,
    pub feeGrowthInside1LastX128:Uint256,
    // the fees owed to the position owner in token0/token1
    pub tokensOwed0:u128,
    pub tokensOwed1:u128,
}

// trait MapOperation{
//     fn update(&self,key:u128,liquid:u128);
// }

// impl MapOperation for Mapping<u128,Info>{
//     fn update(&self,key:u128,liquid:u128){
//         let mut info:Info = self.get(key).unwrap();
//         info.liquidity = liquid;
//     }
// }

impl Info {
    /// @notice Credits accumulated fees to a user's position
    /// @param self The individual position to update
    /// @param liquidityDelta The change in pool liquidity as a result of the position update
    /// @param feeGrowthInside0X128 The all-time fee growth in token0, per unit of liquidity, inside the position's tick boundaries
    /// @param feeGrowthInside1X128 The all-time fee growth in token1, per unit of liquidity, inside the position's tick boundaries
    pub fn update(
        &mut self,
        liquidityDelta:i128,
        feeGrowthInside0X128:U256,
        feeGrowthInside1X128:U256
    )  {

        let liquidityNext:u128;
        if liquidityDelta == 0 {
            assert!(self.liquidity > 0, "NP"); // disallow pokes for 0 liquidity positions
            liquidityNext = self.liquidity;
        } else {
            liquidityNext = LiquidityMath::addDelta(self.liquidity, liquidityDelta);
        }

        // calculate accumulated fees
        let tokensOwed0:u128 =
                FullMath::mulDiv(
                    feeGrowthInside0X128 - self.feeGrowthInside0LastX128.value,
                    U256::from(self.liquidity),
                    U256::from(FixedPoint128::Q128)
                ).as_u128()
            ;
        let tokensOwed1:u128 =
                FullMath::mulDiv(
                    feeGrowthInside1X128 - self.feeGrowthInside1LastX128.value,
                    U256::from(self.liquidity),
                    U256::from(FixedPoint128::Q128)
                ).as_u128();

        // update the position
        if liquidityDelta != 0 {
            self.liquidity = liquidityNext;
        }
        self.feeGrowthInside0LastX128 = Uint256::new_with_u256(feeGrowthInside0X128);
        self.feeGrowthInside1LastX128 = Uint256::new_with_u256(feeGrowthInside1X128);
        if tokensOwed0 > 0 || tokensOwed1 > 0 {
            // overflow is acceptable, have to withdraw before you hit type(uint128).max fees
            self.tokensOwed0 += tokensOwed0;
            self.tokensOwed1 += tokensOwed1;
        }
    }
}
