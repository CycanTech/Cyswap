#![cfg_attr(not(feature = "std"), no_std)]

// SPDX-License-Identifier: GPL-2.0-or-later
use ink_lang as ink;

pub use self::uniswap_v3_pool::{
    UniswapV3Pool,
    UniswapV3PoolRef,
};

#[ink::contract]
mod uniswap_v3_pool {
    use ink_storage::{lazy::Mapping, traits::{SpreadLayout, PackedLayout, StorageLayout}};
    use scale::{Encode, Decode, WrapperTypeEncode};
    use sp_core::U256;
    type Address = AccountId;
    type Uint24 = u32;
    type Int24 = i32;
    type Uint160 = WrapperU256;

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct WrapperU256{
        value:U256,
    }

    

    impl SpreadLayout for WrapperU256{
        const FOOTPRINT: u64=4;

        const REQUIRES_DEEP_CLEAN_UP: bool = true;

        fn pull_spread(ptr: &mut ink_primitives::KeyPtr) -> Self {
            let slice:[u64;4] = SpreadLayout::pull_spread(ptr); 
            Self{
                value: U256(slice),
            }
        }

        fn push_spread(&self, ptr: &mut ink_primitives::KeyPtr) {
            SpreadLayout::push_spread(&self.value.0, ptr);
        }

        fn clear_spread(&self, ptr: &mut ink_primitives::KeyPtr) {
            SpreadLayout::clear_spread(&self.value.0, ptr);
        }
    }
    

    impl PackedLayout for WrapperU256{
        fn pull_packed(&mut self, at: &ink_primitives::Key) {
            self.value.0.pull_packed(at);
        }

        fn push_packed(&self, at: &ink_primitives::Key) {
            self.value.0.push_packed(at);
        }

        fn clear_packed(&self, at: &ink_primitives::Key) {
            self.clear_packed(at);
        }
    }

    impl StorageLayout for WrapperU256{
        fn layout(key_ptr: &mut ink_primitives::KeyPtr) -> ink_metadata::layout::Layout {
            // WrapperU256::layout(key_ptr)
        }
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    struct Slot0 {
        // the current price
        sqrtPriceX96:Uint160,
        // the current tick
        tick:Int24,
        // the most-recently updated index of the observations array
        observationIndex:u16,
        // the current maximum number of observations that are being stored
        observationCardinality:u16,
        // the next maximum number of observations to store, triggered in observations.write
        observationCardinalityNext:u16,
        // the current protocol fee as a percentage of the swap fee taken on withdrawal
        // represented as an integer denominator (1/x)%
        feeProtocol:u8,
        // whether the pool is locked
        unlocked:bool,
    }

    #[ink(storage)]
    pub struct UniswapV3Pool {
        // address public immutable override factory;
        // address public immutable override token0;
        // address public immutable override token1;
        // uint24 public immutable override fee;
        // int24 public immutable override tickSpacing;
        // uint128 public immutable override maxLiquidityPerTick;

        // follow six parameter is immutable
        pub factory:Address,
        pub token0:Address,
        pub token1:Address,
        pub fee:Uint24,
        pub tickSpacing:Int24,
        pub maxLiquidityPerTick:u128,
        pub slot0:Slot0,
    }

    impl UniswapV3Pool {
        #[ink(constructor)]
        pub fn new() -> Self {
            let i = [3u8;20];
            let instance = Self {
            };
            instance
        }

        

        /// @inheritdoc IUniswapV3Factory
        #[ink(message)]
        pub fn create_pool(&mut self,tokenA:Address,tokenB:Address,fee:u32)->AccountId{
            
            [0;32].into()
        }

      
    }

}
