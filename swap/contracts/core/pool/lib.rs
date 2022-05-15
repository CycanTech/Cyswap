#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

#[brush::contract]
pub mod crab_swap_pool {
    use crabswap::traits::core::pool::*;
    use ink_env::DefaultEnvironment;
    use ink_lang::codegen::Env;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use ink_storage::{
        traits::{PackedLayout, SpreadLayout, StorageLayout},
        Mapping,
    };
    use libs::{
        core::{oracle::Observations, TickMath,Position,SqrtPriceMath,LiquidityMath,Tick,TickBitmap},
        get_tick_at_sqrt_ratio,
    };
    use primitives::{Int24, Uint160, U160, U256,Int256, Uint256, Address};
    use scale::{Decode, Encode};
    type Uint24 = u32;
    use ink_lang::codegen::EmitEvent;
    use brush::contracts::psp22::extensions::metadata::*;
    use brush::modifiers;
    use crabswap::traits::periphery::LiquidityManagement::*;
    use ink_env::{
        CallFlags,
    };
    use ink_prelude::vec;
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
        pub maxLiquidityPerTick:u128,

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
        pub feeGrowthGlobal0X128:Uint256,
        /// @inheritdoc IUniswapV3PoolState
        pub feeGrowthGlobal1X128:Uint256,

        
        // pub protocolFees: ProtocolFees,
        pub liquidity: u128,
        // mapping(int24 => Tick.Info) pub ticks;
        pub ticks:Mapping<Int24,Tick::Info>,
        // /// @inheritdoc IUniswapV3PoolState
        // mapping(int16 => uint256) public override tickBitmap;
        pub tickBitmap:Mapping<i16,Uint256>,
        // /// @inheritdoc IUniswapV3PoolState
        // mapping(bytes32 => Position.Info) public override positions;
        pub positions: Mapping<(Address,Int24,Int24), Position::Info>,
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

    /// @notice Emitted when a position's liquidity is removed
    /// @dev Does not withdraw any fees earned by the liquidity position, which must be withdrawn via #collect
    /// @param owner The owner of the position for which liquidity is removed
    /// @param tickLower The lower tick of the position
    /// @param tickUpper The upper tick of the position
    /// @param amount The amount of liquidity to remove
    /// @param amount0 The amount of token0 withdrawn
    /// @param amount1 The amount of token1 withdrawn
    #[ink(event)]
    pub struct Burn{
        #[ink(topic)]
        owner:Address,
        #[ink(topic)]
        tickLower:Int24,
        #[ink(topic)]
        tickUpper:Int24,
        amount:u128,
        amount0:U256,
        amount1:U256
    }



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

    impl PoolAction for PoolContract {
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
        ) -> (u128, u128){
            //     // we don't need to checkTicks here, because invalid positions will never have non-zero tokensOwed{0,1}
            //     Position.Info storage position = positions.get(msg.sender, tickLower, tickUpper);
            ink_env::debug_println!("^^^^^^^^^^^^^^^^1");
            let msg_sender:AccountId = ink_env::caller::<DefaultEnvironment>();
            ink_env::debug_println!("^^^^^^^^^^^^^^^^2");
            let mut position:Position::Info = self.positions.get((msg_sender, tickLower, tickUpper)).expect("position is not exist!");
            ink_env::debug_println!("^^^^^^^^^^^^^^^^3");
            //     amount0 = amount0Requested > position.tokensOwed0 ? position.tokensOwed0 : amount0Requested;
            //     amount1 = amount1Requested > position.tokensOwed1 ? position.tokensOwed1 : amount1Requested;
            let amount0 = if amount0Requested > position.tokensOwed0 {
                position.tokensOwed0
            }else{
                amount0Requested
            };
            ink_env::debug_println!("^^^^^^^^^^^^^^^^4");
            let amount1 = if amount1Requested > position.tokensOwed1{
                position.tokensOwed1
            }else{
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
                PSP22Ref::transfer(&mut self.token0, recipient, amount0, vec![0u8]).expect("token0 transfer error!");
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
                PSP22Ref::transfer(&mut self.token1, recipient, amount1, vec![0u8]).expect("token1 transfer error!");
                ink_env::debug_println!("^^^^^^^^^^^^^^^^11");
            }
            //     emit Collect(msg.sender, recipient, tickLower, tickUpper, amount0, amount1);
            self.env().emit_event(Collect { 
                owner:msg_sender,
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
            self.positions.insert((msg_sender, tickLower, tickUpper),&position);
            ink_env::debug_println!("^^^^^^^^^^^^^^^^13");
            (amount0,amount1)
        }

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

        #[ink(message)]
        fn setUnLock(&mut self,unlock:bool){
            self.slot0.unlocked = unlock;
        }

        #[ink(message)]
        fn positions(&self,position_address:Address,tick_lower:Int24,tick_upper:Int24) -> Position::Info {
            self.positions.get((position_address,tick_lower,tick_upper)).unwrap_or(Default::default())
        }

        #[ink(message)]
        #[modifiers(lock)]
        fn burn(&mut self,
            tickLower:Int24,
            tickUpper:Int24,
            amount:u128,
        ) -> (U256,U256){
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
            let (mut position, amount0Int, amount1Int) =self._modifyPosition(ModifyPositionParams{
                owner:msg_sender,
                tickLower,
                tickUpper,
                liquidityDelta:-i128::try_from(amount).expect("amount to i128 failed!")
            }
            );

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
            if amount0 > U256::zero(){
                position.tokensOwed0 += amount0.as_u128();
            }
            if amount1 > U256::zero(){
                position.tokensOwed1 += amount1.as_u128();
            }
            self.env().emit_event(Burn {
                owner:msg_sender, 
                tickLower, 
                tickUpper, 
                amount, 
                amount0, 
                amount1});
            // emit Burn(msg.sender, tickLower, tickUpper, amount, amount0, amount1);
            (amount0,amount1)
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
        ) -> (U256, U256) { //uint256 amount0, uint256 amount1
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

            let amount0:U256 = U256::from(amount0Int);
            let amount1:U256 = U256::from(amount1Int);

            let mut balance0Before: U256=U256::zero();
            let mut balance1Before: U256=U256::zero();
            // if (amount0 > 0) balance0Before = balance0();
            if amount0 > U256::from(0) {
                balance0Before = self.balance0();
            }
            // if (amount1 > 0) balance1Before = balance1();
            if amount1 > U256::from(0) {
                balance1Before = self.balance1();
            }
            ink_env::debug_println!("**************3");
            let manager_address:AccountId = ink_env::caller::<DefaultEnvironment>();
            ink_env::debug_println!("manager_address is:{:?}",manager_address);
            ink_env::debug_println!("amount0 is:{:?}",amount0);
            ink_env::debug_println!("amount1 is:{:?}",amount1);
            ink_env::debug_println!("data is:{:?}",data);
            // TODO recovery call back
            LiquidityManagementTraitRef::uniswapV3MintCallback_builder(&manager_address,amount0, amount1, data)
                .call_flags(CallFlags::default().set_allow_reentry(true)).fire().unwrap();
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
                sender:manager_address, 
                owner:recipient, 
                tickLower, 
                tickUpper, 
                amount, 
                amount0, 
                amount1});
            ink_env::debug_println!("**************5");
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
    pub struct Collect{
        #[ink(topic)]
        owner:Address,
        recipient:Address,
        #[ink(topic)]
        tickLower:Int24,
        #[ink(topic)]
        tickUpper:Int24,
        amount0:u128,
        amount1:u128,
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
        #[ink(constructor,payable)]
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
                ink_env::debug_message("----------------1");
                instance.factory = factory;
                instance.token0 = token0;
                instance.token1 = token1;
                instance.fee = fee;
                instance.tickSpacing = tickSpacing;
                instance.fee_growth_global0_x128 = Uint160::new();
                ink_env::debug_message("----------------2");
                instance.fee_growth_global1_x128 = Uint160::new();
                ink_env::debug_message("----------------3");
                instance.liquidity = Default::default();
                ink_env::debug_message("----------------4");
                instance.max_liquidity_per_tick =
                    libs::tick_spacing_to_max_liquidity_per_tick(tickSpacing);
                ink_env::debug_message("----------------5");
                instance.slot0 = Default::default();
                instance.slot0.unlocked = true;
                instance.observations = Observations::new();
                instance.maxLiquidityPerTick = Tick::tickSpacingToMaxLiquidityPerTick(tickSpacing);
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
        let mut amount0=0;
        let mut amount1=0;
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
                (self.slot0.observationIndex, self.slot0.observationCardinality) = self.observations.write(
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
                ink_env::debug_println!("self.liquidity is:{:?},liquidityBefore is:{:?}",self.liquidity,liquidityBefore);
                self.liquidity = LiquidityMath::addDelta(liquidityBefore, params.liquidityDelta);
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
        (position,amount0,amount1)
    }

        /// @dev Gets and updates a position with the given liquidity delta
        /// @param owner the owner of the position
        /// @param tickLower the lower tick of the position's tick range
        /// @param tickUpper the upper tick of the position's tick range
        /// @param tick the current tick, passed to avoid sloads
        fn _updatePosition(&mut self,
            owner: Address,
            tickLower: Int24,
            tickUpper: Int24,
            liquidityDelta: i128,
            tick: Int24,
        ) -> Position::Info {
            // position = positions.get(owner, tickLower, tickUpper);
            let mut position:Position::Info = self.positions.get((owner, tickLower, tickUpper)).unwrap_or(Default::default());
            ink_env::debug_println!("++++++++++++7");
            let _feeGrowthGlobal0X128: U256 = self.feeGrowthGlobal0X128.value; // LOAD for gas optimization
            let _feeGrowthGlobal1X128: U256 = self.feeGrowthGlobal1X128.value; // SLOAD for gas optimization

            // if we need to update the ticks, do it
            let flippedLower: bool=false;
            let flippedUpper: bool=false;
            if liquidityDelta != 0 {
                // uint32 time = _blockTimestamp();
                let time = self.env().block_timestamp();
                ink_env::debug_println!("++++++++++++8");
                let (tickCumulative, secondsPerLiquidityCumulativeX128) = self.observations
                    .observeSingle(
                        time,
                        0,
                        self.slot0.tick,
                        self.slot0.observationIndex,
                        self.liquidity,
                        self.slot0.observationCardinality,
                    );
                let mut tick_info_lower:Tick::Info = self.ticks.get(tickLower).unwrap_or(Default::default());
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
                self.ticks.insert(tickLower,&tick_info_lower);

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
                self.ticks.insert(tickUpper,&tick_info_upper);
                ink_env::debug_println!("++++++++++++12");
                
                if flippedLower {
                    let (wordPos, mask) = TickBitmap::flipTick(tickLower, self.tickSpacing);
                    // TickBitmap self[wordPos] ^= mask; 移到外部进行计算
                    let tick_new_value = self.tickBitmap.get(wordPos).unwrap_or(Default::default()).value^mask;
                    ink_env::debug_println!("++++++++++++13 flippedLower");
                    self.tickBitmap.insert(wordPos,&Uint256::new_with_u256(tick_new_value));
                    // self[wordPos] ^= mask;
                }
                if flippedUpper {
                    let (wordPos, mask) = TickBitmap::flipTick(tickUpper, self.tickSpacing);
                    // self[wordPos] ^= mask;
                    let tick_new_value = self.tickBitmap.get(wordPos).unwrap_or(Default::default()).value^mask;
                    ink_env::debug_println!("++++++++++++13 flippedUpper");
                    self.tickBitmap.insert(wordPos,&Uint256::new_with_u256(tick_new_value));
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
            self.positions.insert((owner, tickLower, tickUpper),&position);
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
        tickLower:Int24,
        tickUpper:Int24,
        tickCurrent:Int24,
        feeGrowthGlobal0X128:U256,
        feeGrowthGlobal1X128:U256
    ) -> (U256, U256) {
        let lower:Tick::Info = self.ticks.get(tickLower).unwrap_or(Default::default());
        let upper:Tick::Info = self.ticks.get(tickUpper).unwrap_or(Default::default());

        // calculate fee growth below
        let feeGrowthBelow0X128:U256;
        let feeGrowthBelow1X128:U256;
        if tickCurrent >= tickLower {
            feeGrowthBelow0X128 = lower.feeGrowthOutside0X128.value;
            feeGrowthBelow1X128 = lower.feeGrowthOutside1X128.value;
        } else {
            feeGrowthBelow0X128 = feeGrowthGlobal0X128 - lower.feeGrowthOutside0X128.value;
            feeGrowthBelow1X128 = feeGrowthGlobal1X128 - lower.feeGrowthOutside1X128.value;
        }

        // calculate fee growth above
        let feeGrowthAbove0X128:U256;
        let feeGrowthAbove1X128:U256;
        if tickCurrent < tickUpper {
            feeGrowthAbove0X128 = upper.feeGrowthOutside0X128.value;
            feeGrowthAbove1X128 = upper.feeGrowthOutside1X128.value;
        } else {
            feeGrowthAbove0X128 = feeGrowthGlobal0X128 - upper.feeGrowthOutside0X128.value;
            feeGrowthAbove1X128 = feeGrowthGlobal1X128 - upper.feeGrowthOutside1X128.value;
        }

        let feeGrowthInside0X128 = feeGrowthGlobal0X128 - feeGrowthBelow0X128 - feeGrowthAbove0X128;
        let feeGrowthInside1X128 = feeGrowthGlobal1X128 - feeGrowthBelow1X128 - feeGrowthAbove1X128;
        (feeGrowthInside0X128,feeGrowthInside1X128)
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
