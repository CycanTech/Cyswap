#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::AccountId;
// SPDX-License-Identifier: GPL-2.0-or-later

use ink_storage::{
    traits::{PackedLayout, SpreadLayout, StorageLayout},
};
use scale::{Decode, Encode};
use sp_core::U256;

#[cfg(feature = "std")]
use ink_metadata::layout::{FieldLayout, Layout, StructLayout};

pub type Address = AccountId;
pub type Uint24 = u32;
pub type Int24 = i32;
pub type Uint160 = WrapperU256;

pub static ADDRESS0:[u8;32] = [0u8;32];

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct WrapperU256 {
    pub value: U256,
}

impl SpreadLayout for WrapperU256 {
    const FOOTPRINT: u64 = 4;

    const REQUIRES_DEEP_CLEAN_UP: bool = true;

    fn pull_spread(ptr: &mut ink_primitives::KeyPtr) -> Self {
        let slice: [u64; 4] = SpreadLayout::pull_spread(ptr);
        Self { value: U256(slice) }
    }

    fn push_spread(&self, ptr: &mut ink_primitives::KeyPtr) {
        SpreadLayout::push_spread(&self.value.0, ptr);
    }

    fn clear_spread(&self, ptr: &mut ink_primitives::KeyPtr) {
        SpreadLayout::clear_spread(&self.value.0, ptr);
    }
}

impl PackedLayout for WrapperU256 {
    fn pull_packed(&mut self, at: &ink_primitives::Key) {
        self.value.0.pull_packed(at);
    }

    fn push_packed(&self, at: &ink_primitives::Key) {
        self.value.0.push_packed(at);
    }

    fn clear_packed(&self, at: &ink_primitives::Key) {
        self.value.0.clear_packed(at);
    }
}

#[cfg(feature = "std")]
impl StorageLayout for WrapperU256 {
    fn layout(key_ptr: &mut ink_primitives::KeyPtr) -> Layout {
        Layout::Struct(StructLayout::new([
            FieldLayout::new(
                "len",
                <[u32; 4] as StorageLayout>::layout(key_ptr),
            ),
            FieldLayout::new("elems", <[u32; 6] as StorageLayout>::layout(key_ptr)),
        ]))
    }
}

// #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
// struct Slot0 {
//     // the current price
//     sqrtPriceX96: Uint160,
//     // the current tick
//     tick: Int24,
//     // the most-recently updated index of the observations array
//     observationIndex: u16,
//     // the current maximum number of observations that are being stored
//     observationCardinality: u16,
//     // the next maximum number of observations to store, triggered in observations.write
//     observationCardinalityNext: u16,
//     // the current protocol fee as a percentage of the swap fee taken on withdrawal
//     // represented as an integer denominator (1/x)%
//     feeProtocol: u8,
//     // whether the pool is locked
//     unlocked: bool,
// }

// // accumulated protocol fees in token0/token1 units
// #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
// struct ProtocolFees {
//     token0: u128,
//     token1: u128,
// }
