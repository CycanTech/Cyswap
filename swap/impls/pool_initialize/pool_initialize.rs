pub use super::data::*;
pub use crate::traits::periphery::pool_initialize::*;

use brush::{
    traits::{
        AccountId,
    },
};
use primitives::{Uint160, ADDRESS0};
use crate::traits::core::factory::*;
use crate::traits::core::pool::*;

impl<T:PoolInitializeStorage> Initializer for T{
    default fn create_and_initialize_pool_if_necessary(
        &mut self,
        token0: AccountId,
        token1: AccountId,
        fee: u32,
        sqrt_price_x96: Uint160,
    ) -> u32 {
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



        assert!(token0<token1,"token0 must less than token1");
        let factory_address = self.get().factory;
        let mut pool_address = FactoryRef::get_pool(&factory_address,fee,token0,token1);
        if pool_address == ADDRESS0.into() {
            pool_address = FactoryRef::create_pool(&factory_address,fee,token0,token1);
            PoolRef::initialize(&mut pool_address,sqrt_price_x96);
        }
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

        // let pool = self.factory.get_pool(fee,token0,token1,);
        // if pool == crate::ADDRESS0.into(){
        //     self.factory.create_pool(fee,token0,token1,);
        // }
        0u32
    }
}