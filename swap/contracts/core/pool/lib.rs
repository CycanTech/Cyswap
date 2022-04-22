#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

#[brush::contract]
pub mod crab_swap_pool {
    use ink_env::DefaultEnvironment;
    use ink_storage::{
        Mapping,
        traits::{PackedLayout, SpreadLayout, StorageLayout},
    };
    use libs::{core::{tick_math, oracle::Observations}, get_tick_at_sqrt_ratio};
    use scale::{Decode, Encode, WrapperTypeEncode};
    use primitives::{U160,Uint160, Int24};
    #[cfg(feature = "std")]
    use ink_metadata::layout::{FieldLayout, Layout, StructLayout};
    use ink_storage::traits::SpreadAllocate;
    use crabswap::traits::core::pool::*;
    use ink_lang::codegen::Env;
    type Address = AccountId;
    type Uint24 = u32;
    use ink_lang::codegen::EmitEvent;


    // accumulated protocol fees in token0/token1 units
    #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
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
        /// @inheritdoc IUniswapV3PoolState
        pub observations:Observations,
    }


    /// @notice Emitted exactly once by a pool when #initialize is first called on the pool
    /// @dev Mint/Burn/Swap cannot be emitted by the pool before Initialize
    /// @param sqrtPriceX96 The initial sqrt price of the pool, as a Q64.96
    /// @param tick The initial tick of the pool, i.e. log base 1.0001 of the starting price of the pool
    #[ink(event)]
    pub struct Initialize{
        #[ink(topic)]
        sqrtPriceX96:U160, 
        #[ink(topic)]
        tick:Int24,
    }

    impl Pool for PoolContract{
        /// @inheritdoc IUniswapV3PoolActions
        /// @dev not locked because it initializes unlocked
        #[ink(message, payable)]
        fn initialize(&mut self,sqrtPriceX96:U160){
            // require(slot0.sqrtPriceX96 == 0, 'AI');
            assert!(self.slot0.sqrtPriceX96.value.is_zero(), "AI");
            // int24 tick = TickMath.getTickAtSqrtRatio(sqrtPriceX96);
            let tick:Int24 = get_tick_at_sqrt_ratio(sqrtPriceX96);
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
            self.slot0 = Slot0{
                    sqrtPriceX96: Uint160::new_with_u256(sqrtPriceX96),
                    tick: tick,
                    observationIndex: 0,
                    observationCardinality: cardinality,
                    observationCardinalityNext: cardinalityNext,
                    feeProtocol: 0,
                    unlocked: true
                };
            // emit Initialize(sqrtPriceX96, tick);
            self.env().emit_event(Initialize{sqrtPriceX96,tick});
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
                ink_env::debug_message("----------------1");
                instance.factory = factory;
                instance.token0 = token0;
                instance.token1 = token1;
                instance.fee = fee;
                instance.tick_spacing = tick_spacing;
                instance.fee_growth_global0_x128 = Uint160::new();
                ink_env::debug_message("----------------2");
                instance.fee_growth_global1_x128=Uint160::new();
                ink_env::debug_message("----------------3");
                instance.liquidity = Default::default();
                ink_env::debug_message("----------------4");
                instance.max_liquidity_per_tick = libs::tick_spacing_to_max_liquidity_per_tick(tick_spacing);
                ink_env::debug_message("----------------5");
                instance.slot0 = Default::default();
                instance.observations = Observations::new();
                ink_env::debug_println!("----------------6");
            })
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

        fn default_accounts(
        ) -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
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
            let pool_contract = PoolContract::new(default_accounts.alice,default_accounts.alice,default_accounts.alice,500,10);
            ink_env::debug_println!("test success:{:?}",pool_contract);
            println!("test success:{:?}",pool_contract);
            // assert_eq!(weth9_contract.metadata.name,Some(String::from("weth9")));
        }
        
    }
}
