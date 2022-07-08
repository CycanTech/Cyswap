#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

/// This is a simple `PSP-22` which will be used as a stable coin and a collateral token in our lending contract
#[openbrush::contract]
pub mod tick_math_test {
    use ink_env::DefaultEnvironment;
    // use lending_project::traits::stable_coin::*;
    use ink_storage::traits::SpreadAllocate;
    use libs::core::TickMath;
    use primitives::{Int24, U160, U256};
    

    /// Define the storage for PSP22 data and Metadata data
    #[ink(storage)]
    #[derive(Default, SpreadAllocate,)]
    pub struct TickMathTestContract {
    }


    // It forces the compiler to check that you implemented all super traits
    // impl StableCoin for StableCoinContract {}

    impl TickMathTestContract {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut TickMathTestContract| {
                instance;
            })
        }

        #[ink(message)]
        pub fn getSqrtRatioAtTick(&self,tick:Int24)->U160 {
            return TickMath::getSqrtRatioAtTick(tick);
        }

        #[ink(message)]
        pub fn getGasCostOfGetSqrtRatioAtTick(&self,tick:Int24)-> u64 {
            // uint256 gasBefore = gasleft();
            let gasBefore = ink_env::gas_left::<DefaultEnvironment>();
            TickMath::getSqrtRatioAtTick(tick);
            return gasBefore - ink_env::gas_left::<DefaultEnvironment>();
        }
    
        #[ink(message)]
        pub fn getTickAtSqrtRatio(&self,sqrtPriceX96:U160) -> Int24 {
            return TickMath::getTickAtSqrtRatio(sqrtPriceX96);
        }
    
        #[ink(message)]
        pub fn getGasCostOfGetTickAtSqrtRatio(&self,sqrtPriceX96:U160)-> u64 {
            // uint256 gasBefore = gasleft();
            // TickMath.getTickAtSqrtRatio(sqrtPriceX96);
            // return gasBefore - gasleft();
            let gasBefore = ink_env::gas_left::<DefaultEnvironment>();
            TickMath::getTickAtSqrtRatio(sqrtPriceX96);
            return gasBefore - ink_env::gas_left::<DefaultEnvironment>();
        }
        #[ink(message)]
        pub fn MIN_SQRT_RATIO(&self) ->U160 {
            return U256::from_dec_str(TickMath::MIN_SQRT_RATIO).expect("Error!");
        }
    
        #[ink(message)]
        pub fn MAX_SQRT_RATIO(&self) ->U160 {
            return U256::from_dec_str(TickMath::MAX_SQRT_RATIO).expect("Errro!");
        }
    }

}
