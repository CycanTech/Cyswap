#![allow(non_snake_case)]
pub use super::data::*;
use crate::impls::pool_initialize::PoolInitializeStorage;
use crate::traits::core::pool::*;
pub use crate::traits::periphery::position_manager::*;
use ink_storage::traits::{SpreadLayout, PackedLayout, SpreadAllocate};
use primitives::{Address, Int24, Uint128, Uint24, Uint256, Uint80, Uint96, U256};
use libs::{PoolKey, periphery::PoolAddress};
use scale::{Encode, Decode};
use ink_storage::traits::StorageLayout;
use crate::traits::periphery::LiquidityManagement::*;
use ink_env::DefaultEnvironment;
#[cfg(feature = "position_manager")]
pub use swap_project_derive::PositionStorage;

impl<T: PositionStorage+PoolInitializeStorage> PositionManager for T {
    default fn mint(
        &mut self,
        params: MintParams,
    ) -> (
        Uint256, //tokenId
        u128,    //liquidity
        Uint256, //amount0
        Uint256, //amount1
    ) {
        // IUniswapV3Pool pool;
        let pool: &PoolActionRef;
        // (liquidity, amount0, amount1, pool) = addLiquidity(
        //     AddLiquidityParams({
        //         token0: params.token0,
        //         token1: params.token1,
        //         fee: params.fee,
        //         recipient: address(this),
        //         tickLower: params.tickLower,
        //         tickUpper: params.tickUpper,
        //         amount0Desired: params.amount0Desired,
        //         amount1Desired: params.amount1Desired,
        //         amount0Min: params.amount0Min,
        //         amount1Min: params.amount1Min
        //     })
        // );
        let position_manager_address = ink_env::account_id::<DefaultEnvironment>();
        let addLiquidityParams = AddLiquidityParams{
            token0: params.token0,
            token1: params.token1,
            fee: params.fee,
            recipient: position_manager_address,
            tickLower: params.tickLower,
            tickUpper: params.tickUpper,
            amount0Desired: params.amount0Desired,
            amount1Desired: params.amount1Desired,
            amount0Min: params.amount0Min,
            amount1Min: params.amount1Min
        };
        LiquidityManagementTraitRef::addLiquidity(self,addLiquidityParams);
        // _mint(params.recipient, (tokenId = _nextId++));

        // bytes32 positionKey = PositionKey.compute(address(this), params.tickLower, params.tickUpper);
        // (, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128, , ) = pool.positions(positionKey);

        // // idempotent set
        // uint80 poolId =
        //     cachePoolKey(
        //         address(pool),
        //         PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee})
        //     );

        // _positions[tokenId] = Position({
        //     nonce: 0,
        //     operator: address(0),
        //     poolId: poolId,
        //     tickLower: params.tickLower,
        //     tickUpper: params.tickUpper,
        //     liquidity: liquidity,
        //     feeGrowthInside0LastX128: feeGrowthInside0LastX128,
        //     feeGrowthInside1LastX128: feeGrowthInside1LastX128,
        //     tokensOwed0: 0,
        //     tokensOwed1: 0
        // });

        // emit IncreaseLiquidity(tokenId, liquidity, amount0, amount1);
        (Uint256::new(), 0u128, Uint256::new(), Uint256::new())
    }
    
}

