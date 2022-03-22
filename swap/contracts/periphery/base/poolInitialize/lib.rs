#![cfg_attr(not(feature = "std"), no_std)]

// SPDX-License-Identifier: GPL-2.0-or-later
use ink_env::AccountId;
use ink_lang as ink;
// pub type U256 = [u8; 32];
/// @title Creates and initializes V3 Pools
/// @notice Provides a method for creating and initializing a pool, if necessary, for bundling with other methods that
/// require the pool to exist.
// #[ink::trait_definition]
// pub trait IPoolInitializer {
//     /// @notice Creates a new pool if it does not exist, then initializes if not initialized
//     /// @dev This method can be bundled with others via IMulticall for the first action (e.g. mint) performed against a pool
//     /// @param token0 The contract address of token0 of the pool
//     /// @param token1 The contract address of token1 of the pool
//     /// @param fee The fee amount of the v3 pool for the specified token pair
//     /// @param sqrtPriceX96 The initial square root price of the pool as a Q64.96 value
//     /// @return pool Returns the pool address based on the pair of tokens and fee, will return the newly created pool address if necessary
//     #[ink(message, payable)]
//     fn create_and_initialize_pool_if_necessary(
//         &self,
//         token0: AccountId,
//         token1: AccountId,
//         fee: u64,
//         sqrt_price_x96: U256,
//     ) -> AccountId;

//     // function createAndInitializePoolIfNecessary(
//     //     address token0,
//     //     address token1,
//     //     uint24 fee,
//     //     uint160 sqrtPriceX96
//     // ) external payable returns (address pool);
// }


#[brush::contract]
mod pool_initializer {
    use factory::uniswap_v3_factory::UniswapV3FactoryRef;
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PoolInitializeStorage)]
    pub struct PoolInitializer {
        #[PoolInitialStorageField]
        initialize: PoolInitializeData,
    }

    impl PoolInitializer {
        #[ink(constructor)]
        pub fn new(factory: UniswapV3FactoryRef, weth9: u32) -> Self {
            let instance = Self { factory, weth9 };
            instance
        }

        
    }
}
