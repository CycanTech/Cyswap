#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod position_manager {
    use brush::contracts::psp34::PSP34Data;
    use crabswap::impls::periphery::position_manager::*;
    use ink_storage::traits::SpreadAllocate;
    use crabswap::impls::pool_initialize::*;
    use crabswap::impls::erc721_permit::*;
    use crabswap::impls::psp34_base::*;
    use brush::contracts::psp34::*;
    use brush::contracts::psp34::extensions::mintable::*;
    use brush::contracts::psp34::extensions::burnable::*;
    use ink_prelude::string::String;
    use ink_lang::codegen::Env;
    use ink_lang::codegen::EmitEvent;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate,PositionStorage, PoolInitializeStorage,PSP34Storage,ERC721PermitStorage,PSP34BaseStorage)]
    pub struct PositionMangerContract {
        #[PoolInitializeStorageField]
        initializer: PoolInitializeData,
        #[PSP34StorageField]
        psp34:PSP34Data,
        #[ERC721PermitStorageField]
        erc721_permit:ERC721PermitData,
        #[PSP34BaseStorageField]
        psp34_base:PSP34BaseData,

        #[PositionStorageField]
        position:PositionData,
        // /// @dev The address of the token descriptor contract, which handles generating token URIs for position tokens
        // address private immutable _tokenDescriptor;
        tokenDescriptor:AccountId,
        // // field for testing _before_token_transfer
        // return_err_on_before: bool,
        // // field for testing _after_token_transfer
        // return_err_on_after: bool,
    }

    impl Initializer for PositionMangerContract{}
    impl PSP34 for PositionMangerContract{}
    impl PSP34Mintable for PositionMangerContract{}
    impl PSP34Burnable for PositionMangerContract{}
    impl IERC721Permit for PositionMangerContract{}
    impl PSP34Base for PositionMangerContract{}
    impl PositionManager for PositionMangerContract{}
    
    impl PositionMangerContract {
        #[ink(constructor, payable)]
        pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut PositionMangerContract| {
                instance.initializer.factory = factory;
                instance.initializer.WETH9 = weth9;
                let name = "Crabswap V3 Positions NFT-V1";
                let symbol = "Crab-V3-POS";
                // let version = "1";
                instance.erc721_permit.nameHash = ink_lang::blake2x256!("Crabswap V3 Positions NFT-V1");
                instance.erc721_permit.versionHash = ink_lang::blake2x256!("1");
                instance.psp34_base.name = String::from(name);
                instance.psp34_base.symbol = String::from(symbol);
                instance.tokenDescriptor = tokenDescriptor;
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

    impl PSP34Internal for PositionMangerContract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
            self.env().emit_event(Approval { from, to, id, approved });
        }
    }

}
