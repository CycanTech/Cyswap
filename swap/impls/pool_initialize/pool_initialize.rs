pub use super::data::*;
pub use crate::traits::pool_initialize::*;

use brush::{
    contracts::{
        traits::{
            psp34::Id,
        },
    },
    modifiers,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Timestamp,
        ZERO_ADDRESS,
    },
};
use ink_prelude::vec::Vec;

impl<T:PoolInitializeStorage> Initializer for T{
    default fn create_and_initialize_pool_if_necessary(
        &mut self,
        token0: AccountId,
        token1: AccountId,
        fee: u32,
        sqrtPriceX96: u128,
    ) -> u32 {
        // let accumulator = UniswapV3FactoryRef::new()
        //     .endowment(100 / 4)
        //     .code_hash(Default::default())
        //     .salt_bytes([0;32])
        //     .instantiate()
        //     .unwrap_or_else(|error| {
        //         panic!(
        //             "failed at instantiating the Accumulator contract: {:?}",
        //             error
        //         )
        //     });

        // assert!(token0<token1);
        // let pool = self.factory.get_pool(fee,token0,token1,);
        // if pool == crate::ADDRESS0.into(){
        //     self.factory.create_pool(fee,token0,token1,);
        // }
        0u32
    }
}