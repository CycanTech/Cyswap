use ink_storage::traits::{SpreadAllocate, SpreadLayout};
use primitives::{Address, Int24, Uint128, Uint256, Uint80, Uint96};
use brush::{
    declare_storage_trait,
};
pub use crate::traits::periphery::position_manager::*;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
// details about the uniswap position
#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct Position {
    // the nonce for permits
    nonce:Uint96,
    // the address that is approved for spending this token
    operator:Address,
    // the ID of the pool with which this token is connected
    poolId:Uint80,
    // the tick range of the position
    tickLower:Int24,
    tickUpper:Int24,
    // the liquidity of the position
    liquidity:Uint128,
    // the fee growth of the aggregate position as of the last action on the individual position
    feeGrowthInside0LastX128:Uint256,
    feeGrowthInside1LastX128:Uint256,
    // how many uncollected tokens are owed to the position, as of the last computation
    tokensOwed0:Uint128,
    tokensOwed1:Uint128,
}

declare_storage_trait!(PositionStorage, Position);

impl<T:PositionStorage>  PositionManager for T{
    default fn mint(
        &mut self,
        params: MintParams,
    ) -> (
        Uint256, //tokenId
        u128,    //liquidity
        Uint256, //amount0
        Uint256, //amount1
    ){
        // IUniswapV3Pool pool;
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
        (Uint256::new(),0u128,Uint256::new(),Uint256::new())
    }
}