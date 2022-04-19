#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod PositionDescriptor {
    use ink_storage::traits::SpreadAllocate;
    use crabswap::traits::periphery::position_descriptor::*;
    use primitives::Address;
    use primitives::Uint256;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct PositionDescriptor {
    }

    impl Descriptor for PositionDescriptor{
        #[ink(message)]
        fn tokenURI(&self,positionManager:Address, tokenId:Uint256) -> String{
            String::from("")
        }
    }
    
    impl PositionDescriptor {
        #[ink(constructor, payable)]
        pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut PositionDescriptor| {
            })
        }
    }
}