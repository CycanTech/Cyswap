#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]


#[brush::contract]
pub mod crab_swap_pool {
    use ink_storage::{
        Mapping,
        traits::{PackedLayout, SpreadLayout, StorageLayout},
    };
    use libs::core::tick_math;
    use scale::{Decode, Encode, WrapperTypeEncode};
    use primitives::Uint160;
    #[cfg(feature = "std")]
    use ink_metadata::layout::{FieldLayout, Layout, StructLayout};
    use ink_storage::traits::SpreadAllocate;
    use crabswap::traits::core::pool::*;
    type Address = AccountId;
    type Uint24 = u32;
    type Int24 = i32;


    // accumulated protocol fees in token0/token1 units
    #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    struct ProtocolFees {
        token0: u128,
        token1: u128,
    }

    #[ink(storage)]
    #[derive(Default,SpreadAllocate)]
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
    }

    impl Pool for PoolContract{
        /// @inheritdoc IUniswapV3PoolActions
        /// @dev not locked because it initializes unlocked
        #[ink(message, payable)]
        fn initialize(&mut self,sqrtPriceX96:Uint160){
            // require(slot0.sqrtPriceX96 == 0, 'AI');
            // int24 tick = TickMath.getTickAtSqrtRatio(sqrtPriceX96);
            // (uint16 cardinality, uint16 cardinalityNext) = observations.initialize(_blockTimestamp());
            // slot0 = Slot0({
            //     sqrtPriceX96: sqrtPriceX96,
            //     tick: tick,
            //     observationIndex: 0,
            //     observationCardinality: cardinality,
            //     observationCardinalityNext: cardinalityNext,
            //     feeProtocol: 0,
            //     unlocked: true
            // });
            // emit Initialize(sqrtPriceX96, tick);
            assert!(self.slot0.sqrtPriceX96.value.is_zero(), "AI");
            // let tick:Int24 = tick_math::getTickAtSqrtRatio(sqrtPriceX96);
        }

        #[ink(message)]
        fn slot0(&self)->Slot0{
            self.slot0.clone()
        }
    }

    impl  PoolContract {
        #[ink(constructor)]
        pub fn new(factory:Address,token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24) -> Self {
            // (factory, token0, token1, fee, _tickSpacing) = IUniswapV3PoolDeployer(msg.sender).parameters();
            // tickSpacing = _tickSpacing;
            // TODO maxLiquidityPerTick = Tick.tickSpacingToMaxLiquidityPerTick(_tickSpacing);
            ink_lang::utils::initialize_contract(|instance:&mut Self|{
                instance.factory = factory;
                instance.token0 = token0;
                instance.token1 = token1;
                instance.fee = fee;
                instance.tick_spacing = tick_spacing;
                instance.max_liquidity_per_tick = Default::default();
                instance.fee_growth_global0_x128 = Default::default();
                instance.fee_growth_global1_x128=Uint160::new([0u64;4]);
                instance.liquidity = Default::default();
                instance.max_liquidity_per_tick = libs::tick_spacing_to_max_liquidity_per_tick(tick_spacing);
            })
        }

        /// @inheritdoc IUniswapV3Factory
        #[ink(message)]
        pub fn create_pool(&mut self, tokenA: Address, tokenB: Address, fee: u32) -> AccountId {
            self.env().caller();
            [0; 32].into()
        }
       
    }
}
