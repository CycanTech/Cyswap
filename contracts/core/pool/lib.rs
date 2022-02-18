#![cfg_attr(not(feature = "std"), no_std)]

// SPDX-License-Identifier: GPL-2.0-or-later
use ink_lang as ink;

pub use self::uniswap_v3_factory::{
    UniswapV3Factory,
    UniswapV3FactoryRef,
};

#[ink::contract]
mod uniswap_v3_factory {
    use ink_storage::{lazy::Mapping, traits::{SpreadLayout, PackedLayout, StorageLayout}};
    use scale::{Encode, Decode};
    type Address = AccountId;
    type Uint24 = u32;
    type Int24 = i32;

    #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout,StorageLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Parameters {
        factory:Address,
        token0:Address,
        token1:Address,
        fee:Uint24,
        tick_spacing:Int24,
    }

    impl Default for Parameters{
        fn default() -> Self {
            Self { 
                factory: Default::default(), 
                token0: Default::default(), 
                token1: Default::default(), 
                fee: Default::default(), 
                tick_spacing: Default::default(),
            }
        }
    }

    #[ink(storage)]
    pub struct UniswapV3Factory {

        // mapping(uint24 => int24) public override feeAmountTickSpacing;
        pub fee_amount_tick_spacing:Mapping<u32,i32>,

        // mapping(address => mapping(address => mapping(uint24 => address))) public override getPool;
        /// @inheritdoc IPeripheryImmutableState
        pub pool: Mapping<(AccountId,AccountId,u32),AccountId>,
        pub parameters:Parameters,
    }

    impl UniswapV3Factory {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {
                fee_amount_tick_spacing:Default::default(),
                pool:Default::default(),
                parameters:Default::default(),
            };
            instance
        }

        #[ink(message)]
        pub fn get_pool(&self,token0:AccountId, token1:AccountId, fee:u32)->AccountId{
            let key = (token0,token1,fee);
            self.pool.get(key).unwrap_or([0u8;32].into())
        }

        /// @inheritdoc IUniswapV3Factory
        #[ink(message)]
        pub fn create_pool(&mut self,tokenA:Address,tokenB:Address,fee:u32)->AccountId{
            assert!(tokenA!=tokenB,"token A should not equals token B");
            let (token0,token1);
            if tokenA < tokenB {
                (token0,token1) = (tokenA,tokenB);
            }else{
                (token0,token1) = (tokenB,tokenA);
            }
            let tick_spacing = self.fee_amount_tick_spacing.get(fee).unwrap_or(0);
            assert!(tick_spacing!=0,"tick spacing should not be zero!");
            assert!(self.pool.get((token0,token1,fee)).is_none(),"pool have been exist!");
            let address_this = self.env().account_id();
            [0;32].into()
        }

        // function createPool(
        //     address tokenA,
        //     address tokenB,
        //     uint24 fee
        // ) external override noDelegateCall returns (address pool) {
        //     require(tokenA != tokenB);
        //     (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        //     require(token0 != address(0));
        //     int24 tickSpacing = feeAmountTickSpacing[fee];
        //     require(tickSpacing != 0);
        //     require(getPool[token0][token1][fee] == address(0));
        //     pool = deploy(address(this), token0, token1, fee, tickSpacing);
        //     getPool[token0][token1][fee] = pool;
        //     // populate mapping in the reverse direction, deliberate choice to avoid the cost of comparing addresses
        //     getPool[token1][token0][fee] = pool;
        //     emit PoolCreated(token0, token1, fee, tickSpacing, pool);
        // }
    }

}
