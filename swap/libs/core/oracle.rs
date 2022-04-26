#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

#[cfg(feature = "std")]
use ink_metadata::layout::{FieldLayout, Layout, StructLayout};
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{SpreadAllocate, SpreadLayout};
use primitives::{Int24, Uint256, U160, U256};

/// @title Oracle
/// @notice Provides price and liquidity data useful for a wide variety of system designs
/// @dev Instances of stored oracle data, "observations", are collected in the oracle array
/// Every pool is initialized with an oracle array length of 1. Anyone can pay the SSTOREs to increase the
/// maximum length of the oracle array. New slots will be added when the array is fully populated.
/// Observations are overwritten when the full length of the oracle array is populated.
/// The most recent observation is available, independent of the length of the oracle array, by passing 0 to observe()
#[derive(Default, Debug, Copy, Clone, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct Observation {
    // the block timestamp of the observation
    pub blockTimestamp: u64,
    // the tick accumulator, i.e. tick * time elapsed since the pool was first initialized
    pub tickCumulative: i64,
    // the seconds per liquidity, i.e. seconds elapsed / max(1, liquidity) since the pool was first initialized
    pub secondsPerLiquidityCumulativeX128: u128,
    // whether or not the observation is initialized
    pub initialized: bool,
}

#[derive(Debug, SpreadAllocate, SpreadLayout)]
// #[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct Observations {
    pub obs: [Observation; 16],
    // pub obs: [Observation;65535],
}

#[cfg(feature = "std")]
impl StorageLayout for Observations {
    fn layout(key_ptr: &mut ink_primitives::KeyPtr) -> Layout {
        Layout::Struct(StructLayout::new([
            FieldLayout::new("len", <[Observation; 16] as StorageLayout>::layout(key_ptr)),
            FieldLayout::new(
                "elems",
                <[Observation; 16] as StorageLayout>::layout(key_ptr),
            ),
        ]))
    }
}

impl Observations {
    pub fn new() -> Self {
        let observation: Observation = Default::default();
        Observations {
            obs: [observation; 16],
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
        self.obs[0] = Observation {
            blockTimestamp: time,
            tickCumulative: 0,
            secondsPerLiquidityCumulativeX128: 0,
            initialized: true,
        };
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
    fn observeSingle(
        &mut self,
        time: u64,
        secondsAgo: u64,
        tick: Int24,
        index: u16,
        liquidity: u128,
        cardinality: u16,
    ) -> (i64, U160) {
        if secondsAgo == 0 {
            let mut last: Observation = self.obs[usize::from(index)];
            // if (last.blockTimestamp != time) last = transform(last, time, tick, liquidity);
            if last.blockTimestamp != time {
                last = transform(&last, time, tick, liquidity);
            }
            return (
                last.tickCumulative,
                U160::from(last.secondsPerLiquidityCumulativeX128),
            );
        }

        let target: u64 = time - secondsAgo;

        // (Observation memory beforeOrAt, Observation memory atOrAfter) =
        //     getSurroundingObservations(, time, target, tick, index, liquidity, cardinality);
        let (beforeOrAt, atOrAfter) =
            self.getSurroundingObservations(time, target, tick, index, liquidity, cardinality);

        if (target == beforeOrAt.blockTimestamp) {
            // we're at the left boundary
            return (
                beforeOrAt.tickCumulative,
                U160::from(beforeOrAt.secondsPerLiquidityCumulativeX128),
            );
        } else if (target == atOrAfter.blockTimestamp) {
            // we're at the right boundary
            return (
                atOrAfter.tickCumulative,
                U160::from(atOrAfter.secondsPerLiquidityCumulativeX128),
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
            let secondsPerLiquidityCumulativeX128 = U256::from(
                beforeOrAt.secondsPerLiquidityCumulativeX128
                    + ((atOrAfter.secondsPerLiquidityCumulativeX128
                        - beforeOrAt.secondsPerLiquidityCumulativeX128)
                        * u128::from(targetDelta))
                        / u128::from(observationTimeDelta),
            );
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
        let mut beforeOrAt: Observation = self.obs[usize::from(index)];
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
        beforeOrAt = self.obs[usize::from((index + 1) % cardinality)];
        // if (!beforeOrAt.initialized) beforeOrAt = self[0];
        if !beforeOrAt.initialized {
            beforeOrAt = self.obs[0usize];
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

            beforeOrAt = self.obs[i.as_usize() % usize::from(cardinality)];

            // we've landed on an uninitialized tick, keep searching higher (more recently)
            if !beforeOrAt.initialized {
                l = i + 1;
                continue;
            }

            // atOrAfter = self[(i + 1) % cardinality];
            atOrAfter = self.obs[(i + 1).as_usize() % usize::from(cardinality)];

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

    if !liquidity > 0 {
        liquidity = 1;
    }
    let delta: i64 = delta.try_into().unwrap();
    Observation {
        blockTimestamp: blockTimestamp,
        tickCumulative: last.tickCumulative + (i64::from(tick) * delta),
        secondsPerLiquidityCumulativeX128: U256::from(last.secondsPerLiquidityCumulativeX128)
            .saturating_add((U256::from(delta) << 128) / liquidity)
            .as_u128(),
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
