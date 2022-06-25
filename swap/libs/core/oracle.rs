#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use ink_storage::traits::StorageLayout;
use ink_storage::{
    traits::{PackedLayout, SpreadAllocate, SpreadLayout},
    Mapping,
};
use primitives::{Int24, Uint160,  U160, U256,  I56};
use scale::{Decode, Encode};
use ink_prelude::vec::Vec;
use ink_prelude::string::ToString;

/// @title Oracle
/// @notice Provides price and liquidity data useful for a wide variety of system designs
/// @dev Instances of stored oracle data, "observations", are collected in the oracle array
/// Every pool is initialized with an oracle array length of 1. Anyone can pay the SSTOREs to increase the
/// maximum length of the oracle array. New slots will be added when the array is fully populated.
/// Observations are overwritten when the full length of the oracle array is populated.
/// The most recent observation is available, independent of the length of the oracle array, by passing 0 to observe()
#[derive(
    Default, Debug, Decode, Encode, Copy, Clone, SpreadAllocate, SpreadLayout, PackedLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Observation {
    // the block timestamp of the observation
    pub blockTimestamp: u64,
    // the tick accumulator, i.e. tick * time elapsed since the pool was first initialized
    pub tickCumulative: i64,
    // the seconds per liquidity, i.e. seconds elapsed / max(1, liquidity) since the pool was first initialized
    pub secondsPerLiquidityCumulativeX128: Uint160,
    // whether or not the observation is initialized
    pub initialized: bool,
}

#[derive(Debug, Default,SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo,StorageLayout))]
pub struct Observations {
    pub obs: Mapping<u16, Observation>,
    // pub obs: [Observation;65535],
}

// #[cfg(feature = "std")]
// impl StorageLayout for Observations {
//     fn layout(key_ptr: &mut ink_primitives::KeyPtr) -> Layout {
//         Layout::Struct(StructLayout::new([
//             FieldLayout::new("len", <Mapping<usize,Observation> as StorageLayout>::layout(key_ptr)),
//             FieldLayout::new(
//                 "elems",
//                 <Mapping<usize,Observation> as StorageLayout>::layout(key_ptr),
//             ),
//         ]))
//     }
// }

impl Observations {
    pub fn new() -> Self {
        Observations {
            obs: Default::default(),
        }
    }
    /// @notice Initialize the oracle array by writing the first slot. Called once for the lifecycle of the observations array
    /// @param self The stored oracle array
    /// @param time The time of the oracle initialization, via block.timestamp truncated to uint32
    /// @return cardinality The number of populated elements in the oracle array
    /// @return cardinalityNext The new length of the oracle array, independent of population
    pub fn initialize(
        &mut self,
        time: u64,
    ) -> (
        u16, //cardinality:
        u16, //cardinalityNext
    ) {
        // self[0] = Observation({
        //     blockTimestamp: time,
        //     tickCumulative: 0,
        //     secondsPerLiquidityCumulativeX128: 0,
        //     initialized: true
        // });
        // return (1, 1);
        self.obs.insert(
            0,
            &Observation {
                blockTimestamp: time,
                tickCumulative: 0,
                secondsPerLiquidityCumulativeX128: Uint160::new(),
                initialized: true,
            },
        );
        return (1, 1);
    }

    /// @dev Reverts if an observation at or before the desired observation timestamp does not exist.
    /// 0 may be passed as `secondsAgo' to return the current cumulative values.
    /// If called with a timestamp falling between two observations, returns the counterfactual accumulator values
    /// at exactly the timestamp between the two observations.
    /// @param self The stored oracle array
    /// @param time The current block timestamp
    /// @param secondsAgo The amount of time to look back, in seconds, at which point to return an observation
    /// @param tick The current tick
    /// @param index The index of the observation that was most recently written to the observations array
    /// @param liquidity The current in-range pool liquidity
    /// @param cardinality The number of populated elements in the oracle array
    /// @return tickCumulative The tick * time elapsed since the pool was first initialized, as of `secondsAgo`
    /// @return secondsPerLiquidityCumulativeX128 The time elapsed / max(1, liquidity) since the pool was first initialized, as of `secondsAgo`
    pub fn observeSingle(
        &self,
        time: u64,
        secondsAgo: u64,
        tick: Int24,
        index: u16,
        liquidity: u128,
        cardinality: u16,
    ) -> (i64, U160) {
        // if (secondsAgo == 0) {
        if secondsAgo == 0 {
            ink_env::debug_println!("secondsAgo is----------------:{:?}",secondsAgo);
            //     Observation memory last = self[index];
            let mut last: Observation = self.obs.get(index).expect("error!");
            //     if (last.blockTimestamp != time) last = transform(last, time, tick, liquidity);
            if last.blockTimestamp != time {
                last = transform(&last, time, tick, liquidity);
            }
            //     return (last.tickCumulative, last.secondsPerLiquidityCumulativeX128);
            // }
            return (
                last.tickCumulative,
                last.secondsPerLiquidityCumulativeX128.value,
            );
        }

        ink_env::debug_println!("time is----------------:{:?},secondsAgo is-------------:{:?}",time,secondsAgo);
        // uint32 target = time - secondsAgo;
        let target: u64 = time - secondsAgo;
        // (Observation memory beforeOrAt, Observation memory atOrAfter) =
        //     getSurroundingObservations(, time, target, tick, index, liquidity, cardinality);
        let (beforeOrAt, atOrAfter) =
            self.getSurroundingObservations(time, target, tick, index, liquidity, cardinality);

        // if (target == beforeOrAt.blockTimestamp) {
        //     // we're at the left boundary
        //     return (beforeOrAt.tickCumulative, beforeOrAt.secondsPerLiquidityCumulativeX128);
        // } else if (target == atOrAfter.blockTimestamp) {
        //     // we're at the right boundary
        //     return (atOrAfter.tickCumulative, atOrAfter.secondsPerLiquidityCumulativeX128);
        // } else {
        //     // we're in the middle
        //     uint32 observationTimeDelta = atOrAfter.blockTimestamp - beforeOrAt.blockTimestamp;
        //     uint32 targetDelta = target - beforeOrAt.blockTimestamp;
        //     return (
        //         beforeOrAt.tickCumulative +
        //             ((atOrAfter.tickCumulative - beforeOrAt.tickCumulative) / observationTimeDelta) *
        //             targetDelta,
        //         beforeOrAt.secondsPerLiquidityCumulativeX128 +
        //             uint160(
        //                 (uint256(
        //                     atOrAfter.secondsPerLiquidityCumulativeX128 - beforeOrAt.secondsPerLiquidityCumulativeX128
        //                 ) * targetDelta) / observationTimeDelta
        //             )
        //     );
        // }


        if target == beforeOrAt.blockTimestamp {
            // we're at the left boundary
            return (
                beforeOrAt.tickCumulative,
                beforeOrAt.secondsPerLiquidityCumulativeX128.value,
            );
        } else if target == atOrAfter.blockTimestamp {
            // we're at the right boundary
            return (
                atOrAfter.tickCumulative,
                beforeOrAt.secondsPerLiquidityCumulativeX128.value,
            );
        } else {
            // we're in the middle
            let observationTimeDelta = atOrAfter.blockTimestamp - beforeOrAt.blockTimestamp;
            let targetDelta = target - beforeOrAt.blockTimestamp;
            // return (
            //     beforeOrAt.tickCumulative +
            //         ((atOrAfter.tickCumulative - beforeOrAt.tickCumulative) / observationTimeDelta) *
            //         targetDelta,
            //     beforeOrAt.secondsPerLiquidityCumulativeX128 +
            //         uint160(
            //             (uint256(
            //                 atOrAfter.secondsPerLiquidityCumulativeX128 - beforeOrAt.secondsPerLiquidityCumulativeX128
            //             ) * targetDelta) / observationTimeDelta
            //         )
            // );
            let tickCumulative = beforeOrAt.tickCumulative
                + ((atOrAfter.tickCumulative - beforeOrAt.tickCumulative)
                    / i64::try_from(observationTimeDelta).unwrap())
                    * i64::try_from(targetDelta).unwrap();

            let secondsPerLiquidityCumulativeX128 =
                beforeOrAt.secondsPerLiquidityCumulativeX128.value
                    + ((atOrAfter.secondsPerLiquidityCumulativeX128.value
                        - beforeOrAt.secondsPerLiquidityCumulativeX128.value)
                        * U160::from(targetDelta))
                        / U160::from(observationTimeDelta);

            return (tickCumulative, secondsPerLiquidityCumulativeX128);
        }
    }

    /// @notice Fetches the observations beforeOrAt and atOrAfter a given target, i.e. where [beforeOrAt, atOrAfter] is satisfied
    /// @dev Assumes there is at least 1 initialized observation.
    /// Used by observeSingle() to compute the counterfactual accumulator values as of a given block timestamp.
    /// @param self The stored oracle array
    /// @param time The current block.timestamp
    /// @param target The timestamp at which the reserved observation should be for
    /// @param tick The active tick at the time of the returned or simulated observation
    /// @param index The index of the observation that was most recently written to the observations array
    /// @param liquidity The total pool liquidity at the time of the call
    /// @param cardinality The number of populated elements in the oracle array
    /// @return beforeOrAt The observation which occurred at, or before, the given timestamp
    /// @return atOrAfter The observation which occurred at, or after, the given timestamp
    fn getSurroundingObservations(
        &self,
        time: u64,
        target: u64,
        tick: Int24,
        index: u16,
        liquidity: u128,
        cardinality: u16,
    ) -> (Observation, Observation) {
        // optimistically set before to the newest observation
        // beforeOrAt = self[index];
        let mut beforeOrAt: Observation = self.obs.get(index).expect("error!");
        let atOrAfter: Observation;
        // if the target is chronologically at or after the newest observation, we can early return
        if lte(time, beforeOrAt.blockTimestamp, target) {
            if beforeOrAt.blockTimestamp == target {
                // if newest observation equals target, we're in the same block, so we can ignore atOrAfter
                atOrAfter = Observation::default();
                return (beforeOrAt, atOrAfter);
            } else {
                // otherwise, we need to transform
                return (beforeOrAt, transform(&beforeOrAt, target, tick, liquidity));
            }
        }

        // now, set before to the oldest observation
        // beforeOrAt = self[(index + 1) % cardinality];
        beforeOrAt = self.obs.get((index + 1) % cardinality).expect("error!");
        // if (!beforeOrAt.initialized) beforeOrAt = self[0];
        if !beforeOrAt.initialized {
            beforeOrAt = self.obs.get(0_u16).expect("error!");
        }

        // ensure that the target is chronologically at or after the oldest observation
        assert!(lte(time, beforeOrAt.blockTimestamp, target), "OLD");

        // if we've reached this point, we have to binary search
        return self.binarySearch(time, target, index, cardinality);
    }

    /// @notice Fetches the observations beforeOrAt and atOrAfter a target, i.e. where [beforeOrAt, atOrAfter] is satisfied.
    /// The result may be the same observation, or adjacent observations.
    /// @dev The answer must be contained in the array, used when the target is located within the stored observation
    /// boundaries: older than the most recent observation and younger, or the same age as, the oldest observation
    /// @param self The stored oracle array
    /// @param time The current block.timestamp
    /// @param target The timestamp at which the reserved observation should be for
    /// @param index The index of the observation that was most recently written to the observations array
    /// @param cardinality The number of populated elements in the oracle array
    /// @return beforeOrAt The observation recorded before, or at, the target
    /// @return atOrAfter The observation recorded at, or after, the target
    fn binarySearch(
        &self,
        time: u64,
        target: u64,
        index: u16,
        cardinality: u16,
    ) -> (Observation, Observation) {
        // uint256 l = (index + 1) % cardinality; // oldest observation
        let mut l: U256 = U256::from((index + 1) % cardinality); // oldest observation
                                                                 // uint256 r = l + cardinality - 1; // newest observation
        let mut r: U256 = l + cardinality - 1; // newest observation
        let mut i: U256;
        let mut beforeOrAt: Observation;
        let mut atOrAfter: Observation;
        loop {
            i = (l + r) / 2;

            beforeOrAt = self
                .obs
                .get((i.as_usize() % usize::from(cardinality)) as u16)
                .expect("error!");

            // we've landed on an uninitialized tick, keep searching higher (more recently)
            if !beforeOrAt.initialized {
                l = i + 1;
                continue;
            }

            // atOrAfter = self[(i + 1) % cardinality];
            atOrAfter = self
                .obs
                .get(((i + 1).as_usize() % usize::from(cardinality)) as u16)
                .expect("error!");

            // bool targetAtOrAfter = lte(time, beforeOrAt.blockTimestamp, target);
            let targetAtOrAfter: bool = lte(time, beforeOrAt.blockTimestamp, target);

            // check if we've found the answer!
            // if (targetAtOrAfter && lte(time, target, atOrAfter.blockTimestamp)) break;
            if targetAtOrAfter && lte(time, target, atOrAfter.blockTimestamp) {
                break;
            }

            // if (!targetAtOrAfter) r = i - 1;
            if !targetAtOrAfter {
                r = i - 1;
            } else {
                l = i + 1;
            }
        }
        return (beforeOrAt, atOrAfter);
    }

    /// @notice Writes an oracle observation to the array
    /// @dev Writable at most once per block. Index represents the most recently written element. cardinality and index must be tracked externally.
    /// If the index is at the end of the allowable array length (according to cardinality), and the next cardinality
    /// is greater than the current one, cardinality may be increased. This restriction is created to preserve ordering.
    /// @param self The stored oracle array
    /// @param index The index of the observation that was most recently written to the observations array
    /// @param blockTimestamp The timestamp of the new observation
    /// @param tick The active tick at the time of the new observation
    /// @param liquidity The total in-range liquidity at the time of the new observation
    /// @param cardinality The number of populated elements in the oracle array
    /// @param cardinalityNext The new length of the oracle array, independent of population
    /// @return indexUpdated The new index of the most recently written element in the oracle array
    /// @return cardinalityUpdated The new cardinality of the oracle array
    pub fn write(
        &mut self,
        index: u16,
        blockTimestamp: u64,
        tick: Int24,
        liquidity: u128,
        cardinality: u16,
        cardinalityNext: u16,
    ) -> (u16, u16) {
        // Observation memory last = self[index];
        let last: Observation = self.obs.get(index).expect("error!");

        // early return if we've already written an observation this block
        // if (last.blockTimestamp == blockTimestamp) return (index, cardinality);
        if last.blockTimestamp == blockTimestamp {
            return (index, cardinality);
        }

        let cardinalityUpdated: u16;
        // if the conditions are right, we can bump the cardinality
        // if (cardinalityNext > cardinality && index == (cardinality - 1)) {
        //     cardinalityUpdated = cardinalityNext;
        // } else {
        //     cardinalityUpdated = cardinality;
        // }
        if cardinalityNext > cardinality && index == (cardinality - 1) {
            cardinalityUpdated = cardinalityNext;
        } else {
            cardinalityUpdated = cardinality;
        }

        // indexUpdated = (index + 1) % cardinalityUpdated;
        let indexUpdated = (index + 1) % cardinalityUpdated;
        // self[indexUpdated] = transform(last, blockTimestamp, tick, liquidity);
        self.obs.insert(
            indexUpdated,
            &transform(&last, blockTimestamp, tick, liquidity),
        );
        (indexUpdated, cardinalityUpdated)
    }

    /// @notice Prepares the oracle array to store up to `next` observations
    /// @param self The stored oracle array
    /// @param current The current next cardinality of the oracle array
    /// @param next The proposed next cardinality which will be populated in the oracle array
    /// @return next The next cardinality which will be populated in the oracle array
    pub fn grow(& mut self, current: u16, next: u16) -> u16 {
        // require(current > 0, 'I');
        assert!(current > 0, "I");
        // no-op if the passed next value isn't greater than the current next value
        // if (next <= current) return current;
        // for (uint16 i = current; i < next; i++) self[i].blockTimestamp = 1;
        if next <= current {
            return current;
        }
        // store in each slot to prevent fresh SSTOREs in swaps
        // this data will not be used because the initialized boolean is still false
        // for (uint16 i = current; i < next; i++) self[i].blockTimestamp = 1;
        for i in current..next {
            let mut observation = Observation::default();
            observation.blockTimestamp = 1;
            self.obs.insert(i,&observation);
        }
        // return next;
        return next;
    }

    /// @notice Returns the accumulator values as of each time seconds ago from the given time in the array of `secondsAgos`
    /// @dev Reverts if `secondsAgos` > oldest observation
    /// @param self The stored oracle array
    /// @param time The current block.timestamp
    /// @param secondsAgos Each amount of time to look back, in seconds, at which point to return an observation
    /// @param tick The current tick
    /// @param index The index of the observation that was most recently written to the observations array
    /// @param liquidity The current in-range pool liquidity
    /// @param cardinality The number of populated elements in the oracle array
    /// @return tickCumulatives The tick * time elapsed since the pool was first initialized, as of each `secondsAgo`
    /// @return secondsPerLiquidityCumulativeX128s The cumulative seconds / max(1, liquidity) since the pool was first initialized, as of each `secondsAgo`
    pub fn observe(
        &self,
        time:u64,
        secondsAgos:Vec<u64>,
        tick:Int24,
        index:u16,
        liquidity:u128,
        cardinality:u16
    )->(Vec<I56>, Vec<U160>) {
        // require(cardinality > 0, 'I');
        assert!(cardinality > 0, "I");

        // tickCumulatives = new int56[](secondsAgos.length);
        let mut tickCumulatives = <Vec::<I56>>::with_capacity(secondsAgos.len());
        // secondsPerLiquidityCumulativeX128s = new uint160[](secondsAgos.length);
        let mut secondsPerLiquidityCumulativeX128s =Vec::<U160>::with_capacity(secondsAgos.len());
        // for (uint256 i = 0; i < secondsAgos.length; i++) {
        //     (tickCumulatives[i], secondsPerLiquidityCumulativeX128s[i]) = observeSingle(
        //         self,
        //         time,
        //         secondsAgos[i],
        //         tick,
        //         index,
        //         liquidity,
        //         cardinality
        //     );
        // }
        for secondsAgo in secondsAgos {
            let (tickCumulative, secondsPerLiquidityCumulativeX128) = self.observeSingle(
                time,
                secondsAgo,
                tick,
                index,
                liquidity,
                cardinality
            );
            tickCumulatives.push(tickCumulative);
            secondsPerLiquidityCumulativeX128s.push(secondsPerLiquidityCumulativeX128);
        }
        (tickCumulatives,secondsPerLiquidityCumulativeX128s)
    }
}

/// @notice Transforms a previous observation into a new observation, given the passage of time and the current tick and liquidity values
/// @dev blockTimestamp _must_ be chronologically equal to or greater than last.blockTimestamp, safe for 0 or 1 overflows
/// @param last The specified observation to be transformed
/// @param blockTimestamp The timestamp of the new observation
/// @param tick The active tick at the time of the new observation
/// @param liquidity The total in-range liquidity at the time of the new observation
/// @return Observation The newly populated observation
fn transform(
    last: &Observation,
    blockTimestamp: u64,
    tick: Int24,
    mut liquidity: u128,
) -> Observation {
    // uint32 delta = blockTimestamp - last.blockTimestamp;
    let delta = blockTimestamp - last.blockTimestamp;
    // return
    //     Observation({
    //         blockTimestamp: blockTimestamp,
    //         tickCumulative: last.tickCumulative + int56(tick) * delta,
    //         secondsPerLiquidityCumulativeX128: last.secondsPerLiquidityCumulativeX128 +
    //             ((uint160(delta) << 128) / (liquidity > 0 ? liquidity : 1)),
    //         initialized: true
    //     });
    // here have very error! for !liquidity > 0 先对liquid做了取反的运算。导致liquid取反为负数。
    if !(liquidity > 0) {
        liquidity = 1;
    }
    let delta: i64 = delta.try_into().unwrap();
    Observation {
        blockTimestamp: blockTimestamp,
        tickCumulative: last.tickCumulative + (i64::from(tick) * delta),
        secondsPerLiquidityCumulativeX128: Uint160::new_with_u256(
            last.secondsPerLiquidityCumulativeX128
                .value
                .saturating_add((U256::from(delta) << 128) / liquidity),
        ),
        initialized: true,
    }
}

/// @notice comparator for 32-bit timestamps
/// @dev safe for 0 or 1 overflows, a and b _must_ be chronologically before or equal to time
/// @param time A timestamp truncated to 32 bits
/// @param a A comparison timestamp from which to determine the relative position of `time`
/// @param b From which to determine the relative position of `time`
/// @return bool Whether `a` is chronologically <= `b`
fn lte(time: u64, a: u64, b: u64) -> bool {
    // if there hasn't been overflow, no need to adjust
    // if (a <= time && b <= time) return a <= b;
    if a <= time && b <= time {
        return a <= b;
    }
    // uint256 aAdjusted = a > time ? a : a + 2**32;
    let aAdjusted: U256;
    if a > time {
        aAdjusted = U256::from(a);
    } else {
        aAdjusted = U256::from(a) + (U256::from(1) << 32);
    }
    // uint256 bAdjusted = b > time ? b : b + 2**32;
    let bAdjusted: U256;
    if b > time {
        bAdjusted = U256::from(b);
    } else {
        bAdjusted = U256::from(b) + U256::from(1) << 32;
    }
    return aAdjusted <= bAdjusted;
}
