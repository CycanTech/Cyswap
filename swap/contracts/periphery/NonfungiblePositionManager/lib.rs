#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod position_manager {
    use ink_storage::traits::SpreadAllocate;
    use crabswap::impls::pool_initialize::*;
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PoolInitializeStorage)]
    pub struct PositionManger {
        #[PoolInitializeStorageField]
        initializer: PoolInitializeData,
    }

    impl Initializer for PositionManger{}

    impl PositionManger {
        #[ink(constructor, payable)]
        pub fn new(factory: AccountId, weth9: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut PositionManger| {
                instance.initializer.factory = factory;
                instance.initializer.WETH9 = weth9;
            })
        }

    }
}
