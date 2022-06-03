#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

#[brush::contract]
pub mod crab_swap_pool {
    use crabswap::traits::core::pool_action::*;
    use ink_env::DefaultEnvironment;
    use ink_lang::codegen::Env;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use ink_storage::{
        traits::{PackedLayout, SpreadLayout, StorageLayout},
        Mapping,
    };
    use libs::{
        core::{
            oracle::Observations, LiquidityMath, Position, SqrtPriceMath, Tick, TickBitmap,
            TickMath,
        },
        getTickAtSqrtRatio,
    };
    use primitives::{Address, Int24, Int256, Uint160, Uint256, I256, U160, U256};
    use scale::{Decode, Encode};
    type Uint24 = u32;
    use brush::contracts::psp22::extensions::metadata::*;
    use brush::{modifiers, modifier_definition};
    use crabswap::impls::core::no_delegate_call::{NoDelegateCallData, NoDelegateCallStorage};
    use crabswap::traits::core::no_delegate_call::{noDelegateCall, NoDelegateCall};
    use crabswap::traits::periphery::swap_callback::SwapCallbackRef;
    use crabswap::traits::periphery::LiquidityManagement::*;
    use ink_env::CallFlags;
    use ink_lang::codegen::EmitEvent;
    use ink_prelude::vec;
    use libs::core::FixedPoint128;
    use libs::core::SwapMath;
    use libs::swap::FullMath;
    use crabswap::traits::core::pool_owner_action::PoolOwnerActions;
    use crabswap::traits::core::pool_owner_action::poolowneractions_external;
    use brush::contracts::traits::ownable::OwnableRef;

    // accumulated protocol fees in token0/token1 units
    #[derive(
        Debug, Default, PartialEq, Eq, Encode, Decode, SpreadLayout, SpreadAllocate, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct ProtocolFees {
        token0: u128,
        token1: u128,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate, NoDelegateCallStorage)]
    pub struct PoolContract {
        #[NoDelegateCallField]
        no_delegate_call: NoDelegateCallData,
        // address public immutable override factory;
        // address public immutable override token0;
        // address public immutable override token1;
        // uint24 public immutable override fee;
        pub maxLiquidityPerTick: u128,

        // follow six parameter is immutable
        pub factory: Address,
        pub token0: Address,
        pub token1: Address,
        pub fee: Uint24,
        pub tickSpacing: Int24,
        pub max_liquidity_per_tick: u128,
        pub slot0: Slot0,

        pub fee_growth_global0_x128: Uint160,
        pub fee_growth_global1_x128: Uint160,

        /// @inheritdoc IUniswapV3PoolState
        pub feeGrowthGlobal0X128: Uint256,
        /// @inheritdoc IUniswapV3PoolState
        pub feeGrowthGlobal1X128: Uint256,

        pub protocolFees: ProtocolFees,
        pub liquidity: u128,
        // mapping(int24 => Tick.Info) pub ticks;
        pub ticks: Mapping<Int24, Tick::Info>,
        // /// @inheritdoc IUniswapV3PoolState
        // mapping(int16 => uint256) public override tickBitmap;
        pub tickBitmap: Mapping<i16, Uint256>,
        // /// @inheritdoc IUniswapV3PoolState
        // mapping(bytes32 => Position.Info) public override positions;
        pub positions: Mapping<(Address, Int24, Int24), Position::Info>,
        /// @inheritdoc IUniswapV3PoolState
        pub observations: Observations,
    }

    impl NoDelegateCall for PoolContract {}

    impl PSP22Receiver for PoolContract {
        #[ink(message)]
        fn before_received(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _value: Balance,
            _data: Vec<u8>,
        ) -> Result<(), PSP22ReceiverError> {
            // if self.revert_next_transfer {
            //     self.revert_next_transfer = false;
            //     return Err(PSP22ReceiverError::TransferRejected(String::from(
            //         "I should reject next transfer",
            //     )))
            // }
            // self.call_counter += 1;
            Ok(())
        }
    }

    struct SwapCache {
        // the protocol fee for the input token
        pub feeProtocol: u8,
        // liquidity at the beginning of the swap
        pub liquidityStart: u128,
        // the timestamp of the current block
        pub blockTimestamp: u64,
        // the current value of the tick accumulator, computed only if we cross an initialized tick
        pub tickCumulative: i64,
        // the current value of seconds per liquidity accumulator, computed only if we cross an initialized tick
        pub secondsPerLiquidityCumulativeX128: U160,
        // whether we've computed and cached the above two accumulators
        pub computedLatestObservation: bool,
    }

    // the top level state of the swap, the results of which are recorded in storage at the end
    struct SwapState {
        // the amount remaining to be swapped in/out of the input/output asset
        pub amountSpecifiedRemaining: Int256,
        // the amount already swapped out/in of the output/input asset
        pub amountCalculated: Int256,
        // current sqrt(price)
        pub sqrtPriceX96: U160,
        // the tick associated with the current price
        pub tick: Int24,
        // the global fee growth of the input token
        pub feeGrowthGlobalX128: U256,
        // amount of input token paid as protocol fee
        pub protocolFee: u128,
        // the current liquidity in range
        pub liquidity: u128,
    }

    #[derive(Default)]
    struct StepComputations {
        // the price at the beginning of the step
        pub sqrtPriceStartX96: U160,
        // the next tick to swap to from the current tick in the swap direction
        pub tickNext: Int24,
        // whether tickNext is initialized or not
        pub initialized: bool,
        // sqrt(price) for the next tick (1/0)
        pub sqrtPriceNextX96: U160,
        // how much is being swapped in in this step
        pub amountIn: U256,
        // how much is being swapped out
        pub amountOut: U256,
        // how much fee is being paid in
        pub feeAmount: U256,
    }

    #[modifier_definition]
    pub fn onlyFactoryOwner<T, F, R>(instance: &mut T, body: F) -> R
    where
        T: PoolOwnerActions,
        F: FnOnce(&mut T) -> R,
    {
        // require(msg.sender == IUniswapV3Factory(factory).owner());
        // _;
        let msg_sender = ink_env::caller::<DefaultEnvironment>();
        let factory_owner:Address = instance.get_factory();
        let owner = OwnableRef::owner(&factory_owner);
        assert!(msg_sender == owner);
        body(instance)
    }

    impl PoolOwnerActions for PoolContract {
        // function setFeeProtocol(uint8 feeProtocol0, uint8 feeProtocol1) external override lock onlyFactoryOwner {
        #[ink(message)]
        #[modifiers(lock)]
        #[modifiers(onlyFactoryOwner)]
        fn setFeeProtocol(&mut self, feeProtocol0: u8, feeProtocol1: u8) {
            // require(
            //     (feeProtocol0 == 0 || (feeProtocol0 >= 4 && feeProtocol0 <= 10)) &&
            //         (feeProtocol1 == 0 || (feeProtocol1 >= 4 && feeProtocol1 <= 10))
            // );
            assert!(
                (feeProtocol0 == 0 || (feeProtocol0 >= 4 && feeProtocol0 <= 10))
                    && (feeProtocol1 == 0 || (feeProtocol1 >= 4 && feeProtocol1 <= 10))
            );

            // uint8 feeProtocolOld = slot0.feeProtocol;
            let feeProtocolOld: u8 = self.slot0.feeProtocol;
            // slot0.feeProtocol = feeProtocol0 + (feeProtocol1 << 4);
            self.slot0.feeProtocol = feeProtocol0 + (feeProtocol1 << 4);
            // emit SetFeeProtocol(feeProtocolOld % 16, feeProtocolOld >> 4, feeProtocol0, feeProtocol1);
            self.env().emit_event(SetFeeProtocol{
                feeProtocol0Old:feeProtocolOld % 16, feeProtocol1Old:feeProtocolOld >> 4, feeProtocol0New:feeProtocol0, feeProtocol1New:feeProtocol1
            });
        }

        #[ink(message)]
        #[modifiers(lock)]
        #[modifiers(onlyFactoryOwner)]
        fn collectProtocol(
            &mut self,
            recipient:Address,
            amount0Requested:u128,
            amount1Requested:u128
        )->(u128 , u128){
            // amount0 = amount0Requested > protocolFees.token0 ? protocolFees.token0 : amount0Requested;
            // amount1 = amount1Requested > protocolFees.token1 ? protocolFees.token1 : amount1Requested;
            let mut amount0 = if amount0Requested > self.protocolFees.token0{
                self.protocolFees.token0
            }else{
                amount0Requested
            };
            let mut amount1 =if amount1Requested > self.protocolFees.token1{
                self.protocolFees.token1
            }else{
                amount1Requested
            };
            
            // if (amount0 > 0) {
            //     if (amount0 == protocolFees.token0) amount0--; // ensure that the slot is not cleared, for gas savings
            //     protocolFees.token0 -= amount0;
            //     TransferHelper.safeTransfer(token0, recipient, amount0);
            // }
            if amount0 > 0 {
                if amount0 == self.protocolFees.token0 { amount0 = amount0-1;} // ensure that the slot is not cleared, for gas savings
                self.protocolFees.token0 -= amount0;
                PSP22Ref::transfer(&mut self.token0, recipient, amount0, vec![0u8])
                    .expect("token0 transfer error!");
            }
            // if (amount1 > 0) {
            //     if (amount1 == protocolFees.token1) amount1--; // ensure that the slot is not cleared, for gas savings
            //     protocolFees.token1 -= amount1;
            //     TransferHelper.safeTransfer(token1, recipient, amount1);
            // }
            if amount1 > 0 {
                if amount1 == self.protocolFees.token1 { amount1 = amount1-1;} // ensure that the slot is not cleared, for gas savings
                self.protocolFees.token1 -= amount1;
                PSP22Ref::transfer(&mut self.token1, recipient, amount1, vec![0u8])
                    .expect("token1 transfer error!");
            }
            // emit CollectProtocol(msg.sender, recipient, amount0, amount1);
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            self.env().emit_event(CollectProtocol{
                sender:msg_sender, 
                recipient, 
                amount0, 
                amount1
            });
            (amount0,amount1)

        }

        #[ink(message)]
        fn get_factory(&self)->Address{
            self.factory
        }
    }

    impl PoolAction for PoolContract {
        #[ink(message)]
        fn get_tickspacing(&self)->Int24{
            self.tickSpacing
        }

        #[ink(message)]
        #[modifiers(lock)]
        #[modifiers(noDelegateCall)]
        fn flash(&mut self, recipient: Address, amount0: U256, amount1: U256, data: Vec<u8>) {
            // uint128 _liquidity = liquidity;
            // require(_liquidity > 0, 'L');
            let _liquidity: u128 = self.liquidity;
            assert!(_liquidity > 0, "L");

            // uint256 fee0 = FullMath.mulDivRoundingUp(amount0, fee, 1e6);
            // uint256 fee1 = FullMath.mulDivRoundingUp(amount1, fee, 1e6);
            // uint256 balance0Before = balance0();
            // uint256 balance1Before = balance1();
            let fee0: U256 =
                FullMath::mulDivRoundingUp(amount0, U256::from(self.fee), U256::from(1e6 as u64));
            let fee1: U256 =
                FullMath::mulDivRoundingUp(amount1, U256::from(self.fee), U256::from(1e6 as u64));
            let balance0Before: U256 = self.balance0();
            let balance1Before: U256 = self.balance1();

            // if (amount0 > 0) TransferHelper.safeTransfer(token0, recipient, amount0);
            // if (amount1 > 0) TransferHelper.safeTransfer(token1, recipient, amount1);
            if amount0 > U256::zero() {
                PSP22Ref::transfer(&mut self.token0, recipient, amount0.as_u128(), vec![0u8])
                    .expect("token1 transfer error!");
            }
            if amount1 > U256::zero() {
                PSP22Ref::transfer(&mut self.token1, recipient, amount1.as_u128(), vec![0u8])
                    .expect("token1 transfer error!");
            }

            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            // TODO flash call back need finished.
            // IUniswapV3FlashCallback(msg_sender).uniswapV3FlashCallback(fee0, fee1, data);

            let balance0After: U256 = self.balance0();
            let balance1After: U256 = self.balance1();

            // require(balance0Before.add(fee0) <= balance0After, 'F0');
            // require(balance1Before.add(fee1) <= balance1After, 'F1');
            assert!(balance0Before + fee0 <= balance0After, "F0");
            assert!(balance1Before + fee1 <= balance1After, "F1");

            // sub is safe because we know balanceAfter is gt balanceBefore by at least fee
            // uint256 paid0 = balance0After - balance0Before;
            // uint256 paid1 = balance1After - balance1Before;
            let paid0: U256 = balance0After - balance0Before;
            let paid1: U256 = balance1After - balance1Before;

            // if (paid0 > 0) {
            //     uint8 feeProtocol0 = slot0.feeProtocol % 16;
            //     uint256 fees0 = feeProtocol0 == 0 ? 0 : paid0 / feeProtocol0;
            //     if (uint128(fees0) > 0) protocolFees.token0 += uint128(fees0);
            //     feeGrowthGlobal0X128 += FullMath.mulDiv(paid0 - fees0, FixedPoint128.Q128, _liquidity);
            // }
            // if (paid1 > 0) {
            //     uint8 feeProtocol1 = slot0.feeProtocol >> 4;
            //     uint256 fees1 = feeProtocol1 == 0 ? 0 : paid1 / feeProtocol1;
            //     if (uint128(fees1) > 0) protocolFees.token1 += uint128(fees1);
            //     feeGrowthGlobal1X128 += FullMath.mulDiv(paid1 - fees1, FixedPoint128.Q128, _liquidity);
            // }
            if paid0 > U256::zero() {
                let feeProtocol0: u8 = self.slot0.feeProtocol % 16;
                let fees0: U256 = if feeProtocol0 == 0 {
                    U256::zero()
                } else {
                    paid0 / feeProtocol0
                };
                if fees0 > U256::zero() {
                    self.protocolFees.token0 += fees0.as_u128();
                }
                // TODO check the self.feeGrowthGlobal0X128 changed
                self.feeGrowthGlobal0X128.value += FullMath::mulDiv(
                    paid0 - fees0,
                    U256::from(FixedPoint128::Q128),
                    U256::from(_liquidity),
                );
            }
            if paid1 > U256::zero() {
                let feeProtocol1: u8 = self.slot0.feeProtocol >> 4;
                let fees1: U256 = if feeProtocol1 == 0 {
                    U256::zero()
                } else {
                    paid1 / feeProtocol1
                };
                if fees1 > U256::zero() {
                    self.protocolFees.token1 += fees1.as_u128();
                }
                self.feeGrowthGlobal1X128.value += FullMath::mulDiv(
                    paid1 - fees1,
                    U256::from(FixedPoint128::Q128),
                    U256::from(_liquidity),
                );
            }

            // emit Flash(msg.sender, recipient, amount0, amount1, paid0, paid1);
            self.env().emit_event(Flash {
                sender: msg_sender,
                recipient,
                amount0,
                amount1,
                paid0,
                paid1,
            });
            todo!();
        }

        /// @inheritdoc IUniswapV3PoolActions
        // function swap(
        //     address recipient,
        //     bool zeroForOne,
        //     int256 amountSpecified,
        //     uint160 sqrtPriceLimitX96,
        //     bytes calldata data
        // ) external override noDelegateCall returns (int256 amount0, int256 amount1) {
        #[ink(message)]
        #[modifiers(noDelegateCall)]
        fn swap(
            &mut self,
            recipient: Address,
            zeroForOne: bool,
            amountSpecified: Int256,
            sqrtPriceLimitX96: U160,
            data: Vec<u8>,
        ) -> (Int256, Int256) {
            let mut amount0: Int256 = Default::default();
            let mut amount1: Int256 = Default::default();
            //     require(amountSpecified != 0, 'AS');
            assert!(amountSpecified != 0, "AS");

            let slot0Start: Slot0 = self.slot0.clone();

            //     require(slot0Start.unlocked, 'LOK');
            assert!(slot0Start.unlocked, "LOK");
            //     require(
            //         zeroForOne
            //             ? sqrtPriceLimitX96 < slot0Start.sqrtPriceX96 && sqrtPriceLimitX96 > TickMath.MIN_SQRT_RATIO
            //             : sqrtPriceLimitX96 > slot0Start.sqrtPriceX96 && sqrtPriceLimitX96 < TickMath.MAX_SQRT_RATIO,
            //         'SPL'
            //     );
            assert!(
                if zeroForOne {
                    sqrtPriceLimitX96 < slot0Start.sqrtPriceX96.value
                        && sqrtPriceLimitX96 > U256::from(TickMath::MIN_SQRT_RATIO)
                } else {
                    sqrtPriceLimitX96 > slot0Start.sqrtPriceX96.value
                        && sqrtPriceLimitX96 < U256::from(TickMath::MAX_SQRT_RATIO)
                },
                "SPL"
            );

            //     slot0.unlocked = false;
            self.slot0.unlocked = false;
            //     SwapCache memory cache =
            //         SwapCache({
            //             liquidityStart: liquidity,
            //             blockTimestamp: _blockTimestamp(),
            //             feeProtocol: zeroForOne ? (slot0Start.feeProtocol % 16) : (slot0Start.feeProtocol >> 4),
            //             secondsPerLiquidityCumulativeX128: 0,
            //             tickCumulative: 0,
            //             computedLatestObservation: false
            //         });
            let mut cache: SwapCache = SwapCache {
                liquidityStart: self.liquidity,
                blockTimestamp: ink_env::block_timestamp::<DefaultEnvironment>(),
                feeProtocol: if zeroForOne {
                    slot0Start.feeProtocol % 16
                } else {
                    slot0Start.feeProtocol >> 4
                },
                secondsPerLiquidityCumulativeX128: U256::zero(),
                tickCumulative: 0,
                computedLatestObservation: false,
            };

            //     bool exactInput = amountSpecified > 0;
            let exactInput: bool = amountSpecified > 0;

            //     SwapState memory state =
            //         SwapState({
            //             amountSpecifiedRemaining: amountSpecified,
            //             amountCalculated: 0,
            //             sqrtPriceX96: slot0Start.sqrtPriceX96,
            //             tick: slot0Start.tick,
            //             feeGrowthGlobalX128: zeroForOne ? feeGrowthGlobal0X128 : feeGrowthGlobal1X128,
            //             protocolFee: 0,
            //             liquidity: cache.liquidityStart
            //         });
            let mut state: SwapState = SwapState {
                amountSpecifiedRemaining: amountSpecified,
                amountCalculated: 0,
                sqrtPriceX96: slot0Start.sqrtPriceX96.value,
                tick: slot0Start.tick,
                feeGrowthGlobalX128: if zeroForOne {
                    self.feeGrowthGlobal0X128.value
                } else {
                    self.feeGrowthGlobal1X128.value
                },
                protocolFee: 0,
                liquidity: cache.liquidityStart,
            };

            //     // continue swapping as long as we haven't used the entire input/output and haven't reached the price limit
            //     while (state.amountSpecifiedRemaining != 0 && state.sqrtPriceX96 != sqrtPriceLimitX96) {
            ink_env::debug_println!("state.amountSpecifiedRemaining,state.sqrtPriceX96,sqrtPriceLimitX96 is:{:?},{:?},{:?}",state.amountSpecifiedRemaining,state.sqrtPriceX96,sqrtPriceLimitX96);
            while state.amountSpecifiedRemaining != 0 && state.sqrtPriceX96 != sqrtPriceLimitX96 {
                //StepComputations memory step;
                let mut step: StepComputations = Default::default();

                //         step.sqrtPriceStartX96 = state.sqrtPriceX96;
                step.sqrtPriceStartX96 = state.sqrtPriceX96;

                //         (step.tickNext, step.initialized) = tickBitmap.nextInitializedTickWithinOneWord(
                //             state.tick,
                //             tickSpacing,
                //             zeroForOne
                //         );
                (step.tickNext, step.initialized) = TickBitmap::nextInitializedTickWithinOneWord(
                    &mut self.tickBitmap,
                    state.tick,
                    self.tickSpacing,
                    zeroForOne,
                );
                ink_env::debug_println!(
                    "step.tickNext,step.initialized is:{:?},{:?}",
                    step.tickNext,
                    step.initialized
                );
                //         // ensure that we do not overshoot the min/max tick, as the tick bitmap is not aware of these bounds
                //         if (step.tickNext < TickMath.MIN_TICK) {
                //             step.tickNext = TickMath.MIN_TICK;
                //         } else if (step.tickNext > TickMath.MAX_TICK) {
                //             step.tickNext = TickMath.MAX_TICK;
                //         }
                if step.tickNext < TickMath::MIN_TICK {
                    step.tickNext = TickMath::MIN_TICK;
                } else if step.tickNext > TickMath::MAX_TICK {
                    step.tickNext = TickMath::MAX_TICK;
                }

                //         // get the price for the next tick
                //         step.sqrtPriceNextX96 = TickMath.getSqrtRatioAtTick(step.tickNext);
                step.sqrtPriceNextX96 = TickMath::getSqrtRatioAtTick(step.tickNext);

                //         // compute values to swap to the target tick, price limit, or point where input/output amount is exhausted
                //         (state.sqrtPriceX96, step.amountIn, step.amountOut, step.feeAmount) = SwapMath.computeSwapStep(
                //             state.sqrtPriceX96,
                //             (zeroForOne ? step.sqrtPriceNextX96 < sqrtPriceLimitX96 : step.sqrtPriceNextX96 > sqrtPriceLimitX96)
                //                 ? sqrtPriceLimitX96
                //                 : step.sqrtPriceNextX96,
                //             state.liquidity,
                //             state.amountSpecifiedRemaining,
                //             fee
                //         );
                (
                    state.sqrtPriceX96,
                    step.amountIn,
                    step.amountOut,
                    step.feeAmount,
                ) = SwapMath::computeSwapStep(
                    state.sqrtPriceX96,
                    if if zeroForOne {
                        step.sqrtPriceNextX96 < sqrtPriceLimitX96
                    } else {
                        step.sqrtPriceNextX96 > sqrtPriceLimitX96
                    } {
                        sqrtPriceLimitX96
                    } else {
                        step.sqrtPriceNextX96
                    },
                    state.liquidity,
                    state.amountSpecifiedRemaining,
                    self.fee,
                );
                ink_env::debug_println!("state.sqrtPriceX96,step.amountIn,step.amountOut,step.feeAmount:{:?},{:?},{:?},{:?}",state.sqrtPriceX96,step.amountIn,step.amountOut,step.feeAmount);
                //         if (exactInput) {
                //             state.amountSpecifiedRemaining -= (step.amountIn + step.feeAmount).toInt256();
                //             state.amountCalculated = state.amountCalculated.sub(step.amountOut.toInt256());
                //         } else {
                //             state.amountSpecifiedRemaining += step.amountOut.toInt256();
                //             state.amountCalculated = state.amountCalculated.add((step.amountIn + step.feeAmount).toInt256());
                //         }
                if exactInput {
                    // TODO check i128 maybe over MAX of i128
                    state.amountSpecifiedRemaining -=
                        I256::try_from((step.amountIn + step.feeAmount).as_u128())
                            .expect("U256 to I256 error!");
                    state.amountCalculated = state.amountCalculated
                        - I256::try_from(step.amountOut.as_u128()).expect("U256 to I256 error!");
                } else {
                    state.amountSpecifiedRemaining +=
                        I256::try_from(step.amountOut.as_u128()).expect("U256 to I256 error!");
                    state.amountCalculated = state.amountCalculated
                        + (I256::try_from((step.amountIn + step.feeAmount).as_u128())
                            .expect("U256 to I256 error!"));
                }

                //         // if the protocol fee is on, calculate how much is owed, decrement feeAmount, and increment protocolFee
                //         if (cache.feeProtocol > 0) {
                //             uint256 delta = step.feeAmount / cache.feeProtocol;
                //             step.feeAmount -= delta;
                //             state.protocolFee += uint128(delta);
                //         }
                if cache.feeProtocol > 0 {
                    let delta: U256 = step.feeAmount / cache.feeProtocol;
                    step.feeAmount -= delta;
                    state.protocolFee += delta.as_u128();
                }

                //         // update global fee tracker
                //         if (state.liquidity > 0)
                //             state.feeGrowthGlobalX128 += FullMath.mulDiv(step.feeAmount, FixedPoint128.Q128, state.liquidity);
                if state.liquidity > 0 {
                    state.feeGrowthGlobalX128 += FullMath::mulDiv(
                        step.feeAmount,
                        U256::from(FixedPoint128::Q128),
                        U256::from(state.liquidity),
                    );
                }
                //         // shift tick if we reached the next price
                //         if (state.sqrtPriceX96 == step.sqrtPriceNextX96) {
                if state.sqrtPriceX96 == step.sqrtPriceNextX96 {
                    //             // if the tick is initialized, run the tick transition
                    //             if (step.initialized) {
                    if step.initialized {
                        //                 // check for the placeholder value, which we replace with the actual value the first time the swap
                        //                 // crosses an initialized tick
                        //                 if (!cache.computedLatestObservation) {
                        //                     (cache.tickCumulative, cache.secondsPerLiquidityCumulativeX128) = observations.observeSingle(
                        //                         cache.blockTimestamp,
                        //                         0,
                        //                         slot0Start.tick,
                        //                         slot0Start.observationIndex,
                        //                         cache.liquidityStart,
                        //                         slot0Start.observationCardinality
                        //                     );
                        //                     cache.computedLatestObservation = true;
                        //                 }
                        if !cache.computedLatestObservation {
                            (
                                cache.tickCumulative,
                                cache.secondsPerLiquidityCumulativeX128,
                            ) = self.observations.observeSingle(
                                cache.blockTimestamp,
                                0,
                                slot0Start.tick,
                                slot0Start.observationIndex,
                                cache.liquidityStart,
                                slot0Start.observationCardinality,
                            );
                            cache.computedLatestObservation = true;
                        }
                        //                 int128 liquidityNet =
                        //                     ticks.cross(
                        //                         step.tickNext,
                        //                         (zeroForOne ? state.feeGrowthGlobalX128 : feeGrowthGlobal0X128),
                        //                         (zeroForOne ? feeGrowthGlobal1X128 : state.feeGrowthGlobalX128),
                        //                         cache.secondsPerLiquidityCumulativeX128,
                        //                         cache.tickCumulative,
                        //                         cache.blockTimestamp
                        //                     );
                        let mut liquidityNet: i128 = Tick::cross(
                            &mut self.ticks,
                            step.tickNext,
                            if zeroForOne {
                                state.feeGrowthGlobalX128
                            } else {
                                self.feeGrowthGlobal0X128.value
                            },
                            if zeroForOne {
                                self.feeGrowthGlobal1X128.value
                            } else {
                                state.feeGrowthGlobalX128
                            },
                            cache.secondsPerLiquidityCumulativeX128,
                            cache.tickCumulative,
                            cache.blockTimestamp,
                        );
                        //                 // if we're moving leftward, we interpret liquidityNet as the opposite sign
                        //                 // safe because liquidityNet cannot be type(int128).min
                        //                 if (zeroForOne) liquidityNet = -liquidityNet;
                        if zeroForOne {
                            liquidityNet = -liquidityNet;
                        }

                        //                 state.liquidity = LiquidityMath.addDelta(state.liquidity, liquidityNet);
                        state.liquidity = LiquidityMath::addDelta(state.liquidity, liquidityNet);
                        //             }
                    }
                    //             state.tick = zeroForOne ? step.tickNext - 1 : step.tickNext;
                    state.tick = if zeroForOne {
                        step.tickNext - 1
                    } else {
                        step.tickNext
                    };
                //         } else if (state.sqrtPriceX96 != step.sqrtPriceStartX96) {
                //             // recompute unless we're on a lower tick boundary (i.e. already transitioned ticks), and haven't moved
                //             state.tick = TickMath.getTickAtSqrtRatio(state.sqrtPriceX96);
                //         }
                } else if state.sqrtPriceX96 != step.sqrtPriceStartX96 {
                    // recompute unless we're on a lower tick boundary (i.e. already transitioned ticks), and haven't moved
                    state.tick = TickMath::getTickAtSqrtRatio(state.sqrtPriceX96);
                }
                //     }
            }

            //     // update tick and write an oracle entry if the tick change
            //     if (state.tick != slot0Start.tick) {
            //         (uint16 observationIndex, uint16 observationCardinality) =
            //             observations.write(
            //                 slot0Start.observationIndex,
            //                 cache.blockTimestamp,
            //                 slot0Start.tick,
            //                 cache.liquidityStart,
            //                 slot0Start.observationCardinality,
            //                 slot0Start.observationCardinalityNext
            //             );
            //         (slot0.sqrtPriceX96, slot0.tick, slot0.observationIndex, slot0.observationCardinality) = (
            //             state.sqrtPriceX96,
            //             state.tick,
            //             observationIndex,
            //             observationCardinality
            //         );
            //     } else {
            //         // otherwise just update the price
            //         slot0.sqrtPriceX96 = state.sqrtPriceX96;
            //     }
            if state.tick != slot0Start.tick {
                let (observationIndex, observationCardinality): (u16, u16) =
                    self.observations.write(
                        slot0Start.observationIndex,
                        cache.blockTimestamp,
                        slot0Start.tick,
                        cache.liquidityStart,
                        slot0Start.observationCardinality,
                        slot0Start.observationCardinalityNext,
                    );
                (
                    self.slot0.sqrtPriceX96.value,
                    self.slot0.tick,
                    self.slot0.observationIndex,
                    self.slot0.observationCardinality,
                ) = (
                    state.sqrtPriceX96,
                    state.tick,
                    observationIndex,
                    observationCardinality,
                );
            } else {
                // otherwise just update the price
                self.slot0.sqrtPriceX96 = Uint256::new_with_u256(state.sqrtPriceX96);
            }

            //     // update liquidity if it changed
            //     if (cache.liquidityStart != state.liquidity) liquidity = state.liquidity;
            if cache.liquidityStart != state.liquidity {
                self.liquidity = state.liquidity;
            }
            // update fee growth global and, if necessary, protocol fees
            // overflow is acceptable, protocol has to withdraw before it hits type(uint128).max fees
            //     if (zeroForOne) {
            //         feeGrowthGlobal0X128 = state.feeGrowthGlobalX128;
            //         if (state.protocolFee > 0) protocolFees.token0 += state.protocolFee;
            //     } else {
            //         feeGrowthGlobal1X128 = state.feeGrowthGlobalX128;
            //         if (state.protocolFee > 0) protocolFees.token1 += state.protocolFee;
            //     }
            if zeroForOne {
                self.feeGrowthGlobal0X128 = Uint256::new_with_u256(state.feeGrowthGlobalX128);
                if state.protocolFee > 0 {
                    self.protocolFees.token0 += state.protocolFee;
                }
            } else {
                self.feeGrowthGlobal1X128 = Uint256::new_with_u256(state.feeGrowthGlobalX128);
                if state.protocolFee > 0 {
                    self.protocolFees.token1 += state.protocolFee;
                }
            }
            ink_env::debug_println!("amountSpecified, state.amountSpecifiedRemaining,state.amountCalculated is:{:?},{:?},{:?}", amountSpecified, state.amountSpecifiedRemaining,state.amountCalculated);
            (amount0, amount1) = if zeroForOne == exactInput {
                (
                    amountSpecified - state.amountSpecifiedRemaining,
                    state.amountCalculated,
                )
            } else {
                (
                    state.amountCalculated,
                    amountSpecified - state.amountSpecifiedRemaining,
                )
            };
            ink_env::debug_println!("amount0, amount1 is:{},{}", amount0, amount1);

            // do the transfers and collect payment
            // if (zeroForOne) {
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            if zeroForOne {
                //     if (amount1 < 0) TransferHelper.safeTransfer(token1, recipient, uint256(-amount1));
                if amount1 < 0 {
                    PSP22Ref::transfer(
                        &mut self.token1,
                        recipient,
                        u128::try_from(-amount1).expect("i128 to 128 error!"),
                        vec![0u8],
                    )
                    .expect("token0 transfer error!");
                }
                //     uint256 balance0Before = balance0();
                let balance0Before: U256 = self.balance0();
                //     IUniswapV3SwapCallback(msg.sender).uniswapV3SwapCallback(amount0, amount1, data);
                ink_env::debug_println!("-------------+1");
                SwapCallbackRef::swapCallback_builder(&msg_sender, amount0, amount1, data)
                    .call_flags(CallFlags::default().set_allow_reentry(true))
                    .fire()
                    .unwrap();
                ink_env::debug_println!("-------------+2");
                //     require(balance0Before.add(uint256(amount0)) <= balance0(), 'IIA');
                assert!(
                    balance0Before + (U256::from(amount0)) <= self.balance0(),
                    "IIA"
                );
            } else {
                // } else {
                // if (amount0 < 0) TransferHelper.safeTransfer(token0, recipient, uint256(-amount0));
                if amount0 < 0 {
                    PSP22Ref::transfer(
                        &mut self.token0,
                        recipient,
                        u128::try_from(-amount0).expect("i128 to 128 error!"),
                        vec![0u8],
                    )
                    .expect("token0 transfer error!");
                }
                //     uint256 balance1Before = balance1();
                //     IUniswapV3SwapCallback(msg.sender).uniswapV3SwapCallback(amount0, amount1, data);
                //     require(balance1Before.add(uint256(amount1)) <= balance1(), 'IIA');
                let balance1Before: U256 = self.balance1();
                ink_env::debug_println!("-------------+3");
                SwapCallbackRef::swapCallback_builder(&msg_sender, amount0, amount1, data)
                    .call_flags(CallFlags::default().set_allow_reentry(true))
                    .fire()
                    .unwrap();
                ink_env::debug_println!("-------------+4");
                assert!(
                    balance1Before + (U256::from(amount1)) <= self.balance1(),
                    "IIA"
                );
            }

            //     emit Swap(msg.sender, recipient, amount0, amount1, state.sqrtPriceX96, state.liquidity, state.tick);
            self.env().emit_event(Swap {
                sender: msg_sender,
                recipient,
                amount0,
                amount1,
                sqrtPriceX96: state.sqrtPriceX96,
                liquidity: state.liquidity,
                tick: state.tick,
            });

            //     slot0.unlocked = true;
            self.slot0.unlocked = true;
            (amount0, amount1)
        }

        // function collect(
        //     address recipient,
        //     int24 tickLower,
        //     int24 tickUpper,
        //     uint128 amount0Requested,
        //     uint128 amount1Requested
        // ) external override lock returns (uint128 amount0, uint128 amount1) {
        // }
        #[ink(message)]
        #[modifiers(lock)]
        fn collect(
            &mut self,
            recipient: Address,
            tickLower: Int24,
            tickUpper: Int24,
            amount0Requested: u128,
            amount1Requested: u128,
        ) -> (u128, u128) {
            //     // we don't need to checkTicks here, because invalid positions will never have non-zero tokensOwed{0,1}
            //     Position.Info storage position = positions.get(msg.sender, tickLower, tickUpper);
            ink_env::debug_println!("^^^^^^^^^^^^^^^^1");
            let msg_sender: AccountId = ink_env::caller::<DefaultEnvironment>();
            ink_env::debug_println!("^^^^^^^^^^^^^^^^2");
            let mut position: Position::Info = self
                .positions
                .get((msg_sender, tickLower, tickUpper))
                .expect("position is not exist!");
            ink_env::debug_println!("^^^^^^^^^^^^^^^^3");
            //     amount0 = amount0Requested > position.tokensOwed0 ? position.tokensOwed0 : amount0Requested;
            //     amount1 = amount1Requested > position.tokensOwed1 ? position.tokensOwed1 : amount1Requested;
            let amount0 = if amount0Requested > position.tokensOwed0 {
                position.tokensOwed0
            } else {
                amount0Requested
            };
            ink_env::debug_println!("^^^^^^^^^^^^^^^^4");
            let amount1 = if amount1Requested > position.tokensOwed1 {
                position.tokensOwed1
            } else {
                amount1Requested
            };
            ink_env::debug_println!("^^^^^^^^^^^^^^^^5");
            //     if (amount0 > 0) {
            //         position.tokensOwed0 -= amount0;
            //         TransferHelper.safeTransfer(token0, recipient, amount0);
            //     }
            if amount0 > 0 {
                ink_env::debug_println!("^^^^^^^^^^^^^^^^6");
                position.tokensOwed0 -= amount0;
                ink_env::debug_println!("^^^^^^^^^^^^^^^^7");
                // TransferHelper::safeTransfer instead of transfer of PSP22.
                PSP22Ref::transfer(&mut self.token0, recipient, amount0, vec![0u8])
                    .expect("token0 transfer error!");
                ink_env::debug_println!("^^^^^^^^^^^^^^^^8");
            }
            //     if (amount1 > 0) {
            //         position.tokensOwed1 -= amount1;
            //         TransferHelper.safeTransfer(token1, recipient, amount1);
            //     }
            if amount1 > 0 {
                ink_env::debug_println!("^^^^^^^^^^^^^^^^9");
                position.tokensOwed1 -= amount1;
                ink_env::debug_println!("^^^^^^^^^^^^^^^^10");
                PSP22Ref::transfer(&mut self.token1, recipient, amount1, vec![0u8])
                    .expect("token1 transfer error!");
                ink_env::debug_println!("^^^^^^^^^^^^^^^^11");
            }
            //     emit Collect(msg.sender, recipient, tickLower, tickUpper, amount0, amount1);
            self.env().emit_event(Collect {
                owner: msg_sender,
                recipient,
                tickLower,
                tickUpper,
                amount0,
                amount1,
            });
            // ink_lang::codegen::EmitEvent::<PoolContract>::emit_event(self.env(), Collect {
            //         owner:msg_sender,
            //         recipient,
            //         tickLower,
            //         tickUpper,
            //         amount0,
            //         amount1,
            //     });
            ink_env::debug_println!("^^^^^^^^^^^^^^^^12");
            self.positions
                .insert((msg_sender, tickLower, tickUpper), &position);
            ink_env::debug_println!("^^^^^^^^^^^^^^^^13");
            (amount0, amount1)
        }

        /// @inheritdoc IUniswapV3PoolActions
        /// @dev not locked because it initializes unlocked
        // #[ink(message, payable)]
        #[ink(message, payable)]
        fn initialize(&mut self, sqrtPriceX96: U160) {
            // require(slot0.sqrtPriceX96 == 0, 'AI');
            assert!(self.slot0.sqrtPriceX96.value.is_zero(), "AI");
            // int24 tick = TickMath.getTickAtSqrtRatio(sqrtPriceX96);
            let tick: Int24 = getTickAtSqrtRatio(sqrtPriceX96);
            // (uint16 cardinality, uint16 cardinalityNext) = observations.initialize(_blockTimestamp());
            let time_stamp = self.env().block_timestamp();
            let (cardinality, cardinalityNext) = self.observations.initialize(time_stamp);
            // slot0 = Slot0({
            //     sqrtPriceX96: sqrtPriceX96,
            //     tick: tick,
            //     observationIndex: 0,
            //     observationCardinality: cardinality,
            //     observationCardinalityNext: cardinalityNext,
            //     feeProtocol: 0,
            //     unlocked: true
            // });
            self.slot0 = Slot0 {
                sqrtPriceX96: Uint160::new_with_u256(sqrtPriceX96),
                tick: tick,
                observationIndex: 0,
                observationCardinality: cardinality,
                observationCardinalityNext: cardinalityNext,
                feeProtocol: 0,
                unlocked: true,
            };
            // emit Initialize(sqrtPriceX96, tick);
            self.env().emit_event(Initialize { sqrtPriceX96, tick });
        }

        #[ink(message)]
        fn getSlot0(&self) -> Slot0 {
            self.slot0.clone()
        }

        #[ink(message)]
        fn setUnLock(&mut self, unlock: bool) {
            self.slot0.unlocked = unlock;
        }

        #[ink(message)]
        fn positions(
            &self,
            position_address: Address,
            tick_lower: Int24,
            tick_upper: Int24,
        ) -> Position::Info {
            self.positions
                .get((position_address, tick_lower, tick_upper))
                .unwrap_or(Default::default())
        }

        #[ink(message)]
        #[modifiers(lock)]
        fn burn(&mut self, tickLower: Int24, tickUpper: Int24, amount: u128) -> (U256, U256) {
            // (Position.Info storage position, int256 amount0Int, int256 amount1Int) =
            // _modifyPosition(
            //     ModifyPositionParams({
            //         owner: msg.sender,
            //         tickLower: tickLower,
            //         tickUpper: tickUpper,
            //         liquidityDelta: -int256(amount).toInt128()
            //     })
            // );
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            let (mut position, amount0Int, amount1Int) =
                self._modifyPosition(ModifyPositionParams {
                    owner: msg_sender,
                    tickLower,
                    tickUpper,
                    liquidityDelta: -i128::try_from(amount).expect("amount to i128 failed!"),
                });

            // amount0 = uint256(-amount0Int);
            // amount1 = uint256(-amount1Int);
            let amount0 = U256::from(amount0Int.abs());
            let amount1 = U256::from(amount1Int.abs());

            // if (amount0 > 0 || amount1 > 0) {
            //     (position.tokensOwed0, position.tokensOwed1) = (
            //         position.tokensOwed0 + uint128(amount0),
            //         position.tokensOwed1 + uint128(amount1)
            //     );
            // }
            if amount0 > U256::zero() {
                position.tokensOwed0 += amount0.as_u128();
            }
            if amount1 > U256::zero() {
                position.tokensOwed1 += amount1.as_u128();
            }
            self.env().emit_event(Burn {
                owner: msg_sender,
                tickLower,
                tickUpper,
                amount,
                amount0,
                amount1,
            });
            // emit Burn(msg.sender, tickLower, tickUpper, amount, amount0, amount1);
            (amount0, amount1)
        }

        /// @inheritdoc IUniswapV3PoolActions
        /// @dev noDelegateCall is applied indirectly via _modifyPosition
        #[ink(message)]
        #[modifiers(lock)]
        fn mint(
            &mut self,
            recipient: Address,
            tickLower: Int24,
            tickUpper: Int24,
            amount: u128,
            data: Vec<u8>,
        ) -> (U256, U256) {
            //uint256 amount0, uint256 amount1
            ink_env::debug_println!("--------------1");
            assert!(amount > 0, "amount must big than 0");

            // let (_, int256 amount0Int, int256 amount1Int) =
            //     _modifyPosition(
            //         ModifyPositionParams({
            //             owner: recipient,
            //             tickLower: tickLower,
            //             tickUpper: tickUpper,
            //             liquidityDelta: int256(amount).toInt128()
            //         })
            //     );
            let (_, amount0Int, amount1Int) = self._modifyPosition(ModifyPositionParams {
                owner: recipient,
                tickLower: tickLower,
                tickUpper: tickUpper,
                liquidityDelta: i128::try_from(amount).unwrap(),
            });
            ink_env::debug_println!("--------------2");

            let amount0: U256 = U256::from(amount0Int);
            let amount1: U256 = U256::from(amount1Int);

            let mut balance0Before: U256 = U256::zero();
            let mut balance1Before: U256 = U256::zero();
            // if (amount0 > 0) balance0Before = balance0();
            if amount0 > U256::from(0) {
                balance0Before = self.balance0();
            }
            // if (amount1 > 0) balance1Before = balance1();
            if amount1 > U256::from(0) {
                balance1Before = self.balance1();
            }
            ink_env::debug_println!("**************3");
            let manager_address: AccountId = ink_env::caller::<DefaultEnvironment>();
            ink_env::debug_println!("manager_address is:{:?}", manager_address);
            ink_env::debug_println!("amount0 is:{:?}", amount0);
            ink_env::debug_println!("amount1 is:{:?}", amount1);
            ink_env::debug_println!("data is:{:?}", data);
            // TODO recovery call back
            LiquidityManagementTraitRef::uniswapV3MintCallback_builder(
                &manager_address,
                amount0,
                amount1,
                data,
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap();
            ink_env::debug_println!("**************3.1");
            // let address_of_this = ink_env::account_id::<DefaultEnvironment>();
            // let balance = PSP22Ref::balance_of(&self.token0,address_of_this);
            // ink_env::debug_println!("balance is:{:?}",balance);
            if amount0 > U256::from(0) {
                assert!(balance0Before + amount0 <= self.balance0(), "M0");
            }
            if amount1 > U256::from(0) {
                assert!(balance1Before + amount1 <= self.balance1(), "M1");
            }
            ink_env::debug_println!("**************4");
            // emit Mint(msg.sender, recipient, tickLower, tickUpper, amount, amount0, amount1);
            self.env().emit_event(Mint {
                sender: manager_address,
                owner: recipient,
                tickLower,
                tickUpper,
                amount,
                amount0,
                amount1,
            });
            ink_env::debug_println!("**************5");
            (amount0, amount1)
        }
    }

    /// @notice Emitted when the collected protocol fees are withdrawn by the factory owner
    /// @param sender The address that collects the protocol fees
    /// @param recipient The address that receives the collected protocol fees
    /// @param amount0 The amount of token0 protocol fees that is withdrawn
    /// @param amount0 The amount of token1 protocol fees that is withdrawn
    #[ink(event)]
    pub struct CollectProtocol{
        #[ink(topic)]
        sender:Address, 
        #[ink(topic)]
        recipient:Address, 
        amount0:u128, 
        amount1:u128
    }

    /// @notice Emitted when the protocol fee is changed by the pool
    /// @param feeProtocol0Old The previous value of the token0 protocol fee
    /// @param feeProtocol1Old The previous value of the token1 protocol fee
    /// @param feeProtocol0New The updated value of the token0 protocol fee
    /// @param feeProtocol1New The updated value of the token1 protocol fee
    #[ink(event)]
    pub struct SetFeeProtocol {
        feeProtocol0Old: u8,
        feeProtocol1Old: u8,
        feeProtocol0New: u8,
        feeProtocol1New: u8,
    }

    /// @notice Emitted by the pool for any flashes of token0/token1
    /// @param sender The address that initiated the swap call, and that received the callback
    /// @param recipient The address that received the tokens from flash
    /// @param amount0 The amount of token0 that was flashed
    /// @param amount1 The amount of token1 that was flashed
    /// @param paid0 The amount of token0 paid for the flash, which can exceed the amount0 plus the fee
    /// @param paid1 The amount of token1 paid for the flash, which can exceed the amount1 plus the fee
    #[ink(event)]
    pub struct Flash {
        #[ink(topic)]
        sender: Address,
        #[ink(topic)]
        recipient: Address,
        amount0: U256,
        amount1: U256,
        paid0: U256,
        paid1: U256,
    }

    /// @notice Emitted when liquidity is minted for a given position
    /// @param sender The address that minted the liquidity
    /// @param owner The owner of the position and recipient of any minted liquidity
    /// @param tickLower The lower tick of the position
    /// @param tickUpper The upper tick of the position
    /// @param amount The amount of liquidity minted to the position range
    /// @param amount0 How much token0 was required for the minted liquidity
    /// @param amount1 How much token1 was required for the minted liquidity
    #[ink(event)]
    pub struct Mint {
        #[ink(topic)]
        sender: Address,
        #[ink(topic)]
        owner: Address,
        // #[ink(topic)]
        tickLower: Int24,
        // #[ink(topic)]
        tickUpper: Int24,
        amount: u128,
        amount0: U256,
        amount1: U256,
    }

    /// @notice Emitted when fees are collected by the owner of a position
    /// @dev Collect events may be emitted with zero amount0 and amount1 when the caller chooses not to collect fees
    /// @param owner The owner of the position for which fees are collected
    /// @param tickLower The lower tick of the position
    /// @param tickUpper The upper tick of the position
    /// @param amount0 The amount of token0 fees collected
    /// @param amount1 The amount of token1 fees collected
    #[ink(event)]
    pub struct Collect {
        #[ink(topic)]
        owner: Address,
        recipient: Address,
        #[ink(topic)]
        tickLower: Int24,
        #[ink(topic)]
        tickUpper: Int24,
        amount0: u128,
        amount1: u128,
    }

    struct ModifyPositionParams {
        // the address that owns the position
        owner: Address,
        // the lower and upper tick of the position
        tickLower: Int24,
        tickUpper: Int24,
        // any change in liquidity
        liquidityDelta: i128,
    }

    impl PoolContract {
        #[ink(constructor, payable)]
        pub fn new(
            factory: Address,
            token0: Address,
            token1: Address,
            fee: Uint24,
            tickSpacing: Int24,
        ) -> Self {
            // (factory, token0, token1, fee, _tickSpacing) = IUniswapV3PoolDeployer(msg.sender).parameters();
            //
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.factory = factory;
                instance.token0 = token0;
                instance.token1 = token1;
                instance.fee = fee;
                instance.tickSpacing = tickSpacing;
                instance.fee_growth_global0_x128 = Uint160::new();
                instance.fee_growth_global1_x128 = Uint160::new();
                instance.liquidity = Default::default();
                instance.max_liquidity_per_tick =
                    libs::tick_spacing_to_max_liquidity_per_tick(tickSpacing);
                instance.slot0 = Default::default();
                instance.slot0.unlocked = true;
                instance.observations = Observations::new();
                instance.maxLiquidityPerTick = Tick::tickSpacingToMaxLiquidityPerTick(tickSpacing);
                instance.protocolFees = Default::default();
                instance.no_delegate_call.original = ink_env::account_id::<DefaultEnvironment>();
                ink_env::debug_println!("----------------6");
            })
        }

        /// @dev Effect some changes to a position
        /// @param params the position details and the change to the position's liquidity to effect
        /// @return position a storage pointer referencing the position with the given owner and tick range
        /// @return amount0 the amount of token0 owed to the pool, negative if the pool should pay the recipient
        /// @return amount1 the amount of token1 owed to the pool, negative if the pool should pay the recipient
        // TODO noDelegateCall add
        fn _modifyPosition(
            &mut self,
            params: ModifyPositionParams,
        ) -> (Position::Info, Int256, Int256) {
            ink_env::debug_println!("--------------5");
            checkTicks(params.tickLower, params.tickUpper);

            // Slot0 memory _slot0 = slot0;
            let _slot0 = self.slot0.clone(); // SLOAD for gas optimization
            ink_env::debug_println!("--------------6");
            let position = self._updatePosition(
                params.owner,
                params.tickLower,
                params.tickUpper,
                params.liquidityDelta,
                _slot0.tick,
            );
            let mut amount0 = 0;
            let mut amount1 = 0;
            ink_env::debug_println!("--------------7");
            if params.liquidityDelta != 0 {
                if _slot0.tick < params.tickLower {
                    // current tick is below the passed range; liquidity can only become in range by crossing from left to
                    // right, when we'll need _more_ token0 (it's becoming more valuable) so user must provide it
                    amount0 = SqrtPriceMath::getAmount0Delta(
                        TickMath::getSqrtRatioAtTick(params.tickLower),
                        TickMath::getSqrtRatioAtTick(params.tickUpper),
                        params.liquidityDelta,
                    );
                    ink_env::debug_println!("--------------8");
                } else if _slot0.tick < params.tickUpper {
                    // current tick is inside the passed range
                    let liquidityBefore = self.liquidity; // Sloan for gas optimization
                    ink_env::debug_println!("--------------9");
                    // write an oracle entry
                    (
                        self.slot0.observationIndex,
                        self.slot0.observationCardinality,
                    ) = self.observations.write(
                        _slot0.observationIndex,
                        self._blockTimestamp(),
                        _slot0.tick,
                        liquidityBefore,
                        _slot0.observationCardinality,
                        _slot0.observationCardinalityNext,
                    );
                    ink_env::debug_println!("--------------10");
                    amount0 = SqrtPriceMath::getAmount0Delta(
                        _slot0.sqrtPriceX96.value,
                        TickMath::getSqrtRatioAtTick(params.tickUpper),
                        params.liquidityDelta,
                    );
                    amount1 = SqrtPriceMath::getAmount1Delta(
                        TickMath::getSqrtRatioAtTick(params.tickLower),
                        _slot0.sqrtPriceX96.value,
                        params.liquidityDelta,
                    );
                    ink_env::debug_println!(
                        "self.liquidity is:{:?},liquidityBefore is:{:?}",
                        self.liquidity,
                        liquidityBefore
                    );
                    self.liquidity =
                        LiquidityMath::addDelta(liquidityBefore, params.liquidityDelta);
                } else {
                    // current tick is above the passed range; liquidity can only become in range by crossing from right to
                    // left, when we'll need _more_ token1 (it's becoming more valuable) so user must provide it
                    amount1 = SqrtPriceMath::getAmount1Delta(
                        TickMath::getSqrtRatioAtTick(params.tickLower),
                        TickMath::getSqrtRatioAtTick(params.tickUpper),
                        params.liquidityDelta,
                    );
                    ink_env::debug_println!("--------------12");
                }
            }
            (position, amount0, amount1)
        }

        /// @dev Gets and updates a position with the given liquidity delta
        /// @param owner the owner of the position
        /// @param tickLower the lower tick of the position's tick range
        /// @param tickUpper the upper tick of the position's tick range
        /// @param tick the current tick, passed to avoid sloads
        fn _updatePosition(
            &mut self,
            owner: Address,
            tickLower: Int24,
            tickUpper: Int24,
            liquidityDelta: i128,
            tick: Int24,
        ) -> Position::Info {
            // position = positions.get(owner, tickLower, tickUpper);
            let mut position: Position::Info = self
                .positions
                .get((owner, tickLower, tickUpper))
                .unwrap_or(Default::default());
            ink_env::debug_println!("++++++++++++7");
            let _feeGrowthGlobal0X128: U256 = self.feeGrowthGlobal0X128.value; // LOAD for gas optimization
            let _feeGrowthGlobal1X128: U256 = self.feeGrowthGlobal1X128.value; // SLOAD for gas optimization

            // if we need to update the ticks, do it
            let flippedLower: bool = false;
            let flippedUpper: bool = false;
            if liquidityDelta != 0 {
                // uint32 time = _blockTimestamp();
                let time = self.env().block_timestamp();
                ink_env::debug_println!("++++++++++++8");
                let (tickCumulative, secondsPerLiquidityCumulativeX128) =
                    self.observations.observeSingle(
                        time,
                        0,
                        self.slot0.tick,
                        self.slot0.observationIndex,
                        self.liquidity,
                        self.slot0.observationCardinality,
                    );
                let mut tick_info_lower: Tick::Info =
                    self.ticks.get(tickLower).unwrap_or(Default::default());
                ink_env::debug_println!("++++++++++++9");
                let flippedLower = tick_info_lower.update(
                    tickLower,
                    tick,
                    liquidityDelta,
                    _feeGrowthGlobal0X128,
                    _feeGrowthGlobal1X128,
                    secondsPerLiquidityCumulativeX128,
                    tickCumulative,
                    time,
                    false,
                    self.maxLiquidityPerTick,
                );
                ink_env::debug_println!("++++++++++++10");
                self.ticks.insert(tickLower, &tick_info_lower);

                let mut tick_info_upper = self.ticks.get(tickUpper).unwrap_or(Default::default());
                ink_env::debug_println!("++++++++++++11");
                let flippedUpper = tick_info_upper.update(
                    tickUpper,
                    tick,
                    liquidityDelta,
                    _feeGrowthGlobal0X128,
                    _feeGrowthGlobal1X128,
                    secondsPerLiquidityCumulativeX128,
                    tickCumulative,
                    time,
                    true,
                    self.maxLiquidityPerTick,
                );
                self.ticks.insert(tickUpper, &tick_info_upper);
                ink_env::debug_println!("++++++++++++12");

                if flippedLower {
                    let (wordPos, mask) = TickBitmap::flipTick(tickLower, self.tickSpacing);
                    // TickBitmap self[wordPos] ^= mask; 移到外部进行计算
                    let tick_new_value = self
                        .tickBitmap
                        .get(wordPos)
                        .unwrap_or(Default::default())
                        .value
                        ^ mask;
                    ink_env::debug_println!("++++++++++++13 flippedLower");
                    self.tickBitmap
                        .insert(wordPos, &Uint256::new_with_u256(tick_new_value));
                    // self[wordPos] ^= mask;
                }
                if flippedUpper {
                    let (wordPos, mask) = TickBitmap::flipTick(tickUpper, self.tickSpacing);
                    // self[wordPos] ^= mask;
                    let tick_new_value = self
                        .tickBitmap
                        .get(wordPos)
                        .unwrap_or(Default::default())
                        .value
                        ^ mask;
                    ink_env::debug_println!("++++++++++++13 flippedUpper");
                    self.tickBitmap
                        .insert(wordPos, &Uint256::new_with_u256(tick_new_value));
                }
            }
            ink_env::debug_println!("++++++++++++14");
            let (feeGrowthInside0X128, feeGrowthInside1X128) = self.getFeeGrowthInside(
                tickLower,
                tickUpper,
                tick,
                _feeGrowthGlobal0X128,
                _feeGrowthGlobal1X128,
            );
            ink_env::debug_println!("++++++++++++15");
            position.update(liquidityDelta, feeGrowthInside0X128, feeGrowthInside1X128);
            self.positions
                .insert((owner, tickLower, tickUpper), &position);
            // clear any tick data that is no longer needed
            if liquidityDelta < 0 {
                if flippedLower {
                    self.ticks.remove(tickLower);
                }
                if flippedUpper {
                    self.ticks.remove(tickUpper);
                }
            }
            position
        }

        /// @dev Returns the block timestamp truncated to 32 bits, i.e. mod 2**32. This method is overridden in tests.
        pub fn _blockTimestamp(&self) -> u64 {
            // return uint32(block.timestamp); // truncation is desired
            ink_env::block_timestamp::<DefaultEnvironment>()
        }

        /// @notice Retrieves fee growth data
        /// @param self The mapping containing all tick information for initialized ticks
        /// @param tickLower The lower tick boundary of the position
        /// @param tickUpper The upper tick boundary of the position
        /// @param tickCurrent The current tick
        /// @param feeGrowthGlobal0X128 The all-time global fee growth, per unit of liquidity, in token0
        /// @param feeGrowthGlobal1X128 The all-time global fee growth, per unit of liquidity, in token1
        /// @return feeGrowthInside0X128 The all-time fee growth in token0, per unit of liquidity, inside the position's tick boundaries
        /// @return feeGrowthInside1X128 The all-time fee growth in token1, per unit of liquidity, inside the position's tick boundaries
        pub fn getFeeGrowthInside(
            &mut self,
            tickLower: Int24,
            tickUpper: Int24,
            tickCurrent: Int24,
            feeGrowthGlobal0X128: U256,
            feeGrowthGlobal1X128: U256,
        ) -> (U256, U256) {
            let lower: Tick::Info = self.ticks.get(tickLower).unwrap_or(Default::default());
            let upper: Tick::Info = self.ticks.get(tickUpper).unwrap_or(Default::default());

            // calculate fee growth below
            let feeGrowthBelow0X128: U256;
            let feeGrowthBelow1X128: U256;
            if tickCurrent >= tickLower {
                feeGrowthBelow0X128 = lower.feeGrowthOutside0X128.value;
                feeGrowthBelow1X128 = lower.feeGrowthOutside1X128.value;
            } else {
                feeGrowthBelow0X128 = feeGrowthGlobal0X128 - lower.feeGrowthOutside0X128.value;
                feeGrowthBelow1X128 = feeGrowthGlobal1X128 - lower.feeGrowthOutside1X128.value;
            }

            // calculate fee growth above
            let feeGrowthAbove0X128: U256;
            let feeGrowthAbove1X128: U256;
            if tickCurrent < tickUpper {
                feeGrowthAbove0X128 = upper.feeGrowthOutside0X128.value;
                feeGrowthAbove1X128 = upper.feeGrowthOutside1X128.value;
            } else {
                feeGrowthAbove0X128 = feeGrowthGlobal0X128 - upper.feeGrowthOutside0X128.value;
                feeGrowthAbove1X128 = feeGrowthGlobal1X128 - upper.feeGrowthOutside1X128.value;
            }

            let feeGrowthInside0X128 =
                feeGrowthGlobal0X128 - feeGrowthBelow0X128 - feeGrowthAbove0X128;
            let feeGrowthInside1X128 =
                feeGrowthGlobal1X128 - feeGrowthBelow1X128 - feeGrowthAbove1X128;
            (feeGrowthInside0X128, feeGrowthInside1X128)
        }

        /// @dev Get the pool's balance of token0
        /// @dev This function is gas optimized to avoid a redundant extcodesize check in addition to the returndatasize
        /// check
        fn balance0(&self) -> U256 {
            // (bool success, bytes memory data) =
            //     token0.staticcall(abi.encodeWithSelector(IERC20Minimal.balanceOf.selector, address(this)));
            // require(success && data.length >= 32);
            // return abi.decode(data, (uint256));
            let address_of_this = ink_env::account_id::<DefaultEnvironment>();
            U256::from(PSP22Ref::balance_of(&self.token0, address_of_this))
        }

        /// @dev Get the pool's balance of token1
        /// @dev This function is gas optimized to avoid a redundant extcodesize check in addition to the returndatasize
        /// check
        fn balance1(&self) -> U256 {
            // (bool success, bytes memory data) =
            //     token1.staticcall(abi.encodeWithSelector(IERC20Minimal.balanceOf.selector, address(this)));
            // require(success && data.length >= 32);
            // return abi.decode(data, (uint256));
            let address_of_this = ink_env::account_id::<DefaultEnvironment>();
            U256::from(PSP22Ref::balance_of(&self.token1, address_of_this))
        }

        // /// @inheritdoc IUniswapV3Factory
        // #[ink(message)]
        // pub fn create_pool(&mut self, tokenA: Address, tokenB: Address, fee: u32) -> AccountId {
        //     self.env().caller();
        //     [0; 32].into()
        // }
    }

    /// @notice Emitted by the pool for any swaps between token0 and token1
    /// @param sender The address that initiated the swap call, and that received the callback
    /// @param recipient The address that received the output of the swap
    /// @param amount0 The delta of the token0 balance of the pool
    /// @param amount1 The delta of the token1 balance of the pool
    /// @param sqrtPriceX96 The sqrt(price) of the pool after the swap, as a Q64.96
    /// @param liquidity The liquidity of the pool after the swap
    /// @param tick The log base 1.0001 of price of the pool after the swap
    #[ink(event)]
    pub struct Swap {
        #[ink(topic)]
        sender: Address,
        #[ink(topic)]
        recipient: Address,
        amount0: Int256,
        amount1: Int256,
        sqrtPriceX96: U160,
        liquidity: u128,
        tick: Int24,
    }

    /// @notice Emitted exactly once by a pool when #initialize is first called on the pool
    /// @dev Mint/Burn/Swap cannot be emitted by the pool before Initialize
    /// @param sqrtPriceX96 The initial sqrt price of the pool, as a Q64.96
    /// @param tick The initial tick of the pool, i.e. log base 1.0001 of the starting price of the pool
    #[ink(event)]
    pub struct Initialize {
        #[ink(topic)]
        sqrtPriceX96: U160,
        #[ink(topic)]
        tick: Int24,
    }

    /// @notice Emitted when a position's liquidity is removed
    /// @dev Does not withdraw any fees earned by the liquidity position, which must be withdrawn via #collect
    /// @param owner The owner of the position for which liquidity is removed
    /// @param tickLower The lower tick of the position
    /// @param tickUpper The upper tick of the position
    /// @param amount The amount of liquidity to remove
    /// @param amount0 The amount of token0 withdrawn
    /// @param amount1 The amount of token1 withdrawn
    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        owner: Address,
        #[ink(topic)]
        tickLower: Int24,
        #[ink(topic)]
        tickUpper: Int24,
        amount: u128,
        amount0: U256,
        amount1: U256,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<Environment>()
        }

        fn set_next_caller(caller: AccountId) {
            ink_env::test::set_caller::<Environment>(caller);
        }

        #[ink::test]
        fn register_works() {
            let default_accounts = default_accounts();

            set_next_caller(default_accounts.alice);
            // factory:Address,token0: Address, token1: Address, fee: Uint24, tickSpacing: Int24
            let pool_contract = PoolContract::new(
                default_accounts.alice,
                default_accounts.alice,
                default_accounts.alice,
                500,
                10,
            );
            ink_env::debug_println!("test success:{:?}", pool_contract);
            println!("test success:{:?}", pool_contract);
            // assert_eq!(weth9_contract.metadata.name,Some(String::from("weth9")));
        }
    }

    /// @dev Common checks for valid tick inputs.
    fn checkTicks(tickLower: Int24, tickUpper: Int24) {
        assert!(tickLower < tickUpper, "TLU");
        assert!(tickLower >= TickMath::MIN_TICK, "TLM");
        assert!(tickUpper <= TickMath::MAX_TICK, "TUM");
    }
}
