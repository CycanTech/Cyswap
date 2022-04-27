#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

#[brush::contract]
pub mod crab_swap_pool {
    use crabswap::traits::core::pool::*;
    use ink_env::DefaultEnvironment;
    use ink_lang::codegen::Env;
    #[cfg(feature = "std")]
    use ink_metadata::layout::{FieldLayout, Layout, StructLayout};
    use ink_storage::traits::SpreadAllocate;
    use ink_storage::{
        traits::{PackedLayout, SpreadLayout, StorageLayout},
        Mapping,
    };
    use libs::{
        core::{oracle::Observations, TickMath,Position,SqrtPriceMath,LiquidityMath},
        get_tick_at_sqrt_ratio,
    };
    use primitives::{Int24, Uint160, U160, U256,Int256};
    use scale::{Decode, Encode, WrapperTypeEncode};
    type Address = AccountId;
    type Uint24 = u32;
    use ink_lang::codegen::EmitEvent;
    use brush::contracts::psp22::extensions::metadata::*;
    use crabswap::traits::periphery::LiquidityManagement::LiquidityManagementTraitRef;

    // accumulated protocol fees in token0/token1 units
    #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    struct ProtocolFees {
        token0: u128,
        token1: u128,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct PoolContract {
        // address public immutable override factory;
        // address public immutable override token0;
        // address public immutable override token1;
        // uint24 public immutable override fee;
        // int24 public immutable override tickSpacing;
        // uint128 public immutable override maxLiquidityPerTick;

        // follow six parameter is immutable
        pub factory: Address,
        pub token0: Address,
        pub token1: Address,
        pub fee: Uint24,
        pub tick_spacing: Int24,
        pub max_liquidity_per_tick: u128,
        pub slot0: Slot0,

        pub fee_growth_global0_x128: Uint160,
        pub fee_growth_global1_x128: Uint160,
        
        // pub protocolFees: ProtocolFees,
        pub liquidity: u128,
        // mapping(int24 => Tick.Info) pub ticks;
        // /// @inheritdoc IUniswapV3PoolState
        // mapping(int16 => uint256) public override tickBitmap;
        // /// @inheritdoc IUniswapV3PoolState
        // mapping(bytes32 => Position.Info) public override positions;
        pub positions: Mapping<Vec<u8>, Position::Info>,
        /// @inheritdoc IUniswapV3PoolState
        pub observations: Observations,
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



    impl PoolAction for PoolContract {
        /// @inheritdoc IUniswapV3PoolActions
        /// @dev not locked because it initializes unlocked
        // #[ink(message, payable)]
        #[ink(message, payable)]
        fn initialize(&mut self, sqrtPriceX96: U160) {
            // require(slot0.sqrtPriceX96 == 0, 'AI');
            assert!(self.slot0.sqrtPriceX96.value.is_zero(), "AI");
            // int24 tick = TickMath.getTickAtSqrtRatio(sqrtPriceX96);
            let tick: Int24 = get_tick_at_sqrt_ratio(sqrtPriceX96);
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

        /// @inheritdoc IUniswapV3PoolActions
        /// @dev noDelegateCall is applied indirectly via _modifyPosition
        #[ink(message)]
        fn mint(
            &mut self,
            recipient: Address,
            tickLower: Int24,
            tickUpper: Int24,
            amount: u128,
            data: Vec<u8>,
        ) -> (U256, U256) { //uint256 amount0, uint256 amount1
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
            let (_, amount0Int, amount1Int) = _modifyPosition(ModifyPositionParams {
                owner: recipient,
                tickLower: tickLower,
                tickUpper: tickUpper,
                liquidityDelta: i128::try_from(amount).unwrap(),
            });

            let amount0:U256 = U256::from(amount0Int);
            let amount1:U256 = U256::from(amount1Int);

            let balance0Before: U256;
            let balance1Before: U256;
            // if (amount0 > 0) balance0Before = balance0();
            if amount0 > U256::from(0) {
                balance0Before = self.balance0();
            }
            // if (amount1 > 0) balance1Before = balance1();
            if amount1 > U256::from(0) {
                balance1Before = self.balance1();
            }
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            LiquidityManagementTraitRef::uniswapV3MintCallback(&msg_sender,amount0, amount1, data);
            if amount0 > U256::from(0) {
                assert!(balance0Before + amount0 <= self.balance0(), "M0");
            }
            if amount1 > U256::from(0) {
                assert!(balance1Before + amount1 <= self.balance1(), "M1");
            }

            // emit Mint(msg.sender, recipient, tickLower, tickUpper, amount, amount0, amount1);
            ink_env::emit_event(Mint {
                sender:msg_sender, 
                owner:recipient, 
                tickLower, 
                tickUpper, 
                amount, 
                amount0, 
                amount1});
                (amount0,amount1)
        }
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
        #[ink(topic)]
        tickLower: Int24,
        #[ink(topic)]
        tickUpper: Int24,
        amount: u128,
        amount0: U256,
        amount1: U256,
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
        #[ink(constructor)]
        pub fn new(
            factory: Address,
            token0: Address,
            token1: Address,
            fee: Uint24,
            tick_spacing: Int24,
        ) -> Self {
            // (factory, token0, token1, fee, _tickSpacing) = IUniswapV3PoolDeployer(msg.sender).parameters();
            // tickSpacing = _tickSpacing;
            // TODO maxLiquidityPerTick = Tick.tickSpacingToMaxLiquidityPerTick(_tickSpacing);
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                ink_env::debug_message("----------------1");
                instance.factory = factory;
                instance.token0 = token0;
                instance.token1 = token1;
                instance.fee = fee;
                instance.tick_spacing = tick_spacing;
                instance.fee_growth_global0_x128 = Uint160::new();
                ink_env::debug_message("----------------2");
                instance.fee_growth_global1_x128 = Uint160::new();
                ink_env::debug_message("----------------3");
                instance.liquidity = Default::default();
                ink_env::debug_message("----------------4");
                instance.max_liquidity_per_tick =
                    libs::tick_spacing_to_max_liquidity_per_tick(tick_spacing);
                ink_env::debug_message("----------------5");
                instance.slot0 = Default::default();
                instance.observations = Observations::new();
                ink_env::debug_println!("----------------6");
            })
        }

        /// @dev Effect some changes to a position
    /// @param params the position details and the change to the position's liquidity to effect
    /// @return position a storage pointer referencing the position with the given owner and tick range
    /// @return amount0 the amount of token0 owed to the pool, negative if the pool should pay the recipient
    /// @return amount1 the amount of token1 owed to the pool, negative if the pool should pay the recipient
    // TODO noDelegateCall add
    fn _modifyPosition(&mut self,params: ModifyPositionParams) -> (Position::Info, Int256, Int256) {
        checkTicks(params.tickLower, params.tickUpper);

        // Slot0 memory _slot0 = slot0;
        let _slot0 = self.slot0; // SLOAD for gas optimization

        let position = self._updatePosition(
            params.owner,
            params.tickLower,
            params.tickUpper,
            params.liquidityDelta,
            _slot0.tick,
        );
        let amount0;
        let amount1;
        let liquidity;
        if params.liquidityDelta != 0 {
            if _slot0.tick < params.tickLower {
                // current tick is below the passed range; liquidity can only become in range by crossing from left to
                // right, when we'll need _more_ token0 (it's becoming more valuable) so user must provide it
                amount0 = SqrtPriceMath::getAmount0Delta(
                    TickMath::getSqrtRatioAtTick(params.tickLower),
                    TickMath::getSqrtRatioAtTick(params.tickUpper),
                    params.liquidityDelta,
                );
            } else if _slot0.tick < params.tickUpper {
                // current tick is inside the passed range
                let liquidityBefore = self.liquidity; // Sloan for gas optimization

                // write an oracle entry
                (self.slot0.observationIndex, self.slot0.observationCardinality) = self.observations.write(
                    _slot0.observationIndex,
                    self._blockTimestamp(),
                    _slot0.tick,
                    liquidityBefore,
                    _slot0.observationCardinality,
                    _slot0.observationCardinalityNext,
                );

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

                liquidity = LiquidityMath::addDelta(liquidityBefore, params.liquidityDelta);
            } else {
                // current tick is above the passed range; liquidity can only become in range by crossing from right to
                // left, when we'll need _more_ token1 (it's becoming more valuable) so user must provide it
                amount1 = SqrtPriceMath::getAmount1Delta(
                    TickMath::getSqrtRatioAtTick(params.tickLower),
                    TickMath::getSqrtRatioAtTick(params.tickUpper),
                    params.liquidityDelta,
                );
            }
        }
        (position,amount0,amount1)
    }

        /// @dev Gets and updates a position with the given liquidity delta
        /// @param owner the owner of the position
        /// @param tickLower the lower tick of the position's tick range
        /// @param tickUpper the upper tick of the position's tick range
        /// @param tick the current tick, passed to avoid sloads
        fn _updatePosition(&self,
            owner: Address,
            tickLower: Int24,
            tickUpper: Int24,
            liquidityDelta: int128,
            tick: Int24,
        ) -> (Position::Info) {
            position = positions.get(owner, tickLower, tickUpper);

            let _feeGrowthGlobal0X128: U256 = feeGrowthGlobal0X128; // LOAD for gas optimization
            let _feeGrowthGlobal1X128: U256 = feeGrowthGlobal1X128; // SLOAD for gas optimization

            // if we need to update the ticks, do it
            let flippedLower: bool;
            let flippedUpper: bool;
            if liquidityDelta != 0 {
                // uint32 time = _blockTimestamp();
                let time = self.env().block_timestamp();
                let (tickCumulative, secondsPerLiquidityCumulativeX128) = observations
                    .observeSingle(
                        time,
                        0,
                        slot0.tick,
                        slot0.observationIndex,
                        liquidity,
                        slot0.observationCardinality,
                    );

                let flippedLower = ticks.update(
                    tickLower,
                    tick,
                    liquidityDelta,
                    _feeGrowthGlobal0X128,
                    _feeGrowthGlobal1X128,
                    secondsPerLiquidityCumulativeX128,
                    tickCumulative,
                    time,
                    false,
                    maxLiquidityPerTick,
                );
                let flippedUpper = ticks.update(
                    tickUpper,
                    tick,
                    liquidityDelta,
                    _feeGrowthGlobal0X128,
                    _feeGrowthGlobal1X128,
                    secondsPerLiquidityCumulativeX128,
                    tickCumulative,
                    time,
                    true,
                    maxLiquidityPerTick,
                );

                if (flippedLower) {
                    tickBitmap.flipTick(tickLower, tickSpacing);
                }
                if (flippedUpper) {
                    tickBitmap.flipTick(tickUpper, tickSpacing);
                }
            }

            let (feeGrowthInside0X128, feeGrowthInside1X128) = ticks.getFeeGrowthInside(
                tickLower,
                tickUpper,
                tick,
                _feeGrowthGlobal0X128,
                _feeGrowthGlobal1X128,
            );

            position.update(liquidityDelta, feeGrowthInside0X128, feeGrowthInside1X128);

            // clear any tick data that is no longer needed
            if (liquidityDelta < 0) {
                if (flippedLower) {
                    ticks.clear(tickLower);
                }
                if (flippedUpper) {
                    ticks.clear(tickUpper);
                }
            }
        }

        /// @dev Returns the block timestamp truncated to 32 bits, i.e. mod 2**32. This method is overridden in tests.
    pub fn _blockTimestamp(&self) -> u64 {
        // return uint32(block.timestamp); // truncation is desired
        ink_env::block_timestamp::<DefaultEnvironment>()
        
    }

    /// @dev Get the pool's balance of token0
    /// @dev This function is gas optimized to avoid a redundant extcodesize check in addition to the returndatasize
    /// check
    fn balance0(&self)-> U256 {
        // (bool success, bytes memory data) =
        //     token0.staticcall(abi.encodeWithSelector(IERC20Minimal.balanceOf.selector, address(this)));
        // require(success && data.length >= 32);
        // return abi.decode(data, (uint256));
        let address_of_this = ink_env::account_id::<DefaultEnvironment>();
        U256::from(PSP22Ref::balance_of(&self.token0,address_of_this))
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
        U256::from(PSP22Ref::balance_of(&self.token1,address_of_this))
    }

        // /// @inheritdoc IUniswapV3Factory
        // #[ink(message)]
        // pub fn create_pool(&mut self, tokenA: Address, tokenB: Address, fee: u32) -> AccountId {
        //     self.env().caller();
        //     [0; 32].into()
        // }
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
            // factory:Address,token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24
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
