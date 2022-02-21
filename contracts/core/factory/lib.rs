#![cfg_attr(not(feature = "std"), no_std)]

// SPDX-License-Identifier: GPL-2.0-or-later
use ink_lang as ink;

pub use self::uniswap_v3_factory::{
    UniswapV3Factory,
    UniswapV3FactoryRef,
};

#[ink::contract]
mod uniswap_v3_factory {
    use ink_env::hash::{Sha2x256, HashOutput};
    use ink_lang::ToAccountId;
    use ink_storage::{lazy::Mapping, traits::{SpreadLayout, PackedLayout, StorageLayout}};
    use scale::{Encode, Decode};

    use pool::UniswapV3PoolRef;

    type Address = AccountId;
    type Uint24 = u32;
    type Int24 = i32;

    static  accumulator_code_hash:[u8;32] = [0;32];
    #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
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
        pub fee_amount_tick_spacing:Mapping<u32,Int24>,

        // mapping(address => mapping(address => mapping(uint24 => address))) public override getPool;
        /// @inheritdoc IPeripheryImmutableState
        pub pool_map: Mapping<(AccountId,AccountId,u32),AccountId>,
        pub parameters:Parameters,
    }

    // emit PoolCreated(token0, token1, fee, tickSpacing, pool);
    #[ink(event)]
    pub struct PoolCreated{
        #[ink(topic)]
        token0:Address,
        #[ink(topic)]
        token1:Address,
        fee:Uint24,
        tick_spacing:Int24,
        #[ink(topic)]
        pool:AccountId,
    }

    impl UniswapV3Factory {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {
                fee_amount_tick_spacing:Default::default(),
                pool_map:Default::default(),
                parameters:Default::default(),
            };
            instance
        }

        
        #[ink(message)]
        pub fn get_pool(&self,token0:AccountId, token1:AccountId, fee:u32)->AccountId{
            let key = (token0,token1,fee);
            self.pool_map.get(key).unwrap_or([0u8;32].into())
        }

        /// @inheritdoc IUniswapV3Factory
        #[ink(message)]
        pub fn create_pool(&mut self,tokenA:Address,tokenB:Address,fee:u32)->AccountId{
            assert!(tokenA!=tokenB,"token A should not equals token B");
            let (token0,token1);
            if tokenA < tokenB {
                token0 = tokenA;
                token1 = tokenB;
            }else{
                token0 = tokenB;
                token1 = tokenA;
            }
            let tick_spacing = self.fee_amount_tick_spacing.get(fee).unwrap_or(0);
            assert!(tick_spacing!=0,"tick spacing should not be zero!");
            assert!(self.pool_map.get((token0,token1,fee)).is_none(),"pool have been exist!");
            let address_this = self.env().account_id();

            //start deploy the pool contract and initial.
            let pool = self.deploy(address_this,token0,token1,fee,tick_spacing).to_account_id();
            Self::env().emit_event(PoolCreated {
                token0,
                token1,
                fee,
                tick_spacing,
                pool,
            });
            pool
        }

        fn deploy(&mut self,address_this: Address, token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24) -> UniswapV3PoolRef {
            let total_balance = Self::env().balance();
            let encodable = (address_this, token0, token1,fee); // Implements `scale::Encode`
            let mut salt = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            ink_env::hash_encoded::<Sha2x256, _>(&encodable, &mut salt);
            let pool_address = UniswapV3PoolRef::new(address_this,token0, token1, fee, tick_spacing)
                    .endowment(total_balance / 4)
                    .code_hash(accumulator_code_hash.into())
                    .salt_bytes(salt)
                    .instantiate()
                    .unwrap_or_else(|error| {
                        panic!(
                            "failed at instantiating the Accumulator contract: {:?}",
                            error
                        )
                    });
            self.pool_map.insert((token0,token1,fee),&pool_address.to_account_id());
            pool_address
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
