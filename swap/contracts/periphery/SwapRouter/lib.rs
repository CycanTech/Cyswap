#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

#[brush::contract]
pub mod swapper_router {
    use crabswap::impls::periphery_immutable_state::{ImmutableStateStorage,ImmutableStateData};
    use crabswap::traits::periphery::periphery_immutable_state::*;
    use libs::core::TickMath;
    use primitives::{Uint24, Uint256,U160, Address, U256, Int256, ADDRESS0, Uint160, I256};
    use ink_storage::traits::{SpreadAllocate, SpreadLayout};
    use scale::{Decode, Encode};
    use ink_env::DefaultEnvironment;

    use crabswap::traits::periphery::swap_router::*;
    use crabswap::traits::core::factory::FactoryRef;
    use libs::periphery::path;
    use crabswap::traits::periphery::PeripheryPayments::*;
    use crabswap::traits::core::pool::PoolActionRef;

    #[derive(Default, Decode, Encode, Debug, SpreadAllocate, SpreadLayout)]
    struct SwapCallbackData {
        path:String,
        payer:Address,
    }

    /// @dev Used as the placeholder value for amountInCached, because the computed amount in for an exact output swap
    /// can never actually be this value
    const DEFAULT_AMOUNT_IN_CACHED: U256 = U256::MAX;

    #[ink(storage)]
    #[derive(SpreadAllocate, ImmutableStateStorage)]
    pub struct SwapRouterContract {
        #[ImmutableStateField]
        immutable_state: ImmutableStateData,

        /// @dev Transient storage variable used for returning the computed amount in for an exact output swap.
        amountInCached: Uint256,
    }

    impl PeripheryImmutableState for SwapRouterContract{}

    impl PeripheryPaymentsTrait for SwapRouterContract {}

    impl SwapRouter for SwapRouterContract {
        #[ink(message, payable)]
        fn exactInputSingle(
            &mut self,
            tokenIn: Address,
            tokenOut: Address,
            fee: Uint24,
            recipient: Address,
            deadline: U256,
            amountIn: U256,
            amountOutMinimum: U256,
            sqrtPriceLimitX96: U160,
        ) -> U256{
            U256::from(0)
        }
    }

    impl SwapRouterContract {
        #[ink(constructor, payable)]
        pub fn new(factory: AccountId, weth9: AccountId, _tokenDescriptor: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut SwapRouterContract| {
                instance.immutable_state.factory = factory;
                instance.immutable_state.WETH9 = weth9;
                instance.amountInCached = Uint256::new_with_u256(DEFAULT_AMOUNT_IN_CACHED);
            })
        }

        /// @dev Returns the pool for the given token pair and fee. The pool contract may or may not exist.
        fn getPool(&self,
            tokenA:Address,
            tokenB:Address,
            fee:Uint24
        ) -> Address {
                FactoryRef::get_pool(&self.immutable_state.factory, fee, tokenA, tokenB)
        }

        // this method should move to SwapRouter
        #[ink(message)]
        pub fn uniswapV3SwapCallback(&self,
            amount0Delta:Int256,
            amount1Delta:Int256,
            _data:Vec<u8>
        )  {
            // require(amount0Delta > 0 || amount1Delta > 0); // swaps entirely within 0-liquidity regions are not supported
            assert!(amount0Delta > 0 || amount1Delta > 0,"amount0Delta or amount1Delta must bt 0");
            // SwapCallbackData memory data = abi.decode(_data, (SwapCallbackData));
            let data: SwapCallbackData =
                Decode::decode(&mut _data.as_ref()).expect("call back data parse error!");
            // (address tokenIn, address tokenOut, uint24 fee) = data.path.decodeFirstPool();
            let ( tokenIn,  tokenOut, fee, ) = path::decodeFirstPool(data.path);
            let pool = if tokenIn < tokenOut{
                 self.getPool(tokenIn,tokenOut,u32::try_from(fee).expect("usize error!"))
            }else{
                self.getPool(tokenOut,tokenIn,u32::try_from(fee).expect("usize error!"))
            };
            // CallbackValidation.verifyCallback(factory, tokenIn, tokenOut, fee);
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            assert!(msg_sender==pool,"call back is not pool");
            // (bool isExactInput, uint256 amountToPay) =
            //     amount0Delta > 0
            //         ? (tokenIn < tokenOut, uint256(amount0Delta))
            //         : (tokenOut < tokenIn, uint256(amount1Delta));
            let (isExactInput,amountToPay) = 
            if amount0Delta > 0 {
                (tokenIn < tokenOut, U256::from(amount0Delta))
            }else{
                (tokenOut < tokenIn, U256::from(amount1Delta))
            };
            // if (isExactInput) {
            //     pay(tokenIn, data.payer, msg.sender, amountToPay);
            // } else {
            //     // either initiate the next swap or pay
            //     if (data.path.hasMultiplePools()) {
            //         data.path = data.path.skipToken();
            //         exactOutputInternal(amountToPay, msg.sender, 0, data);
            //     } else {
            //         amountInCached = amountToPay;
            //         tokenIn = tokenOut; // swap in/out because exact output swaps are reversed
            //         pay(tokenIn, data.payer, msg.sender, amountToPay);
            //     }
            // }
            if isExactInput{
                self.pay(tokenIn, data.payer,msg_sender, amountToPay);
            }else{
                // either initiate the next swap or pay
                if path::hasMultiplePools(data.path) {
                    data.path = path::skipToken(data.path);
                    exactOutputInternal(amountToPay, msg_sender, 0, data);
                } else {
                    self.amountInCached = Uint256::new_with_u256(amountToPay);
                    tokenIn = tokenOut; // swap in/out because exact output swaps are reversed
                    self.pay(tokenIn, data.payer, msg_sender, amountToPay);
                }
            }
        }

        /// @dev Performs a single exact output swap
        fn exactOutputInternal(
            &self,
            amountOut:U256,
            recipient:Address,
            sqrtPriceLimitX96:U160,
            data:SwapCallbackData
        )-> U256 {
            // allow swapping to the router address with address 0
            // if (recipient == address(0)) recipient = address(this);
            if recipient == ADDRESS0.into() {
                recipient = ink_env::account_id::<DefaultEnvironment>();
            }
            // (address tokenOut, address tokenIn, uint24 fee) = data.path.decodeFirstPool();
            let (tokenOut, tokenIn, fee) = path::decodeFirstPool(data.path);

            // bool zeroForOne = tokenIn < tokenOut;
            let zeroForOne:bool = tokenIn < tokenOut;
            // (int256 amount0Delta, int256 amount1Delta) =
            //     getPool(tokenIn, tokenOut, fee).swap(
            //         recipient,
            //         zeroForOne,
            //         -amountOut.toInt256(),
            //         sqrtPriceLimitX96 == 0
            //             ? (zeroForOne ? TickMath.MIN_SQRT_RATIO + 1 : TickMath.MAX_SQRT_RATIO - 1)
            //             : sqrtPriceLimitX96,
            //         abi.encode(data)
            //     );
            let fee = u32::try_from(fee).expect("usize to u32 error!");
            let factoryAddress = self.immutable_state.factory;
            let pool: Address =
                FactoryRef::get_pool(&factoryAddress, fee, tokenIn, tokenOut);
            let (amount0Delta, amount1Delta) =
            PoolActionRef::swap(&pool,
                    recipient,
                    zeroForOne,
                    -I256::try_from(amountOut.as_u128()).expect("exchange u128 to i256 error!"),
                    if sqrtPriceLimitX96.is_zero() {
                        if zeroForOne{
                            U160::from(TickMath::MIN_SQRT_RATIO) + 1
                        }else{
                            U160::from(TickMath::MAX_SQRT_RATIO) - 1
                        }
                    }else{
                        sqrtPriceLimitX96
                    },
                    //判断data是否已经被改变
                    scale::Encode::encode(&data)
                );
            

            // uint256 amountOutReceived;
            // (amountIn, amountOutReceived) = zeroForOne
            //     ? (uint256(amount0Delta), uint256(-amount1Delta))
            //     : (uint256(amount1Delta), uint256(-amount0Delta));
            // // it's technically possible to not receive the full output amount,
            // // so if no price limit has been specified, require this possibility away
            // if (sqrtPriceLimitX96 == 0) require(amountOutReceived == amountOut);
            let (amountIn, amountOutReceived) = if zeroForOne{
                (U256::from(amount0Delta), U256::from(-amount1Delta))
            }else{
                (U256::from(amount1Delta), U256::from(-amount0Delta))
            };
            // it's technically possible to not receive the full output amount,
            // so if no price limit has been specified, require this possibility away
            if sqrtPriceLimitX96.is_zero() {
                assert!(amountOutReceived == amountOut,"amountOutReceived must equal amountOut");
            } 
            
        }
    }

}
