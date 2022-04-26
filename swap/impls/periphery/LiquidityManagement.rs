#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]


use ink_env::DefaultEnvironment;
use ink_storage::traits::{SpreadAllocate, SpreadLayout};
use libs::core::tick_math;
use libs::periphery::LiquidityAmounts;
use libs::{PoolKey, periphery::PoolAddress};
use primitives::{U256, Address};
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use scale::{WrapperTypeDecode, Encode, Decode};

use crate::impls::pool_initialize::PoolInitializeStorage;
pub use crate::traits::core::pool::*;
pub use crate::traits::periphery::LiquidityManagement::*;
use crate::traits::periphery::PeripheryPayments::*;

#[derive(Default,Decode,Encode, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
struct MintCallbackData {
    poolKey:PoolAddress::PoolKey,
    payer:Address,
}

impl<T:PoolInitializeStorage> LiquidityManagementTrait for T{
// returns (uint128 liquidity,uint256 amount0,uint256 amount1,IUniswapV3Pool pool)
fn addLiquidity(&mut self, params:AddLiquidityParams)->(u128,  U256, U256,Address){
    // PoolAddress.PoolKey memory poolKey =
    //         PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee});
    let poolKey:PoolKey = PoolKey{
            token0:params.token0,
            token1:params.token1,
            fee:params.fee
        };

    // pool = IUniswapV3Pool(PoolAddress.computeAddress(factory, poolKey));
    let factory = self.get().factory;
    let poolAddress = PoolAddress::computeAddress(factory, poolKey.clone());

//         // compute the liquidity amount
//         {
//             // (uint160 sqrtPriceX96, , , , , , ) = poolRef::slot0(poolAddress);
            let slot0:Slot0 = PoolActionRef::getSlot0(&poolAddress);
            let sqrtPriceX96 = slot0.sqrtPriceX96.value;
//             // uint160 sqrtRatioAX96 = TickMath.getSqrtRatioAtTick(params.tickLower);
            let sqrtRatioAX96 = tick_math::getSqrtRatioAtTick(params.tickLower);
//             // uint160 sqrtRatioBX96 = TickMath.getSqrtRatioAtTick(params.tickUpper);
            let sqrtRatioBX96 = tick_math::getSqrtRatioAtTick(params.tickUpper);

//             liquidity = LiquidityAmounts::getLiquidityForAmounts(
//                 sqrtPriceX96,
//                 sqrtRatioAX96,
//                 sqrtRatioBX96,
//                 params.amount0Desired,
//                 params.amount1Desired
//             );
//         }
            let liquidity = LiquidityAmounts::getLiquidityForAmounts(
                    sqrtPriceX96,
                    sqrtRatioAX96,
                    sqrtRatioBX96,
                    params.amount0Desired.value,
                    params.amount1Desired.value
                );

//         (amount0, amount1) = pool.mint(
//             params.recipient,
//             params.tickLower,
//             params.tickUpper,
//             liquidity,
//             abi.encode(MintCallbackData({poolKey: poolKey, payer: msg.sender}))
//         );
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            let mint_callback_data = MintCallbackData{
                poolKey: poolKey, payer: msg_sender
            };
            
            let callback_data = scale::Encode::encode(&mint_callback_data);
            let (amount0, amount1)=PoolActionRef::mint(&poolAddress,params.recipient,
                            params.tickLower,
                            params.tickUpper,
                            liquidity,
                            callback_data);
//         require(amount0 >= params.amount0Min && amount1 >= params.amount1Min, 'Price slippage check');
        assert!(amount0 >= params.amount0Min.value && amount1 >= params.amount1Min.value, "Price slippage check");
        return (liquidity,amount0,amount1,poolAddress);
    }

    fn uniswapV3MintCallback(&mut self,
        amount0Owed:U256,
        amount1Owed:U256,
        mut data:Vec<u8>
    ){
        let manager_address:Address = ink_env::account_id::<DefaultEnvironment>();
        let msg_sender = ink_env::caller::<DefaultEnvironment>();
        // MintCallbackData memory decoded = abi.decode(data, (MintCallbackData));
        let decoded:MintCallbackData = scale::Decode::decode(&mut data.as_ref()).expect("call back data parse error!");
        // TODO add callback validation
        // CallbackValidation.verifyCallback(factory, decoded.poolKey);

        // if (amount0Owed > 0) pay(decoded.poolKey.token0, decoded.payer, msg.sender, amount0Owed);
        if amount0Owed > U256::from(0) {
            PeripheryPaymentsTraitRef::pay(&manager_address,decoded.poolKey.token0, decoded.payer, msg_sender, amount0Owed);
        }
        
        // if (amount1Owed > 0) pay(decoded.poolKey.token1, decoded.payer, msg.sender, amount1Owed);
        if amount1Owed > U256::from(0) {
            PeripheryPaymentsTraitRef::pay(&manager_address,decoded.poolKey.token1, decoded.payer, msg_sender, amount1Owed);
        }
    }
}