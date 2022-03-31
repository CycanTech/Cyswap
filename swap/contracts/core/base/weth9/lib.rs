#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This is a simple `PSP-22` which will be used as a stable coin and a collateral token in our lending contract
#[brush::contract]
pub mod weth9 {
    use brush::contracts::psp22::extensions::metadata::*;
    use ink_prelude::string::String;
    // use lending_project::traits::stable_coin::*;
    use ink_storage::traits::SpreadAllocate;
    use crabswap::impls::weth9::*;

    /// Define the storage for PSP22 data and Metadata data
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage)]
    pub struct Weth9Contract {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    /// implement PSP22 Trait for our coin
    impl PSP22 for Weth9Contract {}

    /// implement PSP22Metadata Trait for our coin
    impl PSP22Metadata for Weth9Contract {}

    impl Weth9 for Weth9Contract{}

    // It forces the compiler to check that you implemented all super traits
    // impl StableCoin for StableCoinContract {}

    impl Weth9Contract {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Weth9Contract| {
                instance.metadata.name = name;
                instance.metadata.symbol = symbol;
                instance.metadata.decimals = 12;
                // let total_supply = 1_000_000 * 10_u128.pow(12);
                // assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            })
        }
    }
}
