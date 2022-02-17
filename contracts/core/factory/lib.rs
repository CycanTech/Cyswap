#![cfg_attr(not(feature = "std"), no_std)]

// SPDX-License-Identifier: GPL-2.0-or-later
use ink_lang as ink;

pub use self::uniswap_v3_factory::{
    UniswapV3Factory,
    UniswapV3FactoryRef,
};

#[ink::contract]
mod uniswap_v3_factory {
    use ink_storage::lazy::Mapping;
    #[ink(storage)]
    pub struct UniswapV3Factory {
        // mapping(address => mapping(address => mapping(uint24 => address))) public override getPool;
        /// @inheritdoc IPeripheryImmutableState
        pub pool: Mapping<(AccountId,AccountId,u32),AccountId>,
    }

    impl UniswapV3Factory {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {
                pool:Default::default(),
            };
            instance
        }

        #[ink(message)]
        pub fn get_pool(&self,token0:AccountId, token1:AccountId, fee:u32)->AccountId{
            let key = (token0,token1,fee);
            self.pool.get(key).unwrap_or([0u8;32].into())
        }
    }

}
