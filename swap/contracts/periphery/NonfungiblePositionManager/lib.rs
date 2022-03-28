#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod position_manager {
    use brush::contracts::psp34::PSP34Data;
    use ink_storage::traits::SpreadAllocate;
    use crabswap::impls::pool_initialize::PoolInitializeStorage;
    use crabswap::impls::pool_initialize::PoolInitializeData;
    use crabswap::impls::pool_initialize::Initializer;
    use crabswap::impls::pool_initialize::initializer_external;
    use brush::contracts::psp34::*;
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PoolInitializeStorage,PSP34Storage)]
    pub struct PositionManger {
        #[PoolInitializeStorageField]
        initializer: PoolInitializeData,
        #[PSP34StorageField]
        psp34:PSP34Data,
    }

    impl Initializer for PositionManger{}
    impl PSP34 for PositionManger{}

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
