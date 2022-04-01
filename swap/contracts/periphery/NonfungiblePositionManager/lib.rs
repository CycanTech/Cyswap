#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod position_manager {
    use brush::contracts::psp34::PSP34Data;
    use ink_storage::traits::SpreadAllocate;
    use crabswap::impls::pool_initialize::{Initializer,PoolInitializeData,PoolInitializeStorage,initializer_external};
    use crabswap::impls::erc721_permit::*;
    use crabswap::impls::psp34_base::*;
    use brush::contracts::psp34::*;
    use brush::contracts::psp34::extensions::mintable::*;
    use brush::contracts::psp34::extensions::burnable::*;
    use ink_prelude::string::String;
    use ink_lang::codegen::Env;
    use ink_lang::codegen::EmitEvent;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PoolInitializeStorage,PSP34Storage,ERC721PermitStorage,PSP34BaseStorage)]
    pub struct PositionManger {
        #[PoolInitializeStorageField]
        initializer: PoolInitializeData,
        #[PSP34StorageField]
        psp34:PSP34Data,
        #[ERC721PermitStorageField]
        erc721_permit:ERC721PermitData,
        #[PSP34BaseStorageField]
        psp34_base:PSP34BaseData,
        // // field for testing _before_token_transfer
        // return_err_on_before: bool,
        // // field for testing _after_token_transfer
        // return_err_on_after: bool,
    }

    impl Initializer for PositionManger{}
    impl PSP34 for PositionManger{}
    impl PSP34Mintable for PositionManger{}
    impl PSP34Burnable for PositionManger{}
    impl IERC721Permit for PositionManger{}
    impl PSP34Base for PositionManger{}
    
    impl PositionManger {
        #[ink(constructor, payable)]
        pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut PositionManger| {
                instance.initializer.factory = factory;
                instance.initializer.WETH9 = weth9;
                let name = "Uniswap V3 Positions NFT-V1";
                let symbol = "UNI-V3-POS";
                // let version = "1";
                instance.erc721_permit.nameHash = ink_lang::blake2x256!("Uniswap V3 Positions NFT-V1");
                instance.erc721_permit.versionHash = ink_lang::blake2x256!("1");
                instance.psp34_base.name = String::from(name);
                instance.psp34_base.symbol = String::from(symbol);
            })
        }
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    impl PSP34Internal for PositionManger {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
            self.env().emit_event(Approval { from, to, id, approved });
        }
    }

}
