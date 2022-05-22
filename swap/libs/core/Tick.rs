#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use ink_storage::{traits::{PackedLayout, SpreadLayout, StorageLayout}, Mapping};
use primitives::{Int24, Uint160, Uint24, Uint256, I56, U160, U256};
use scale::{Decode, Encode};

use crate::core::LiquidityMath;

use super::TickMath;

// info stored for each initialized individual tick
#[derive(Debug, Default, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Info {
    // the total position liquidity that references this tick
    pub liquidityGross: u128,
    // amount of net liquidity added (subtracted) when tick is crossed from left to right (right to left),
    pub liquidityNet: i128,
    // fee growth per unit of liquidity on the _other_ side of this tick (relative to the current tick)
    // only has relative meaning, not absolute — the value depends on when the tick is initialized
    pub feeGrowthOutside0X128: Uint256,
    pub feeGrowthOutside1X128: Uint256,
    // the cumulative tick value on the other side of the tick
    pub tickCumulativeOutside: I56,
    // the seconds per unit of liquidity on the _other_ side of this tick (relative to the current tick)
    // only has relative meaning, not absolute — the value depends on when the tick is initialized
    pub secondsPerLiquidityOutsideX128: Uint160,
    // the seconds spent on the other side of the tick (relative to the current tick)
    // only has relative meaning, not absolute — the value depends on when the tick is initialized
    pub secondsOutside: u64,
    // true iff the tick is initialized, i.e. the value is exactly equivalent to the expression liquidityGross != 0
    // these 8 bits are set to prevent fresh sstores when crossing newly initialized ticks
    pub initialized: bool,
}

    /// @notice Transitions to next tick as needed by price movement
    /// @param self The mapping containing all tick information for initialized ticks
    /// @param tick The destination tick of the transition
    /// @param feeGrowthGlobal0X128 The all-time global fee growth, per unit of liquidity, in token0
    /// @param feeGrowthGlobal1X128 The all-time global fee growth, per unit of liquidity, in token1
    /// @param secondsPerLiquidityCumulativeX128 The current seconds per liquidity
    /// @param tickCumulative The tick * time elapsed since the pool was first initialized
    /// @param time The current block.timestamp
    /// @return liquidityNet The amount of liquidity added (subtracted) when tick is crossed from left to right (right to left)
    // TODO test mapping value changed
    pub fn cross(
        ticks:&mut Mapping<Int24,Info>,
        tick:Int24,
         feeGrowthGlobal0X128:U256,
         feeGrowthGlobal1X128:U256,
         secondsPerLiquidityCumulativeX128:U160,
         tickCumulative:i64,
         time:u64
    ) -> i128 {
        let mut info:Info  = ticks.get(tick).expect("token not exist!");
        info.feeGrowthOutside0X128 = Uint256::new_with_u256(feeGrowthGlobal0X128 - info.feeGrowthOutside0X128.value);
        info.feeGrowthOutside1X128 = Uint256::new_with_u256(feeGrowthGlobal1X128 - info.feeGrowthOutside1X128.value);
        info.secondsPerLiquidityOutsideX128 = Uint256::new_with_u256(secondsPerLiquidityCumulativeX128 - info.secondsPerLiquidityOutsideX128.value);
        info.tickCumulativeOutside = tickCumulative - info.tickCumulativeOutside;
        info.secondsOutside = time - info.secondsOutside;
        ticks.insert(tick,&info);
        let liquidityNet = info.liquidityNet;
        liquidityNet
    }

/// @notice Derives max liquidity per tick from given tick spacing
/// @dev Executed within the pool constructor
/// @param tickSpacing The amount of required tick separation, realized in multiples of `tickSpacing`
///     e.g., a tickSpacing of 3 requires ticks to be initialized every 3rd tick i.e., ..., -6, -3, 0, 3, 6, ...
/// @return The max liquidity per tick
pub fn tick_spacing_to_max_liquidity_per_tick(tick_spacing: Int24) -> u128 {
    // int24 minTick = (TickMath.MIN_TICK / tickSpacing) * tickSpacing;
    //     int24 maxTick = (TickMath.MAX_TICK / tickSpacing) * tickSpacing;
    //     uint24 numTicks = uint24((maxTick - minTick) / tickSpacing) + 1;
    //     return type(uint128).max / numTicks;
    let min_tick: Int24 = (TickMath::MIN_TICK / tick_spacing) * tick_spacing;
    let max_tick: Int24 = (TickMath::MAX_TICK / tick_spacing) * tick_spacing;
    let num_ticks: Uint24 = (max_tick.saturating_sub(min_tick))
        .saturating_div(tick_spacing)
        .saturating_add(1)
        .try_into()
        .unwrap();
    u128::MAX.saturating_div(num_ticks.into())
}

/// @notice Derives max liquidity per tick from given tick spacing
/// @dev Executed within the pool constructor
/// @param tickSpacing The amount of required tick separation, realized in multiples of `tickSpacing`
///     e.g., a tickSpacing of 3 requires ticks to be initialized every 3rd tick i.e., ..., -6, -3, 0, 3, 6, ...
/// @return The max liquidity per tick
pub fn tickSpacingToMaxLiquidityPerTick(tickSpacing: Int24) -> u128 {
    let minTick: Int24 = (TickMath::MIN_TICK / tickSpacing) * tickSpacing;
    let maxTick: Int24 = (TickMath::MAX_TICK / tickSpacing) * tickSpacing;
    let numTicks: Uint24 = Uint24::try_from((maxTick - minTick) / tickSpacing).unwrap() + 1;
    return u128::MAX / u128::from(numTicks);
}

// move the method to pool lib
// /// @notice Retrieves fee growth data
//     /// @param self The mapping containing all tick information for initialized ticks
//     /// @param tickLower The lower tick boundary of the position
//     /// @param tickUpper The upper tick boundary of the position
//     /// @param tickCurrent The current tick
//     /// @param feeGrowthGlobal0X128 The all-time global fee growth, per unit of liquidity, in token0
//     /// @param feeGrowthGlobal1X128 The all-time global fee growth, per unit of liquidity, in token1
//     /// @return feeGrowthInside0X128 The all-time fee growth in token0, per unit of liquidity, inside the position's tick boundaries
//     /// @return feeGrowthInside1X128 The all-time fee growth in token1, per unit of liquidity, inside the position's tick boundaries
//     pub fn getFeeGrowthInside(
//         mut tickLower:&Info,
//         mut tickUpper:&Info,
//         tickCurrent:Int24,
//         feeGrowthGlobal0X128:U256,
//         feeGrowthGlobal1X128:U256
//     ) -> (U256, U256) {
//         let mut lower = tickLower;
//         let mut upper = tickUpper;

//         // calculate fee growth below
//         let feeGrowthBelow0X128:U256;
//         let feeGrowthBelow1X128:U256;
//         if (tickCurrent >= tickLower) {
//             feeGrowthBelow0X128 = lower.feeGrowthOutside0X128;
//             feeGrowthBelow1X128 = lower.feeGrowthOutside1X128;
//         } else {
//             feeGrowthBelow0X128 = feeGrowthGlobal0X128 - lower.feeGrowthOutside0X128;
//             feeGrowthBelow1X128 = feeGrowthGlobal1X128 - lower.feeGrowthOutside1X128;
//         }

//         // calculate fee growth above
//         uint256 feeGrowthAbove0X128;
//         uint256 feeGrowthAbove1X128;
//         if (tickCurrent < tickUpper) {
//             feeGrowthAbove0X128 = upper.feeGrowthOutside0X128;
//             feeGrowthAbove1X128 = upper.feeGrowthOutside1X128;
//         } else {
//             feeGrowthAbove0X128 = feeGrowthGlobal0X128 - upper.feeGrowthOutside0X128;
//             feeGrowthAbove1X128 = feeGrowthGlobal1X128 - upper.feeGrowthOutside1X128;
//         }

//         feeGrowthInside0X128 = feeGrowthGlobal0X128 - feeGrowthBelow0X128 - feeGrowthAbove0X128;
//         feeGrowthInside1X128 = feeGrowthGlobal1X128 - feeGrowthBelow1X128 - feeGrowthAbove1X128;
//     }

impl Info {
    /// @notice Updates a tick and returns true if the tick was flipped from initialized to uninitialized, or vice versa
    /// @param self The mapping containing all tick information for initialized ticks
    /// @param tick The tick that will be updated
    /// @param tickCurrent The current tick
    /// @param liquidityDelta A new amount of liquidity to be added (subtracted) when tick is crossed from left to right (right to left)
    /// @param feeGrowthGlobal0X128 The all-time global fee growth, per unit of liquidity, in token0
    /// @param feeGrowthGlobal1X128 The all-time global fee growth, per unit of liquidity, in token1
    /// @param secondsPerLiquidityCumulativeX128 The all-time seconds per max(1, liquidity) of the pool
    /// @param tickCumulative The tick * time elapsed since the pool was first initialized
    /// @param time The current block timestamp cast to a uint32
    /// @param upper true for updating a position's upper tick, or false for updating a position's lower tick
    /// @param maxLiquidity The maximum liquidity allocation for a single tick
    /// @return flipped Whether the tick was flipped from initialized to uninitialized, or vice versa
    pub fn update(
        &mut self,
        tick: Int24,
        tickCurrent: Int24,
        liquidityDelta: i128,
        feeGrowthGlobal0X128: U256,
        feeGrowthGlobal1X128: U256,
        secondsPerLiquidityCumulativeX128: U160,
        tickCumulative: i64,
        time: u64,
        upper: bool,
        maxLiquidity: u128,
    ) -> bool {
        // Tick.Info storage info = self[tick];
        let liquidityGrossBefore: u128 = self.liquidityGross;
        let liquidityGrossAfter: u128 =
        LiquidityMath::addDelta(liquidityGrossBefore, liquidityDelta);
        assert!(liquidityGrossAfter <= maxLiquidity, "LO");

        let flipped = (liquidityGrossAfter == 0) != (liquidityGrossBefore == 0);
        if liquidityGrossBefore == 0 {
            // by convention, we assume that all growth before a tick was initialized happened _below_ the tick
            if tick <= tickCurrent {
                self.feeGrowthOutside0X128 = Uint256::new_with_u256(feeGrowthGlobal0X128);
                self.feeGrowthOutside1X128 = Uint256::new_with_u256(feeGrowthGlobal1X128);
                self.secondsPerLiquidityOutsideX128 =
                    Uint256::new_with_u256(secondsPerLiquidityCumulativeX128);
                self.tickCumulativeOutside = tickCumulative;
                self.secondsOutside = time;
            }
            self.initialized = true;
        }
        self.liquidityGross = liquidityGrossAfter;

        // when the lower (upper) tick is crossed left to right (right to left), liquidity must be added (removed)
        // info.liquidityNet = upper
        //     ? int256(info.liquidityNet).sub(liquidityDelta).toInt128()
        //     : int256(info.liquidityNet).add(liquidityDelta).toInt128();
        if upper {
            self.liquidityNet = self.liquidityNet.checked_sub(liquidityDelta).unwrap();
        } else {
            self.liquidityNet = self.liquidityNet.checked_add(liquidityDelta).unwrap();
        }
        flipped
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use ink_lang as ink;

    #[ink::test]
    fn it_works() {
        let mut info = Info::default();
        info.liquidityNet = 1000;
        println!("info is:{:?}", info);
        //     &mut self,
        // tick:Int24,
        // tickCurrent:Int24,
        // liquidityDelta:i128,
        // feeGrowthGlobal0X128:U256,
        // feeGrowthGlobal1X128:U256,
        // secondsPerLiquidityCumulativeX128:U160,
        // tickCumulative:i64,
        // time:u32,
        // upper:bool,
        // maxLiquidity:u128
        
        info.update(
            10,
            100,
            10,
            U256::from(100),
            U256::from(200),
            U256::from(300),
            100,
            10,
            true,
            1000000,
        );
        println!("info is:{:?}", info);
    }

    #[ink::test]
    fn it_i_to_string() {
        let i = 5i128;
        let mut j = i.to_string();
        j.push_str("abc");
        println!("j is:{}",j);
    }
}
