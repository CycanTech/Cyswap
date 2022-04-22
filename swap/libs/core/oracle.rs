#![cfg_attr(not(feature = "std"), no_std)]

#![allow(non_snake_case)]
#[cfg(feature = "std")]
use ink_metadata::layout::{Layout, StructLayout, FieldLayout};
use ink_storage::traits::{SpreadAllocate, SpreadLayout};
use primitives::{U160, Uint256};
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

/// @title Oracle
/// @notice Provides price and liquidity data useful for a wide variety of system designs
/// @dev Instances of stored oracle data, "observations", are collected in the oracle array
/// Every pool is initialized with an oracle array length of 1. Anyone can pay the SSTOREs to increase the
/// maximum length of the oracle array. New slots will be added when the array is fully populated.
/// Observations are overwritten when the full length of the oracle array is populated.
/// The most recent observation is available, independent of the length of the oracle array, by passing 0 to observe()
#[derive(Default, Debug,Copy,Clone, SpreadAllocate, SpreadLayout)]
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
    pub obs: [Observation;16],
    // pub obs: [Observation;65535],
}

#[cfg(feature = "std")]
impl StorageLayout for Observations {
    fn layout(key_ptr: &mut ink_primitives::KeyPtr) -> Layout {
        Layout::Struct(StructLayout::new([
            FieldLayout::new(
                "len",
                <[Observation; 16] as StorageLayout>::layout(key_ptr),
            ),
            FieldLayout::new("elems", <[Observation; 16] as StorageLayout>::layout(key_ptr)),
        ]))
    }
}

impl Observations {
    pub fn new()->Self{
        let observation:Observation = Default::default();
        Observations{
            obs:[observation;16],
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
}
