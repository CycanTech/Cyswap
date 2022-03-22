#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod position_manager {
    #[ink(storage)]
    pub struct PositionManger {
    }

    impl PositionManger {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {};
            instance
        }

        /// @inheritdoc IPoolInitializer
        #[ink(message)]
        pub fn cache_pool_key(&self) -> u32 {
            0u32
        }
    }
}
