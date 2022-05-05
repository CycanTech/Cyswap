#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

#[brush::contract]
pub mod position_manager {
    use brush::contracts::psp34::PSP34Data;
    // use crabswap::impls::periphery::position_manager::*;
    use brush::contracts::psp34::extensions::burnable::*;
    use brush::contracts::psp34::extensions::mintable::*;
    use crabswap::impls::erc721_permit::*;
    use crabswap::impls::periphery::PeripheryPayments::*;
    use crabswap::impls::pool_initialize::*;
    use crabswap::impls::psp34_base::*;
    use crabswap::traits::core::pool::PoolActionRef;
    use crabswap::traits::periphery::position_manager::MintParams;
    use crabswap::traits::periphery::position_manager::*;
    use crabswap::traits::periphery::LiquidityManagement::*;
    use ink_env::DefaultEnvironment;
    use ink_lang::codegen::EmitEvent;
    use ink_lang::codegen::Env;
    use ink_prelude::string::String;
    use ink_storage::Mapping;
    use libs::periphery::PoolAddress;
    use primitives::U256;
    use primitives::{Address, Int24, Uint128, Uint256, Uint80, Uint96, ADDRESS0};

    use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};

    #[cfg(feature = "std")]
    use ink_storage::traits::StorageLayout;
    use scale::Decode;
    use scale::Encode;

    #[derive(Default, Debug, Encode, Decode, SpreadAllocate, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Position {
        // the nonce for permits
        nonce: Uint96,
        // the address that is approved for spending this token
        operator: Address,
        // the ID of the pool with which this token is connected
        poolId: Uint80,
        // the tick range of the position
        tickLower: Int24,
        tickUpper: Int24,
        // the liquidity of the position
        liquidity: Uint128,
        // the fee growth of the aggregate position as of the last action on the individual position
        feeGrowthInside0LastX128: Uint256,
        feeGrowthInside1LastX128: Uint256,
        // how many uncollected tokens are owed to the position, as of the last computation
        tokensOwed0: Uint128,
        tokensOwed1: Uint128,
    }

    #[ink(storage)]
    #[derive(
        Default,
        SpreadAllocate,
        PoolInitializeStorage,
        PSP34Storage,
        ERC721PermitStorage,
        PSP34BaseStorage,
    )]
    pub struct PositionMangerContract {
        #[PoolInitializeStorageField]
        initializer: PoolInitializeData,
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[ERC721PermitStorageField]
        erc721_permit: ERC721PermitData,
        #[PSP34BaseStorageField]
        psp34_base: PSP34BaseData,

        // /// @dev IDs of pools assigned by this contract
        // mapping(address => uint80) private _poolIds;
        _poolIds: Mapping<Address, u128>,

        // /// @dev Pool keys by pool ID, to save on SSTOREs for position data
        // mapping(uint80 => PoolAddress.PoolKey) private _poolIdToPoolKey;
        _poolIdToPoolKey: Mapping<u128, PoolAddress::PoolKey>,

        // /// @dev The token ID position data
        // mapping(uint256 => Position) private _positions;
        _positions: Mapping<u128, Position>,

        // /// @dev The ID of the next token that will be minted. Skips 0
        // uint176 private _nextId = 1;
        _nextId: u128,
        // /// @dev The ID of the next pool that is used for the first time. Skips 0
        // uint80 private _nextPoolId = 1;
        _nextPoolId: u128,

        /// @dev The address of the token descriptor contract, which handles generating token URIs for position tokens
        _tokenDescriptor: Address,
    }

    impl Initializer for PositionMangerContract {}
    impl PSP34 for PositionMangerContract {}
    impl PSP34Mintable for PositionMangerContract {}
    impl PSP34Burnable for PositionMangerContract {}
    impl IERC721Permit for PositionMangerContract {}
    impl PSP34Base for PositionMangerContract {}
    // impl PositionManager for PositionMangerContract{}
    impl LiquidityManagementTrait for PositionMangerContract {}
    impl PeripheryPaymentsTrait for PositionMangerContract {}

    impl PositionMangerContract {
        #[ink(constructor, payable)]
        pub fn new(factory: AccountId, weth9: AccountId, _tokenDescriptor: AccountId) -> Self {
            // ink_env::debug_message("----------------1");
            // let initializer = PoolInitializeData {
            //     factory,
            //     WETH9: weth9,
            // };
            // ink_env::debug_message("----------------2");
            // let name = "Crabswap V3 Positions NFT-V1";
            // let symbol = "Crab-V3-POS";
            // let psp34_base = PSP34BaseData {
            //     name: String::from(name),
            //     symbol: String::from(symbol),
            // };
            // ink_env::debug_message("----------------3");
            // let erc721_permit = ERC721PermitData {
            //     nameHash: ink_lang::blake2x256!("Crabswap V3 Positions NFT-V1"),
            //     versionHash: ink_lang::blake2x256!("1"),
            // };
            // let psp34 = PSP34Data::default();
            // ink_env::debug_message("----------------4");
            // let instance: PositionMangerContract = PositionMangerContract {
            //     initializer,
            //     erc721_permit,
            //     psp34_base,
            //     _tokenDescriptor,
            //     psp34,
            //     _nextPoolId: 1,
            //     _nextId: 1,
            //     _positions: Default::default(),
            //     _poolIdToPoolKey: Default::default(),
            //     _poolIds: Default::default(),
            // };
            // ink_env::debug_message("----------------5");
            // instance
            ink_lang::codegen::initialize_contract(|instance: &mut PositionMangerContract| {
                instance.initializer.factory = factory;
                instance.initializer.WETH9 = weth9;
                let name = "Crabswap V3 Positions NFT-V1";
                let symbol = "Crab-V3-POS";
                // let version = "1";
                instance.erc721_permit.nameHash = ink_lang::blake2x256!("Crabswap V3 Positions NFT-V1");
                instance.erc721_permit.versionHash = ink_lang::blake2x256!("1");
                instance.psp34_base.name = String::from(name);
                instance.psp34_base.symbol = String::from(symbol);
                instance._tokenDescriptor = _tokenDescriptor;
                instance._nextPoolId= 1;
                instance._nextId= 1;
                instance._positions= Default::default();
                instance._poolIdToPoolKey= Default::default();
                instance._poolIds= Default::default();
            })
        }

        /// @dev Caches a pool key
        fn cachePoolKey(&mut self, pool: Address, poolKey: PoolAddress::PoolKey) -> u128 {
            let mut poolId = self._poolIds.get(&pool).unwrap();
            if poolId == 0 {
                poolId = self._nextPoolId + 1;
                self._poolIds.insert(pool, &poolId);
                self._poolIdToPoolKey.insert(poolId, &poolKey);
            }
            poolId
        }

        /// @dev Caches a pool key
        #[ink(message)]
        pub fn testEvent(&mut self) -> u128 {
            self.env().emit_event(TestEvent{tokenId:1});
            0
        }
    }

    impl PositionManager for PositionMangerContract {
        #[ink(message, payable)]
        fn mint(
            &mut self,
            params: MintParams,
        ) -> (
            u128, //tokenId
            u128, //liquidity
            U256, //amount0
            U256, //amount1
        ) {
            // IUniswapV3Pool pool;
            let pool: Address;
            // (liquidity, amount0, amount1, pool) = addLiquidity(
            //     AddLiquidityParams({
            //         token0: params.token0,
            //         token1: params.token1,
            //         fee: params.fee,
            //         recipient: address(this),
            //         tickLower: params.tickLower,
            //         tickUpper: params.tickUpper,
            //         amount0Desired: params.amount0Desired,
            //         amount1Desired: params.amount1Desired,
            //         amount0Min: params.amount0Min,
            //         amount1Min: params.amount1Min
            //     })
            // );
            let position_manager_address = ink_env::account_id::<DefaultEnvironment>();
            let addLiquidityParams = AddLiquidityParams {
                token0: params.token0,
                token1: params.token1,
                fee: params.fee,
                recipient: position_manager_address,
                tickLower: params.tickLower,
                tickUpper: params.tickUpper,
                amount0Desired: params.amount0Desired,
                amount1Desired: params.amount1Desired,
                amount0Min: params.amount0Min,
                amount1Min: params.amount1Min,
            };
            // uint128 liquidity,uint256 amount0,uint256 amount1,IUniswapV3Pool pool
            let liquidity: u128;
            let amount0: U256;
            let amount1: U256;
            
            (liquidity, amount0, amount1, pool) = self.addLiquidity(addLiquidityParams);
            let tokenId = self._nextId + 1;
            self._mint_to(params.recipient, Id::U128(tokenId)).unwrap();
            // _mint(params.recipient, (tokenId = _nextId++));

            // bytes32 positionKey = PositionKey.compute(address(this), params.tickLower, params.tickUpper);
            let address_of_this = ink_env::account_id::<DefaultEnvironment>();
            // let positionKey = PositionKey::compute(address_of_this,params.tickLower, params.tickUpper);
            // (, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128, , ) = pool.positions(positionKey);
            let position_info = PoolActionRef::positions(
                &pool,
                address_of_this,
                params.tickLower,
                params.tickUpper,
            );
            let feeGrowthInside0LastX128 = position_info.feeGrowthInside0LastX128;
            let feeGrowthInside1LastX128 = position_info.feeGrowthInside1LastX128;

            // // idempotent set
            // uint80 poolId =
            //     cachePoolKey(
            //         address(pool),
            //         PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee})
            //     );
            let pool_key = PoolAddress::PoolKey {
                token0: params.token0,
                token1: params.token1,
                fee: params.fee,
            };

            let poolId = self.cachePoolKey(address_of_this, pool_key);

            // _positions[tokenId] = Position({
            //     nonce: 0,
            //     operator: address(0),
            //     poolId: poolId,
            //     tickLower: params.tickLower,
            //     tickUpper: params.tickUpper,
            //     liquidity: liquidity,
            //     feeGrowthInside0LastX128: feeGrowthInside0LastX128,
            //     feeGrowthInside1LastX128: feeGrowthInside1LastX128,
            //     tokensOwed0: 0,
            //     tokensOwed1: 0
            // });
            let position = Position {
                nonce: 0,
                operator: ADDRESS0.into(),
                poolId: poolId,
                tickLower: params.tickLower,
                tickUpper: params.tickUpper,
                liquidity: liquidity,
                feeGrowthInside0LastX128: feeGrowthInside0LastX128,
                feeGrowthInside1LastX128: feeGrowthInside1LastX128,
                tokensOwed0: 0,
                tokensOwed1: 0,
            };
            self._positions.insert(tokenId, &position);
            self.env().emit_event(IncreaseLiquidity {
                tokenId,
                liquidity,
                amount0,
                amount1,
            });
            // emit IncreaseLiquidity(tokenId, liquidity, amount0, amount1);
            (tokenId, liquidity, amount0, amount1)
        }
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    /// @notice Emitted when liquidity is increased for a position NFT
    /// @dev Also emitted when a token is minted
    /// @param tokenId The ID of the token for which liquidity was increased
    /// @param liquidity The amount by which liquidity for the NFT position was increased
    /// @param amount0 The amount of token0 that was paid for the increase in liquidity
    /// @param amount1 The amount of token1 that was paid for the increase in liquidity
    // event IncreaseLiquidity(uint256 indexed tokenId, uint128 liquidity, uint256 amount0, uint256 amount1);
    #[ink(event)]
    pub struct IncreaseLiquidity {
        #[ink(topic)]
        tokenId: u128,
        #[ink(topic)]
        liquidity: u128,
        #[ink(topic)]
        amount0: U256,
        #[ink(topic)]
        amount1: U256,
    }

    #[ink(event)]
    pub struct TestEvent {
        #[ink(topic)]
        tokenId: u128,
    }

    impl PSP34Internal for PositionMangerContract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        fn _emit_approval_event(
            &self,
            from: AccountId,
            to: AccountId,
            id: Option<Id>,
            approved: bool,
        ) {
            self.env().emit_event(Approval {
                from,
                to,
                id,
                approved,
            });
        }
    }
}
