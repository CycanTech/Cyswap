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
    use scale::{Encode, Decode};
    type Address = AccountId;
    type Uint24 = u32;
    type Int24 = i32;

    #[ink(storage)]
    pub struct UniswapV3Pool {

    }

    impl UniswapV3Pool {
        #[ink(constructor)]
        pub fn new() -> Self {
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
