#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

#[brush::contract]
pub mod swapper_router {
    use crabswap::impls::periphery_immutable_state::{ImmutableStateData, ImmutableStateStorage};
    use crabswap::traits::periphery::periphery_immutable_state::*;
    use ink_env::{DefaultEnvironment};
    use ink_storage::traits::{SpreadAllocate, SpreadLayout};
    use libs::core::TickMath;
    use primitives::{Address, Int256, Uint24, Uint256, ADDRESS0, I256, U160, U256};
    use scale::{Decode, Encode};

    use crabswap::traits::core::factory::FactoryRef;
    use crabswap::traits::core::pool::PoolActionRef;
    use crabswap::traits::periphery::swap_callback::{swapcallback_external, SwapCallback};
    use crabswap::traits::periphery::swap_router::*;
    use crabswap::traits::periphery::PeripheryPayments::*;
    use libs::periphery::path;
    use brush::modifiers;
    use crabswap::traits::periphery::position_manager::checkDeadline;
    use ink_prelude::vec::Vec;

    #[derive(Default, Decode, Encode, Debug, SpreadAllocate, SpreadLayout)]
    struct SwapCallbackData {
        path: Vec<u8>,
        payer: Address,
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

    impl PeripheryImmutableState for SwapRouterContract {}

    impl PeripheryPaymentsTrait for SwapRouterContract {}

    impl SwapRouter for SwapRouterContract {

        // function exactInputSingle(ExactInputSingleParams calldata params)
        // external
        // payable
        // override
        // checkDeadline(params.deadline)
        // returns (uint256 amountOut)
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
        ) -> U256 {
            let params:ExactInputSingleParams=ExactInputSingleParams{
                 tokenIn,
                 tokenOut,
                 fee,
                 recipient,
                 deadline,
                 amountIn,
                 amountOutMinimum,
                 sqrtPriceLimitX96,
            };
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            let amountOut = self.exactInputInternal(
                params.amountIn,
                params.recipient,
                params.sqrtPriceLimitX96,
                SwapCallbackData{path: scale::Encode::encode(&(params.tokenIn, params.fee, params.tokenOut)), payer: msg_sender}
            );
            assert!(amountOut >= params.amountOutMinimum, "Too little received");
            U256::from(amountOut)
        }

        #[ink(message, payable)]
        #[modifiers(checkDeadline(deadline))]
        fn exactInput(&mut self,
            path: Vec<u8>,
            recipient: Address,
            deadline: u64,
            amountIn: U256,
            amountOutMinimum: U256,)
        ->U256
        {
            let mut amountOut = Default::default();
            let mut params:ExactInputParams=ExactInputParams{
                path,
                recipient,
                deadline,
                amountIn,
                amountOutMinimum
            };
            // address payer = msg.sender; // msg.sender pays for the first hop
            let mut payer:Address = ink_env::caller::<DefaultEnvironment>();
            // while (true) {
            loop {
                // params.amountIn = exactInputInternal(
                //     params.amountIn,
                //     hasMultiplePools ? address(this) : params.recipient, // for intermediate swaps, this contract custodies
                //     0,
                //     SwapCallbackData({
                //         path: params.path.getFirstPool(), // only the first pool in the path is necessary
                //         payer: payer
                //     })
                // );
                let hasMultiplePools:bool = path::hasMultiplePools(&params.path);
                // the outputs of prior swaps become the inputs to subsequent ones
                params.amountIn = self.exactInputInternal(
                    params.amountIn,
                    if hasMultiplePools {
                        ink_env::account_id::<DefaultEnvironment>()
                    }else{
                        params.recipient
                    } , // for intermediate swaps, this contract custodies
                    U256::zero(),
                    SwapCallbackData{
                        path: path::getFirstPool(&params.path), // only the first pool in the path is necessary
                        payer: payer
                    }
                );

            //     // decide whether to continue or terminate
            //     if (hasMultiplePools) {
            //         payer = address(this); // at this point, the caller has paid
            //         params.path = params.path.skipToken();
            //     } else {
            //         amountOut = params.amountIn;
            //         break;
            //     }
                if hasMultiplePools {
                    payer = ink_env::account_id::<DefaultEnvironment>(); // at this point, the caller has paid
                    params.path = path::skipToken(&params.path);
                } else {
                    amountOut = params.amountIn;
                    break;
                }
            }

            assert!(amountOut >= params.amountOutMinimum, "Too little received");
            amountOut
        }

        #[ink(message, payable)]
        #[modifiers(checkDeadline(deadline))]
        fn exactOutputSingle(
            &mut self,
            tokenIn: Address,
            tokenOut: Address,
            fee: Uint24,
            recipient: Address,
            deadline: u64,
            amountOut: U256,
            amountInMaximum: U256,
            sqrtPriceLimitX96: U160,
        ) -> U256
        {
            // avoid an SLOAD by using the swap return data
            // amountIn = exactOutputInternal(
            //     params.amountOut,
            //     params.recipient,
            //     params.sqrtPriceLimitX96,
            //     SwapCallbackData({path: abi.encodePacked(params.tokenOut, params.fee, params.tokenIn), payer: msg.sender})
            // );

            // require(amountIn <= params.amountInMaximum, 'Too much requested');
            // // has to be reset even though we don't use it in the single hop case
            // amountInCached = DEFAULT_AMOUNT_IN_CACHED;
            let params:ExactOutputSingleParams = ExactOutputSingleParams{
                 tokenIn,
                 tokenOut,
                 fee,
                 recipient,
                 deadline,
                 amountOut,
                 amountInMaximum,
                 sqrtPriceLimitX96,
            };
            let msg_sender = ink_env::account_id::<DefaultEnvironment>();
            let amountIn = self.exactOutputInternal(
                params.amountOut,
                params.recipient,
                params.sqrtPriceLimitX96,
                SwapCallbackData{path: Encode::encode(&(params.tokenOut, params.fee, params.tokenIn)), payer: msg_sender}
            );

            assert!(amountIn <= params.amountInMaximum, "Too much requested");
            // has to be reset even though we don't use it in the single hop case
            self.amountInCached = Uint256::new_with_u256(DEFAULT_AMOUNT_IN_CACHED);
            amountIn
        }

        // function exactOutput(ExactOutputParams calldata params)
        // external
        // payable
        // override
        // checkDeadline(params.deadline)
        // returns (uint256 amountIn)
        #[ink(message, payable)]
        fn exactOutput(
            &mut self,
            path: Vec<u8>,
            recipient: Address,
            deadline: u64,
            amountOut: U256,
            amountInMaximum: U256,
        ) -> U256
        {
        //     // it's okay that the payer is fixed to msg.sender here, as they're only paying for the "final" exact output
        //     // swap, which happens first, and subsequent swaps are paid for within nested callback frames
        //     exactOutputInternal(
        //         params.amountOut,
        //         params.recipient,
        //         0,
        //         SwapCallbackData({path: params.path, payer: msg.sender})
        //     );

        //     amountIn = amountInCached;
        //     require(amountIn <= params.amountInMaximum, 'Too much requested');
        //     amountInCached = DEFAULT_AMOUNT_IN_CACHED;
            let params:ExactOutputParams = ExactOutputParams{
                path,
                recipient,
                deadline,
                amountOut,
                amountInMaximum,
            };
            // it's okay that the payer is fixed to msg.sender here, as they're only paying for the "final" exact output
            // swap, which happens first, and subsequent swaps are paid for within nested callback frames
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            self.exactOutputInternal(
                params.amountOut,
                params.recipient,
                U256::zero(),
                SwapCallbackData{path: params.path, payer: msg_sender}
            );

            let amountIn = self.amountInCached.value;
            assert!(amountIn <= params.amountInMaximum, "Too much requested");
            self.amountInCached = Uint256::new_with_u256(DEFAULT_AMOUNT_IN_CACHED);
            amountIn
        }

    }
    

    impl SwapCallback for SwapRouterContract {
        // this method should move to SwapRouter
        #[ink(message)]
        fn swapCallback(&mut self, amount0Delta: Int256, amount1Delta: Int256, _data: Vec<u8>) {
            // require(amount0Delta > 0 || amount1Delta > 0); // swaps entirely within 0-liquidity regions are not supported
            assert!(
                amount0Delta > 0 || amount1Delta > 0,
                "amount0Delta or amount1Delta must bt 0"
            );
            // SwapCallbackData memory data = abi.decode(_data, (SwapCallbackData));
            let mut data: SwapCallbackData =
                Decode::decode(&mut _data.as_ref()).expect("call back data parse error!");
            // (address tokenIn, address tokenOut, uint24 fee) = data.path.decodeFirstPool();
            let (mut tokenIn, fee,tokenOut ) = path::decodeFirstPool(&data.path);
            let pool = if tokenIn < tokenOut {
                self.getPool(tokenIn, tokenOut, u32::try_from(fee).expect("usize error!"))
            } else {
                self.getPool(tokenOut, tokenIn, u32::try_from(fee).expect("usize error!"))
            };
            // CallbackValidation.verifyCallback(factory, tokenIn, tokenOut, fee);
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            assert!(msg_sender == pool, "call back is not pool");
            // (bool isExactInput, uint256 amountToPay) =
            //     amount0Delta > 0
            //         ? (tokenIn < tokenOut, uint256(amount0Delta))
            //         : (tokenOut < tokenIn, uint256(amount1Delta));
            let (isExactInput, amountToPay) = if amount0Delta > 0 {
                (tokenIn < tokenOut, U256::from(amount0Delta))
            } else {
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
            if isExactInput {
                self.pay(tokenIn, data.payer, msg_sender, amountToPay);
            } else {
                // either initiate the next swap or pay
                if path::hasMultiplePools(&data.path) {
                    data.path = path::skipToken(&data.path);
                    self.exactOutputInternal(amountToPay, msg_sender, U160::zero(), data);
                } else {
                    self.amountInCached = Uint256::new_with_u256(amountToPay);
                    tokenIn = tokenOut; // swap in/out because exact output swaps are reversed
                    self.pay(tokenIn, data.payer, msg_sender, amountToPay);
                }
            }
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
        fn getPool(&self, tokenA: Address, tokenB: Address, fee: Uint24) -> Address {
            FactoryRef::get_pool(&self.immutable_state.factory, fee, tokenA, tokenB)
        }

        /// @dev Performs a single exact output swap
        fn exactOutputInternal(
            &mut self,
            amountOut: U256,
            mut recipient: Address,
            sqrtPriceLimitX96: U160,
            data: SwapCallbackData,
        ) -> U256 {
            // allow swapping to the router address with address 0
            // if (recipient == address(0)) recipient = address(this);
            if recipient == ADDRESS0.into() {
                recipient = ink_env::account_id::<DefaultEnvironment>();
            }
            // (address tokenOut, address tokenIn, uint24 fee) = data.path.decodeFirstPool();
            let (tokenOut,fee, tokenIn ) = path::decodeFirstPool(&data.path);

            // bool zeroForOne = tokenIn < tokenOut;
            let zeroForOne: bool = tokenIn < tokenOut;
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
            let pool: Address = FactoryRef::get_pool(&factoryAddress, fee, tokenIn, tokenOut);
            let (amount0Delta, amount1Delta) = PoolActionRef::swap(
                &pool,
                recipient,
                zeroForOne,
                -I256::try_from(amountOut.as_u128()).expect("exchange u128 to i256 error!"),
                if sqrtPriceLimitX96.is_zero() {
                    if zeroForOne {
                        U160::from(TickMath::MIN_SQRT_RATIO) + 1
                    } else {
                        U160::from(TickMath::MAX_SQRT_RATIO) - 1
                    }
                } else {
                    sqrtPriceLimitX96
                },
                //判断data是否已经被改变
                scale::Encode::encode(&data),
            );

            // uint256 amountOutReceived;
            // (amountIn, amountOutReceived) = zeroForOne
            //     ? (uint256(amount0Delta), uint256(-amount1Delta))
            //     : (uint256(amount1Delta), uint256(-amount0Delta));
            // // it's technically possible to not receive the full output amount,
            // // so if no price limit has been specified, require this possibility away
            // if (sqrtPriceLimitX96 == 0) require(amountOutReceived == amountOut);
            let (amountIn, amountOutReceived) = if zeroForOne {
                (U256::from(amount0Delta), U256::from(-amount1Delta))
            } else {
                (U256::from(amount1Delta), U256::from(-amount0Delta))
            };
            // it's technically possible to not receive the full output amount,
            // so if no price limit has been specified, require this possibility away
            if sqrtPriceLimitX96.is_zero() {
                assert!(
                    amountOutReceived == amountOut,
                    "amountOutReceived must equal amountOut"
                );
            }
            self.amountInCached = Uint256::new_with_u256(DEFAULT_AMOUNT_IN_CACHED);
            amountIn
        }


        /// @dev Performs a single exact input swap
        fn exactInputInternal(
            &self,
            amountIn: U256,
            mut recipient: Address,
            sqrtPriceLimitX96: U160,
            data: SwapCallbackData,
        ) -> U256 {
            // allow swapping to the router address with address 0
            if recipient == ADDRESS0.into() {
                recipient = ink_env::account_id::<DefaultEnvironment>();
            }

            let (tokenIn,fee, tokenOut, ): (Address,u32, Address) =
                path::decodeFirstPool(&data.path);

            let zeroForOne: bool = tokenIn < tokenOut;

            // let (amount0, amount1):(Int256,Int256) =
            //     getPool(tokenIn, tokenOut, fee).swap(
            //         recipient,
            //         zeroForOne,
            //         amountIn.toInt256(),
            //         sqrtPriceLimitX96 == 0
            //             ? (zeroForOne ? TickMath.MIN_SQRT_RATIO + 1 : TickMath.MAX_SQRT_RATIO - 1)
            //             : sqrtPriceLimitX96,
            //         abi.encode(data)
            //     );
            let pool = self.getPool(
                tokenIn,
                tokenOut,
                u32::try_from(fee).expect("usize to u32 error!"),
            );
            let (amount0, amount1): (Int256, Int256) = PoolActionRef::swap(
                &pool,
                recipient,
                zeroForOne,
                Int256::try_from(amountIn.as_u128()).expect("u256 to I256 error"),
                if sqrtPriceLimitX96.is_zero() {
                    if zeroForOne {
                        U256::from(TickMath::MIN_SQRT_RATIO) + 1
                    } else {
                        U256::from(TickMath::MAX_SQRT_RATIO) - 1
                    }
                } else {
                    sqrtPriceLimitX96
                },
                scale::Encode::encode(&data),
            );

            return U256::from(-(if zeroForOne { amount1 } else { amount0 }));
        }
    }
}
