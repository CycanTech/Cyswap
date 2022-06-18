#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

#[brush::contract]
pub mod OracleTest {
    use ink_env::DefaultEnvironment;
    use ink_storage::traits::StorageLayout;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::{SpreadAllocate};
    use ink_storage::traits::{PackedLayout, SpreadLayout};
    use libs::core::oracle::Observations;
    use primitives::{Int24, I56, U160};
    use scale::{Decode, Encode};
    use libs::core::oracle::Observation;
    use ink_prelude::string::String;
    use ink_prelude::string::ToString;

    #[derive(
        Default, Debug, Decode, Encode, Copy, Clone, SpreadAllocate, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct InitializeParams {
        time: u64,
        tick: Int24,
        liquidity: u128,
    }

    // struct UpdateParams {
    //     uint32 advanceTimeBy;
    //     int24 tick;
    //     uint128 liquidity;
    // }
    #[derive(
        Default, Debug, Decode, Encode, Copy, Clone, SpreadAllocate, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct UpdateParams {
        pub advanceTimeBy: u64,
        pub tick: Int24,
        pub liquidity: u128,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct OracleTestContract {
        pub time: u64,
        pub tick: Int24,
        pub liquidity: u128,
        pub index: u16,
        pub cardinality: u16,
        pub cardinalityNext: u16,
        pub observations: Observations,
    }

    #[derive(
        Default, Debug, Decode, Encode, Clone, SpreadAllocate, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct ObservationOutput {
        // the block timestamp of the observation
        pub blockTimestamp: u64,
        // the tick accumulator, i.e. tick * time elapsed since the pool was first initialized
        pub tickCumulative: i64,
        // the seconds per liquidity, i.e. seconds elapsed / max(1, liquidity) since the pool was first initialized
        pub secondsPerLiquidityCumulativeX128: String,
        // whether or not the observation is initialized
        pub initialized: bool,
    }

    impl From<Observation> for ObservationOutput{
        fn from(obs:Observation)->ObservationOutput{
            ObservationOutput{
                blockTimestamp:obs.blockTimestamp,
                tickCumulative:obs.tickCumulative,
                secondsPerLiquidityCumulativeX128:obs.secondsPerLiquidityCumulativeX128.value.to_string(),
                initialized:obs.initialized
            }
        }
    }

    impl OracleTestContract {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut OracleTestContract| {
                instance.time = 0;
                instance.tick = 0;
                instance.liquidity = 0;
                instance.index = 0;
                instance.cardinality = 0;
                instance.cardinalityNext = 0;
                instance.observations = Default::default();

            })
        }

        #[ink(message)]
        pub fn observations(&self,index:u16)->ObservationOutput{
            let result = self.observations.obs.get(index).unwrap();
            let output:ObservationOutput = result.into(); 
            ink_env::debug_println!("output is:{:?}",output);
            return output;
        }
        #[ink(message)]
        pub fn cardinality(&self)->u16{
            self.cardinality
        }

        #[ink(message)]
        pub fn cardinalityNext(&self)->u16{
            self.cardinalityNext
        }
        #[ink(message)]
        pub fn initialize(&mut self, params: InitializeParams) {
            // require(self.cardinality == 0, "already initialized");
            assert!(self.cardinality == 0, "already initialized");
            self.time = params.time;
            self.tick = params.tick;
            self.liquidity = params.liquidity;
            let (cardinality, cardinalityNext) = self.observations.initialize(params.time);
            self.cardinality = cardinality;
            self.cardinalityNext = cardinalityNext;
        }

        // function advanceTime(uint32 by) public {
        //     time += by;
        // }

        #[ink(message)]
        pub fn advanceTime(&mut self, by: u64) {
            self.time += by;
        }

        #[ink(message)]
        pub fn index(&self) ->u16{
            self.index
        }

        // // write an observation, then change tick and liquidity
        #[ink(message)]
        pub fn update(&mut self, params: UpdateParams) {
            // advanceTime(params.advanceTimeBy);
            // (index, cardinality) = observations.write(index, time, tick, liquidity, cardinality, cardinalityNext);
            // tick = params.tick;
            // liquidity = params.liquidity;
            self.advanceTime(params.advanceTimeBy);
            let (index, cardinality) = self.observations.write(
                self.index,
                self.time,
                self.tick,
                self.liquidity,
                self.cardinality,
                self.cardinalityNext,
            );
            self.index = index;
            self.cardinality = cardinality;
            self.tick = params.tick;
            self.liquidity = params.liquidity;
        }

        #[ink(message)]
        // function batchUpdate(UpdateParams[] calldata params) external {
        pub fn batchUpdate(&mut self, params: Vec<UpdateParams>) {
            //     // sload everything
            //     int24 _tick = tick;
            let mut _tick: Int24 = self.tick;
            // uint128 _liquidity = liquidity;
            let mut _liquidity: u128 = self.liquidity;
            //     uint16 _index = index;
            let _index: u16 = self.index;
            //     uint16 _cardinality = cardinality;
            let _cardinality: u16 = self.cardinality;
            //     uint16 _cardinalityNext = cardinalityNext;
            let _cardinalityNext: u16 = self.cardinalityNext;
            //     uint32 _time = time;
            let mut _time: u64 = self.time;

            //     for (uint256 i = 0; i < params.length; i++) {
            for param in params {
                //         _time += params[i].advanceTimeBy;
                _time += param.advanceTimeBy;
                //         (_index, _cardinality) = observations.write(
                //             _index,
                //             _time,
                //             _tick,
                //             _liquidity,
                //             _cardinality,
                //             _cardinalityNext
                //         );
                //         _tick = params[i].tick;
                //         _liquidity = params[i].liquidity;
                //     }
                let (_index, _cardinality) = self.observations.write(
                    _index,
                    _time,
                    _tick,
                    _liquidity,
                    _cardinality,
                    _cardinalityNext,
                );
                _tick = param.tick;
                _liquidity = param.liquidity;
            }

            //     // sstore everything
            self.tick = _tick;
            self.liquidity = _liquidity;
            self.index = _index;
            self.cardinality = _cardinality;
            self.time = _time;
        }

        #[ink(message)]
        pub fn grow(&mut self, _cardinalityNext: u16) {
            self.cardinalityNext =
                (&mut self.observations).grow(self.cardinalityNext, _cardinalityNext);
        }

        pub fn observe(&mut self, secondsAgos: Vec<u64>) -> (Vec<I56>, Vec<U160>) {
            return (&mut self.observations).observe(
                self.time,
                secondsAgos,
                self.tick,
                self.index,
                self.liquidity,
                self.cardinality,
            );
        }

        // function getGasCostOfObserve(uint32[] calldata secondsAgos) external view returns (uint256) {
        //     (uint32 _time, int24 _tick, uint128 _liquidity, uint16 _index) = (time, tick, liquidity, index);
        //     uint256 gasBefore = gasleft();
        //     observations.observe(_time, secondsAgos, _tick, _index, _liquidity, cardinality);
        //     return gasBefore - gasleft();
        // }
        pub fn getGasCostOfObserve(&mut self, secondsAgos: Vec<u64>) -> u64 {
            let (_time, _tick, _liquidity, _index) =
                (self.time, self.tick, self.liquidity, self.index);
            let gasBefore: u64 = ink_env::gas_left::<DefaultEnvironment>();
            (&mut self.observations).observe(
                _time,
                secondsAgos,
                _tick,
                _index,
                _liquidity,
                self.cardinality,
            );
            return gasBefore - ink_env::gas_left::<DefaultEnvironment>();
        }
    }
}
