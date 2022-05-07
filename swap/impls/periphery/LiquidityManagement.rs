#![allow(non_snake_case)]

use crate::traits::core::factory::*;
use ink_env::DefaultEnvironment;
use ink_prelude::vec::Vec;
use ink_storage::traits::{SpreadAllocate, SpreadLayout};
use libs::core::TickMath;
use libs::periphery::LiquidityAmounts;
use libs::PoolKey;
use primitives::{Address, U256};
use scale::{Decode, Encode};

use crate::impls::pool_initialize::PoolInitializeStorage;
pub use crate::traits::core::pool::*;
pub use crate::traits::periphery::LiquidityManagement::*;
use crate::traits::periphery::PeripheryPayments::*;

#[derive(Default, Decode, Encode, Debug, SpreadAllocate, SpreadLayout)]
struct MintCallbackData {
    poolKey: PoolKey,
    payer: Address,
}

impl<T: PoolInitializeStorage> LiquidityManagementTrait for T {
    // returns (uint128 liquidity,uint256 amount0,uint256 amount1,IUniswapV3Pool pool)
    default fn addLiquidity(&mut self, params: AddLiquidityParams) -> (u128, U256, U256, Address) {
        // PoolAddress.PoolKey memory poolKey =
        //         PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee});
        let poolKey: PoolKey = PoolKey {
            token0: params.token0,
            token1: params.token1,
            fee: params.fee,
        };

        // pool = IUniswapV3Pool(PoolAddress.computeAddress(factory, poolKey));
        let factory = self.get().factory;
        // let poolAddress = PoolAddress::computeAddress(factory, poolKey.clone());
        let poolAddress = FactoryRef::get_pool(&factory, params.fee, params.token0, params.token1);

        //         // compute the liquidity amount
        //         {
        //             // (uint160 sqrtPriceX96, , , , , , ) = poolRef::slot0(poolAddress);
        let slot0: Slot0 = PoolActionRef::getSlot0(&poolAddress);
        let sqrtPriceX96 = slot0.sqrtPriceX96.value;
        //             // uint160 sqrtRatioAX96 = TickMath.getSqrtRatioAtTick(params.tickLower);
        let sqrtRatioAX96 = TickMath::getSqrtRatioAtTick(params.tickLower);
        //             // uint160 sqrtRatioBX96 = TickMath.getSqrtRatioAtTick(params.tickUpper);
        let sqrtRatioBX96 = TickMath::getSqrtRatioAtTick(params.tickUpper);

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
            params.amount1Desired.value,
        );

        //         (amount0, amount1) = pool.mint(
        //             params.recipient,
        //             params.tickLower,
        //             params.tickUpper,
        //             liquidity,
        //             abi.encode(MintCallbackData({poolKey: poolKey, payer: msg.sender}))
        //         );
        let msg_sender = ink_env::caller::<DefaultEnvironment>();
        let mint_callback_data = MintCallbackData {
            poolKey: poolKey,
            payer: msg_sender,
        };

        let callback_data = scale::Encode::encode(&mint_callback_data);
        let (amount0, amount1) = PoolActionRef::mint(
            &poolAddress,
            params.recipient,
            params.tickLower,
            params.tickUpper,
            liquidity,
            callback_data.clone(),
        );
        // self.uniswapV3MintCallback(amount0, amount1, callback_data);
        ink_env::debug_message("&&&&&&&&&&8");
        //         require(amount0 >= params.amount0Min && amount1 >= params.amount1Min, 'Price slippage check');
        assert!(
            amount0 >= params.amount0Min.value && amount1 >= params.amount1Min.value,
            "Price slippage check"
        );
        ink_env::debug_message("&&&&&&&&&&9");
        return (liquidity, amount0, amount1, poolAddress);
    }

    default fn uniswapV3MintCallback(&mut self, amount0Owed: U256, amount1Owed: U256, data: Vec<u8>) {
        ink_env::debug_message("&&&&&&&&&&1");
        let manager_address: brush::traits::AccountId = ink_env::account_id::<DefaultEnvironment>();
        ink_env::debug_message("&&&&&&&&&&2");
        let msg_sender = ink_env::caller::<DefaultEnvironment>();
        ink_env::debug_message("&&&&&&&&&&3");
        // MintCallbackData memory deceoded = abi.decode(data, (MintCallbackData));
        let decoded: MintCallbackData =
            scale::Decode::decode(&mut data.as_ref()).expect("call back data parse error!");
            ink_env::debug_message("&&&&&&&&&&4");
        // TODO add callback validation
        // CallbackValidation.verifyCallback(factory, decoded.poolKey);

        // if (amount0Owed > 0) pay(decoded.poolKey.token0, decoded.payer, msg.sender, amount0Owed);
        if amount0Owed > U256::from(0) {
            ink_env::debug_message("&&&&&&&&&&4.1");
            PeripheryPaymentsTraitRef::pay(
                &manager_address,
                decoded.poolKey.token0,
                decoded.payer,
                msg_sender,
                amount0Owed,
            );
            ink_env::debug_message("&&&&&&&&&&4.2");
        }
        ink_env::debug_message("&&&&&&&&&&5");
        // if (amount1Owed > 0) pay(decoded.poolKey.token1, decoded.payer, msg.sender, amount1Owed);
        if amount1Owed > U256::from(0) {
            ink_env::debug_message("&&&&&&&&&&6");
            PeripheryPaymentsTraitRef::pay(
                &manager_address,
                decoded.poolKey.token1,
                decoded.payer,
                msg_sender,
                amount1Owed,
            );
            ink_env::debug_message("&&&&&&&&&&7");
        }
    }
}
