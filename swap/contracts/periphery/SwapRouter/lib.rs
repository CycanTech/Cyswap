#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

#[brush::contract]
pub mod swapper_router {
    use crabswap::impls::erc721_permit::*;
    use crabswap::impls::periphery_immutable_state::{ImmutableStateStorage,ImmutableStateData};
    use crabswap::traits::periphery::periphery_immutable_state::*;
    use primitives::{Int24, Uint128, Uint24, Uint256, Uint80, Uint96, ADDRESS0,U160};
    use ink_storage::traits::{SpreadAllocate, SpreadLayout};
    use libs::PoolKey;
    use primitives::{Address, U256};
    use scale::{Decode, Encode};

    use crabswap::traits::periphery::swap_router::*;

    #[derive(Default, Decode, Encode, Debug, SpreadAllocate, SpreadLayout)]
    struct MintCallbackData {
        poolKey: PoolKey,
        payer: Address,
    }

    /// @dev Used as the placeholder value for amountInCached, because the computed amount in for an exact output swap
    /// can never actually be this value
    const DEFAULT_AMOUNT_IN_CACHED: U256 = U256::MAX;

    #[ink(storage)]
    #[derive(SpreadAllocate, ImmutableStateStorage)]
    pub struct SwapRouterContract {
        #[ImmutableStateField]
        immutable_state: ImmutableStateData,

        /// @dev Transient storage variable used for returning the computed amount in for an exact output swap.
        amountInCached: Uint256,
    }

    impl PeripheryImmutableState for SwapRouterContract{}

    impl SwapRouter for SwapRouterContract {
        #[ink(message, payable)]
        fn exactInputSingle(
            &mut self,
            tokenIn: Address,
            tokenOut: Address,
            fee: Uint24,
            recipient: Address,
            deadline: U256,
            amountIn: U256,
            amountOutMinimum: U256,
            sqrtPriceLimitX96: U160,
        ) -> U256{
            U256::from(0)
        }
    }

    impl SwapRouterContract {
        #[ink(constructor, payable)]
        pub fn new(factory: AccountId, weth9: AccountId, _tokenDescriptor: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut SwapRouterContract| {
                instance.immutable_state.factory = factory;
                instance.immutable_state.WETH9 = weth9;
                instance.amountInCached = Uint256::new_with_u256(DEFAULT_AMOUNT_IN_CACHED);
            })
        }
    }

}
