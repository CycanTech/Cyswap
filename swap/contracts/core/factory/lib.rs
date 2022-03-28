#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
// SPDX-License-Identifier: GPL-2.0-or-later
// use ink_lang as ink;

// pub use self::uniswap_v3_factory::{
//     UniswapV3Factory,
//     UniswapV3FactoryRef,
// };

#[brush::contract]
pub mod uniswap_v3_factory {
    use ink_env::hash::{Sha2x256, HashOutput};
    use ink_lang::ToAccountId;
    use ink_storage::{Mapping, traits::{SpreadLayout, PackedLayout, StorageLayout}};
    use ink_storage::traits::SpreadAllocate;
    use scale::{Encode, Decode};

    use pool::uniswap_v3_pool::UniswapV3PoolRef;
    use primitives::{Address, Int24};
    use primitives::Uint24;
    use primitives::ADDRESS0;
    use ink_lang::codegen::Env;
    use ink_lang::codegen::EmitEvent;

    use brush::contracts::{
        ownable::*,
        psp34::*,
    };

    use brush::modifiers;

    const  ACCUMULATOR_CODE_HASH:&str = "52ea1e3471f4d4b8e41c34dfbb79db8b899a3f93be7bcb53cc16f011b81d3ffb";
    #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout,SpreadAllocate)]
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
    #[derive(Default,SpreadAllocate,PSP34Storage,OwnableStorage)]
    pub struct UniswapV3Factory {
        // pub owner:primitives::Address,
        // mapping(uint24 => int24) public override feeAmountTickSpacing;
        pub fee_amount_tick_spacing:Mapping<u32,Int24>,

        // mapping(address => mapping(address => mapping(uint24 => address))) public override getPool;
        /// @inheritdoc IPeripheryImmutableState
        pub pool_map: Mapping<(AccountId,AccountId,u32),AccountId>,
        pub parameters:Parameters,
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[OwnableStorageField]
        ownable: OwnableData,
        next_id: u8,
    }


    /// @notice Emitted when the owner of the factory is changed
    /// @param oldOwner The owner before the owner was changed
    /// @param newOwner The owner after the owner was changed
    #[ink(event)]
    pub struct OwnerChanged{
        #[ink(topic)]
        old_owner:Option<Address>, 
        #[ink(topic)]
        new_owner:Option<Address>,
    }

    /// @notice Emitted when a pool is created
    /// @param token0 The first token of the pool by address sort order
    /// @param token1 The second token of the pool by address sort order
    /// @param fee The fee collected upon every swap in the pool, denominated in hundredths of a bip
    /// @param tickSpacing The minimum number of ticks between initialized ticks
    /// @param pool The address of the created pool
    #[ink(event)]
    pub struct PoolCreated{
        #[ink(topic)]
        token0:Address,
        #[ink(topic)]
        token1:Address,
        #[ink(topic)]
        fee:Uint24,
        tick_spacing:Int24,
        pool:Address,
    }

    /// @notice Emitted when a new fee amount is enabled for pool creation via the factory
    /// @param fee The enabled fee, denominated in hundredths of a bip
    /// @param tickSpacing The minimum number of ticks between initialized ticks for pools created with the given fee
    #[ink(event)]
    pub struct FeeAmountEnabled{
        #[ink(topic)]
        fee:Uint24, 
        #[ink(topic)]
        tick_spacing:Int24,
    }

    impl PSP34 for UniswapV3Factory {}

    impl Ownable for UniswapV3Factory{}

    type Event = <UniswapV3Factory as ::ink_lang::reflect::ContractEventBase>::Type;

    impl OwnableInternal for UniswapV3Factory {
        fn _emit_ownership_transferred_event(&self, previous_owner: Option<AccountId>, new_owner: Option<AccountId>) {
            self.env().emit_event(OwnerChanged{
                old_owner:previous_owner,
                new_owner:new_owner,
            })
        }
    }

    impl UniswapV3Factory {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                instance._init_with_owner(caller);
                instance.fee_amount_tick_spacing.insert(500,&10);
                instance.env().emit_event(FeeAmountEnabled{
                    fee:500,
                    tick_spacing:10,
                });
                instance.fee_amount_tick_spacing.insert(3000,&60);
                instance.env().emit_event(FeeAmountEnabled{
                    fee:3000,
                    tick_spacing:60,
                });
                instance.fee_amount_tick_spacing.insert(10000,&200);
                instance.env().emit_event(FeeAmountEnabled{
                    fee:10000,
                    tick_spacing:200,
                });
            })
        }

        
        #[ink(message)]
        pub fn get_pool(&self,fee:u32,token0:AccountId, token1:AccountId)->AccountId{
            ink_env::debug_println!("get_pool fee is:{:?}",fee);
            let key = (token0,token1,fee);
            self.env().caller();
            self.pool_map.get(key).unwrap_or(ADDRESS0.into())
        }

        

        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            self._mint_to(Self::env().caller(), Id::U8(self.next_id))?;
            self.next_id += 1;
            Ok(())
        }

        #[ink(message)]
        pub fn get_owner(&self)->Address{
            self.ownable.owner
        }

        #[ink(message)]
        pub fn get_fee_amount_tick_spacing(&self,key:u32)->Int24{
            ink_env::debug_println!("fee_amount_tick_spacing is:{:?}",self.fee_amount_tick_spacing);
            self.fee_amount_tick_spacing.get(key).unwrap()
        }

        //此处原有modifier,限制不可以使用delegateCall的方式调用该方法,因为ink!中没有delegate call 调用,所以按时不使用NoDelegateCall
        #[ink(message)]
        pub fn create_pool(&mut self,fee:u32,token_a:Address,token_b:Address)->AccountId{
            assert!(token_a!=token_b,"token A should not equals token B");
            let (token0,token1);
            if token_a < token_b {
                token0 = token_a;
                token1 = token_b;
            }else{
                token0 = token_b;
                token1 = token_a;
            }
            ink_env::debug_println!("input fee is:{}",fee);
            let fee_amount_tick_spacing_option = self.fee_amount_tick_spacing.get(fee);
            ink_env::debug_println!("fee_amount_tick_spacing:{:?}",fee_amount_tick_spacing_option);
            let tick_spacing = fee_amount_tick_spacing_option.unwrap_or(0);
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
                    .code_hash(ink_env::Hash::try_from(hex::decode(ACCUMULATOR_CODE_HASH).unwrap().as_ref()).unwrap())
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
