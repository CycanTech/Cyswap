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
    use crabswap::impls::erc721_permit::*;
    use brush::contracts::psp34::*;
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PoolInitializeStorage,PSP34Storage,ERC721PermitStorage)]
    pub struct PositionManger {
        #[PoolInitializeStorageField]
        initializer: PoolInitializeData,
        #[PSP34StorageField]
        psp34:PSP34Data,
        #[ERC721PermitStorageField]
        erc721_permit:ERC721PermitData,
    }

    impl Initializer for PositionManger{}
    impl PSP34 for PositionManger{}
    impl IERC721Permit for PositionManger{}
    
    impl PositionManger {
        #[ink(constructor, payable)]
        pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut PositionManger| {
                instance.initializer.factory = factory;
                instance.initializer.WETH9 = weth9;
                let name = "Uniswap V3 Positions NFT-V1";
                let symbol = "UNI-V3-POS";
                let version = "1";
                instance.erc721_permit.nameHash = ink_lang::blake2x256!("Uniswap V3 Positions NFT-V1");
                instance.erc721_permit.versionHash = ink_lang::blake2x256!("1");
                // instance.psp34.
            })
        }

    }
}
