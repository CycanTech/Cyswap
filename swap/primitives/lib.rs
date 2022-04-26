#![cfg_attr(not(feature = "std"), no_std)]


use ink_env::AccountId;
// SPDX-License-Identifier: GPL-2.0-or-later

#[cfg(feature = "std")]
use ink_metadata::layout::{StructLayout, Layout, FieldLayout};
use ink_storage::{
    traits::{PackedLayout, SpreadLayout, StorageLayout, SpreadAllocate,ExtKeyPtr},
};
// use primitive_types::U256;
use scale::{Decode, Encode};


#[cfg(feature = "std")]
use scale_info::{TypeInfo, Type};
pub use sp_core::U256;
pub type Address = AccountId;
pub type Uint24 = u32;
pub type Uint16 = u16;
pub type Int24 = i32;
pub type Uint8 = u8;
pub type Uint160 = WrapperU256;
pub type Uint256 = WrapperU256;
pub type U160 = U256;
pub type I56 = i64;
pub type I256 = i128;
pub type Uint128 = u128;
pub type Uint96 = u128;
pub type Uint80 = u128;


pub const ADDRESS0:[u8;32] = [0u8;32];

#[derive(Default,Debug,Clone, PartialEq, Eq,Encode, Decode)]
// #[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct WrapperU256 {
    pub value: U256,
}

impl WrapperU256{
    pub fn new()->Self{
        WrapperU256{
            value:U256::zero(),
        }
    }

    pub fn new_with_u256(v:U256)->Self{
        WrapperU256{
            value:v,
        }
    }
}

impl AsRef<U256> for WrapperU256 {
    #[inline]
    fn as_ref(&self) ->  &U256{
      &self.value
    }
}

#[cfg(feature = "std")]
impl TypeInfo for WrapperU256
{
    type Identity = [u64];

    fn type_info() -> Type {
        Self::Identity::type_info()
    }
}

// #[cfg(feature = "std")]
// impl WrapperTypeEncode for WrapperU256{
    
// }

// #[cfg(feature = "std")]
// impl Deref for WrapperU256{
//     type Target=[u64];

//     fn deref(&self) -> &Self::Target {
//         self.value.as_ref()
//     }
// }

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

impl SpreadAllocate for WrapperU256{
    fn allocate_spread(ptr: &mut ink_primitives::KeyPtr) -> Self {
        ptr.next_for::<WrapperU256>();
        // Id::U8(0)
        WrapperU256::new()
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