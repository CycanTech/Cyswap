#![cfg_attr(not(feature = "std"), no_std)]

// SPDX-License-Identifier: GPL-2.0-or-later
use ink_env::AccountId;
use ink_lang as ink;
use sp_core::U256;
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

static ADDRESS0:[u8;32] = [0;32]; 

#[ink::contract]
mod pool_initializer {
    use factory::UniswapV3FactoryRef;
    #[ink(storage)]
    pub struct PoolInitializer {
        /// @inheritdoc IPeripheryImmutableState
        pub factory: UniswapV3FactoryRef,
        /// @inheritdoc IPeripheryImmutableState
        pub weth9: u32,
    }

    impl PoolInitializer {
        #[ink(constructor)]
        pub fn new(factory: UniswapV3FactoryRef, weth9: u32) -> Self {
            let instance = Self { factory, weth9 };
            instance
        }

        /// @inheritdoc IPoolInitializer
        #[ink(message, payable)]
        pub fn create_and_initialize_pool_if_necessary(
            &mut self,
            token0: AccountId,
            token1: AccountId,
            fee: u32,
            sqrtPriceX96: sp_core::U256,
        ) -> u32 {
            let accumulator = UniswapV3FactoryRef::new()
                .endowment(100 / 4)
                .code_hash(Default::default())
                .salt_bytes([0;32])
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "failed at instantiating the Accumulator contract: {:?}",
                        error
                    )
                });

            assert!(token0<token1);
            let pool = self.factory.get_pool(token0,token1,fee);
            if pool == crate::ADDRESS0.into(){
                self.factory.create_pool(token0,token1,fee);
            }
            // require(token0 < token1);
            // pool = IUniswapV3Factory(factory).getPool(token0, token1, fee);

            // if (pool == address(0)) {
            //     pool = IUniswapV3Factory(factory).createPool(token0, token1, fee);
            //     IUniswapV3Pool(pool).initialize(sqrtPriceX96);
            // } else {
            //     (uint160 sqrtPriceX96Existing, , , , , , ) = IUniswapV3Pool(pool).slot0();
            //     if (sqrtPriceX96Existing == 0) {
            //         IUniswapV3Pool(pool).initialize(sqrtPriceX96);
            //     }
            // }
            0u32
        }
    }
}
