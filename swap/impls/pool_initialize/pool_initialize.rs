pub use super::data::*;
pub use crate::traits::periphery::pool_initialize::*;

use brush::{
    traits::{
        AccountId,
    },
};
use ink_env::DefaultEnvironment;
use primitives::{ ADDRESS0,  Address, U160};
use crate::traits::core::factory::*;
use crate::traits::core::pool::*;

impl<T:PoolInitializeStorage> Initializer for T{
    default fn createAndInitializePoolIfNecessary(
        &mut self,
        token0: AccountId,
        token1: AccountId,
        fee: u32,
        sqrt_price_x96: U160,
    ) -> Address {
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
            let transfer_value = ink_env::transferred_value::<DefaultEnvironment>();
            pool_address = FactoryRef::create_pool_builder(&factory_address,fee,token0,token1)
                .transferred_value(transfer_value/2)
                .fire().unwrap();
            PoolActionRef::initialize(&mut pool_address,sqrt_price_x96);
        }else{
            let sqrt_price_x96_existing = PoolActionRef::getSlot0(&pool_address).sqrtPriceX96;
            if sqrt_price_x96_existing.value.is_zero() {
                PoolActionRef::initialize(&mut pool_address,sqrt_price_x96);
            }
        }
        pool_address
    }
}