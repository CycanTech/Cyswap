//由于部署不同,该mod废弃不用.
use openbrush::declare_storage_trait;
// use openbrush::traits::Flush;
use ink_env::{hash::{HashOutput, Sha2x256}};
use primitives::{Address, Uint24, Int24};
use crate::traits::core::pool_deployer::*;
use crate::traits::core::pool::*;
pub use crate::traits::periphery::weth9::*;

declare_storage_trait!(PoolDeployerStorage, Parameters);

impl<T: PoolDeployerStorage> PoolDeployer for T {

    fn parameters(
        &self,
    ) -> Parameters{
        let parameters = self.get();
        parameters.clone()
    }

    fn deploy(
        &mut self,
        factory:Address,
        token0:Address,
        token1:Address,
        fee:Uint24,
        tick_spacing:Int24,
    ) -> Address{
        // parameters = Parameters({factory: factory, token0: token0, token1: token1, fee: fee, tickSpacing: tickSpacing});
        // pool = address(new UniswapV3Pool{salt: keccak256(abi.encode(token0, token1, fee))}());
        // delete parameters;

        PoolDeployerStorage::get_mut(self).factory = factory;
        PoolDeployerStorage::get_mut(self).token0 = token0;
        PoolDeployerStorage::get_mut(self).token1 = token1;
        PoolDeployerStorage::get_mut(self).fee = fee;
        PoolDeployerStorage::get_mut(self).tick_spacing = tick_spacing;

        let total_balance = Self::env().balance();
        let encodable = (factory, token0, token1,fee); // Implements `scale::Encode`
        let mut salt = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
        ink_env::hash_encoded::<Sha2x256, _>(&encodable, &mut salt);
        let pool_address = PoolRef::new(factory,token0, token1, fee, tick_spacing)
                .endowment(total_balance / 4)
                .code_hash(ink_env::Hash::try_from(hex::decode(ACCUMULATOR_CODE_HASH).unwrap().as_ref()).unwrap())
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "failed at instantiating the Accumulator contract: {:?}",
                        error
                    )
                });
        pool_address
    }
}
