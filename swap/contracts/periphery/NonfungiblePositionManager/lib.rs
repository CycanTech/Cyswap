#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(non_snake_case)]

#[brush::contract]
pub mod position_manager {
    use brush::contracts::psp34::PSP34Data;
    // use crabswap::impls::periphery::position_manager::*;
    use brush::contracts::psp34::extensions::burnable::*;
    use brush::contracts::psp34::extensions::mintable::*;
    use brush::modifiers;
    use crabswap::impls::erc721_permit::*;
    use crabswap::impls::periphery::PeripheryPayments::*;
    use crabswap::impls::pool_initialize::*;
    use crabswap::impls::psp34_base::*;
    use crabswap::traits::core::pool::PoolActionRef;
    use crabswap::traits::periphery::position_manager::*;
    use crabswap::traits::periphery::LiquidityManagement::*;

    use ink_lang::codegen::EmitEvent;
    use ink_lang::codegen::Env;
    use ink_prelude::string::String;
    use ink_storage::Mapping;
    use libs::core::FixedPoint128;
    use libs::periphery::PoolAddress;
    use libs::core::Position::Info;
    use libs::swap::FullMath;
    use primitives::{Int24, Uint128, Uint24, Uint256, Uint80, Uint96, ADDRESS0};

    use ink_storage::traits::PackedLayout;

    #[cfg(feature = "std")]
    use ink_storage::traits::StorageLayout;

    use crabswap::traits::core::factory::*;
    use crabswap::traits::periphery::position_descriptor::*;
    use ink_env::DefaultEnvironment;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::{SpreadAllocate, SpreadLayout};
    use libs::core::TickMath;
    use libs::periphery::LiquidityAmounts;
    use libs::PoolKey;
    use primitives::{Address, U256};
    use scale::{Decode, Encode};

    use crabswap::impls::pool_initialize::PoolInitializeStorage;
    use crabswap::traits::core::pool::*;
    use ink_env::CallFlags;

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

    #[derive(Default, Decode, Encode, Debug, SpreadAllocate, SpreadLayout)]
    struct MintCallbackData {
        poolKey: PoolKey,
        payer: Address,
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
    impl LiquidityManagementTrait for PositionMangerContract {
        // returns (uint128 liquidity,uint256 amount0,uint256 amount1,IUniswapV3Pool pool)
        #[ink(message)]
        fn addLiquidity(&mut self, params: AddLiquidityParams) -> (u128, U256, U256, Address) {
            // PoolAddress.PoolKey memory poolKey =
            //         PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee});
            let poolKey: PoolKey = PoolKey {
                token0: params.token0,
                token1: params.token1,
                fee: params.fee,
            };

            // pool = IUniswapV3Pool(PoolAddress.computeAddress(factory, poolKey));
            let factory = self.initializer.factory;
            // let poolAddress = PoolAddress::computeAddress(factory, poolKey.clone());
            let poolAddress =
                FactoryRef::get_pool(&factory, params.fee, params.token0, params.token1);

            //         // compute the liquidity amount
            //         {
            //             // (uint160 sqrtPriceX96, , , , , , ) = poolRef::slot0(poolAddress);
            let slot0: Slot0 = PoolActionRef::getSlot0(&poolAddress);
            let sqrtPriceX96 = slot0.sqrtPriceX96.value;
            //             // uint160 sqrtRatioAX96 = TickMath.getSqrtRatioAtTick(params.tickLower);
            let sqrtRatioAX96 = TickMath::getSqrtRatioAtTick(params.tickLower);
            //             // uint160 sqrtRatioBX96 = TickMath.getSqrtRatioAtTick(params.tickUpper);
            let sqrtRatioBX96 = TickMath::getSqrtRatioAtTick(params.tickUpper);

            //             liquidity = LiquidityAmounts::getLiquidityForAmounts(
            //                 sqrtPriceX96,
            //                 sqrtRatioAX96,
            //                 sqrtRatioBX96,
            //                 params.amount0Desired,
            //                 params.amount1Desired
            //             );
            //         }
            ink_env::debug_println!(
                "sqrtPriceX96 is:{},sqrtRatioAX96 is:{},sqrtRatioBX96 is:{}",
                sqrtPriceX96,
                sqrtRatioAX96,
                sqrtRatioBX96
            );
            let liquidity = LiquidityAmounts::getLiquidityForAmounts(
                sqrtPriceX96,
                sqrtRatioAX96,
                sqrtRatioBX96,
                params.amount0Desired.value,
                params.amount1Desired.value,
            );

            //         (amount0, amount1) = pool.mint(
            //             params.recipient,
            //             params.tickLower,
            //             params.tickUpper,
            //             liquidity,
            //             abi.encode(MintCallbackData({poolKey: poolKey, payer: msg.sender}))
            //         );
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            let mint_callback_data = MintCallbackData {
                poolKey: poolKey,
                payer: msg_sender,
            };

            let callback_data = scale::Encode::encode(&mint_callback_data);
            let (amount0, amount1) = PoolActionRef::mint_builder(
                &poolAddress,
                params.recipient,
                params.tickLower,
                params.tickUpper,
                liquidity,
                callback_data.clone(),
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap();
            // self.uniswapV3MintCallback(amount0, amount1, callback_data);
            //         require(amount0 >= params.amount0Min && amount1 >= params.amount1Min, 'Price slippage check');
            assert!(
                amount0 >= params.amount0Min.value && amount1 >= params.amount1Min.value,
                "Price slippage check"
            );
            return (liquidity, amount0, amount1, poolAddress);
        }

        #[ink(message)]
        fn uniswapV3MintCallback(&mut self, amount0Owed: U256, amount1Owed: U256, data: Vec<u8>) {
            let manager_address: brush::traits::AccountId =
                ink_env::account_id::<DefaultEnvironment>();
            let msg_sender = ink_env::caller::<DefaultEnvironment>();
            // MintCallbackData memory deceoded = abi.decode(data, (MintCallbackData));
            let decoded: MintCallbackData =
                scale::Decode::decode(&mut data.as_ref()).expect("call back data parse error!");
            // TODO add callback validation
            // CallbackValidation.verifyCallback(factory, decoded.poolKey);

            // if (amount0Owed > 0) pay(decoded.poolKey.token0, decoded.payer, msg.sender, amount0Owed);
            if amount0Owed > U256::from(0) {
                PeripheryPaymentsTraitRef::pay_builder(
                    &manager_address,
                    decoded.poolKey.token0,
                    decoded.payer,
                    msg_sender,
                    amount0Owed,
                )
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap();
            }
            // if (amount1Owed > 0) pay(decoded.poolKey.token1, decoded.payer, msg.sender, amount1Owed);
            if amount1Owed > U256::from(0) {
                PeripheryPaymentsTraitRef::pay_builder(
                    &manager_address,
                    decoded.poolKey.token1,
                    decoded.payer,
                    msg_sender,
                    amount1Owed,
                )
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap();
            }
        }

    }

    impl PeripheryPaymentsTrait for PositionMangerContract {}

    // modifier isAuthorizedForToken(uint256 tokenId) {
    //     require(_isApprovedOrOwner(msg.sender, tokenId), 'Not approved');
    //     _;
    // }

    /// Throws if called by any account other than the owner.
    // #[modifier_definition]
    // pub fn only_owner<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
    // where
    //     T: OwnableStorage,
    //     F: FnOnce(&mut T) -> Result<R, E>,
    //     E: From<OwnableError>,
    // {
    //     if instance.get().owner != T::env().caller() {
    //         return Err(From::from(OwnableError::CallerIsNotOwner))
    //     }
    //     body(instance)
    // }

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
                instance.erc721_permit.nameHash =
                    ink_lang::blake2x256!("Crabswap V3 Positions NFT-V1");
                instance.erc721_permit.versionHash = ink_lang::blake2x256!("1");
                instance.psp34_base.name = String::from(name);
                instance.psp34_base.symbol = String::from(symbol);
                instance._tokenDescriptor = _tokenDescriptor;
                instance._nextPoolId = 0;
                instance._nextId = 0;
                instance._positions = Default::default();
                instance._poolIdToPoolKey = Default::default();
                instance._poolIds = Default::default();
            })
        }

        /// @dev Caches a pool key
        fn cachePoolKey(&mut self, pool: Address, poolKey: PoolAddress::PoolKey) -> u128 {
            let mut poolId = match self._poolIds.get(&pool) {
                Some(id) => id,
                None => 0,
            };
            if poolId == 0 {
                self._nextPoolId = self._nextPoolId + 1;
                poolId = self._nextPoolId;
                self._poolIds.insert(pool, &poolId);
                self._poolIdToPoolKey.insert(poolId, &poolKey);
            }
            poolId
        }

        /// @dev Caches a pool key
        #[ink(message)]
        pub fn testEvent(&mut self) -> u128 {
            self.env().emit_event(TestEvent { tokenId: 1 });
            0
        }
    }

    impl PositionManager for PositionMangerContract {

        #[ink(message, payable)]
        #[modifiers(checkDeadline(deadline))]
        fn increaseLiquidity(&mut self, tokenId: u128,
            amount0Desired: U256,
            amount1Desired: U256,
            amount0Min: U256,
            amount1Min: U256,
            deadline: u64,) -> (
            u128 //liquidity
            , U256//amount0
            , U256//amount1
        ){
            // Position storage position = _positions[params.tokenId];
            let params = IncreaseLiquidityParams{
                tokenId,
            amount0Desired,
            amount1Desired,
            amount0Min,
            amount1Min,
            deadline,
            };
            let mut position:Position = self._positions.get(params.tokenId).expect("token not in _positions!");
            // PoolAddress.PoolKey memory poolKey = _poolIdToPoolKey[position.poolId];
            let poolKey:PoolAddress::PoolKey = self._poolIdToPoolKey.get(position.poolId).expect("token not in _poolIdToPoolKey!");
            // IUniswapV3Pool pool;
            
            // (liquidity, amount0, amount1, pool) = addLiquidity(
            //     AddLiquidityParams({
            //         token0: poolKey.token0,
            //         token1: poolKey.token1,
            //         fee: poolKey.fee,
            //         tickLower: position.tickLower,
            //         tickUpper: position.tickUpper,
            //         amount0Desired: params.amount0Desired,
            //         amount1Desired: params.amount1Desired,
            //         amount0Min: params.amount0Min,
            //         amount1Min: params.amount1Min,
            //         recipient: address(this)
            //     })
            // );
            let address_this = ink_env::account_id::<DefaultEnvironment>();
            let (liquidity, amount0, amount1, pool) = self.addLiquidity(
                AddLiquidityParams{
                    token0: poolKey.token0,
                    token1: poolKey.token1,
                    fee: poolKey.fee,
                    tickLower: position.tickLower,
                    tickUpper: position.tickUpper,
                    amount0Desired: Uint256::new_with_u256(params.amount0Desired),
                    amount1Desired: Uint256::new_with_u256(params.amount1Desired),
                    amount0Min: Uint256::new_with_u256(params.amount0Min),
                    amount1Min: Uint256::new_with_u256(params.amount1Min),
                    recipient: address_this
                }
            );

            // bytes32 positionKey = PositionKey.compute(address(this), position.tickLower, position.tickUpper);
            // let positionKey = PositionKey::compute(address_this, position.tickLower, position.tickUpper);

            // // this is now updated to the current transaction
            // (, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128, , ) = pool.positions(positionKey);
            let positionInfo:Info = PoolActionRef::positions(&pool,address_this, position.tickLower, position.tickUpper);
            let feeGrowthInside0LastX128 = positionInfo.feeGrowthInside0LastX128;
            let feeGrowthInside1LastX128 = positionInfo.feeGrowthInside1LastX128;
            // position.tokensOwed0 += uint128(
            //     FullMath.mulDiv(
            //         feeGrowthInside0LastX128 - position.feeGrowthInside0LastX128,
            //         position.liquidity,
            //         FixedPoint128.Q128
            //     )
            // );
            position.tokensOwed0 = position.tokensOwed0 + FullMath::mulDiv(feeGrowthInside0LastX128.value-position.feeGrowthInside0LastX128.value
                ,U256::from(position.liquidity)
                ,U256::from(FixedPoint128::Q128)).as_u128();
            // position.tokensOwed1 += uint128(
            //     FullMath.mulDiv(
            //         feeGrowthInside1LastX128 - position.feeGrowthInside1LastX128,
            //         position.liquidity,
            //         FixedPoint128.Q128
            //     )
            // );
            position.tokensOwed1 = position.tokensOwed1 + FullMath::mulDiv(feeGrowthInside1LastX128.value-position.feeGrowthInside1LastX128.value
                ,U256::from(position.liquidity)
                ,U256::from(FixedPoint128::Q128)).as_u128();
            // position.feeGrowthInside0LastX128 = feeGrowthInside0LastX128;
            position.feeGrowthInside0LastX128 = feeGrowthInside0LastX128;
            // position.feeGrowthInside1LastX128 = feeGrowthInside1LastX128;
            position.feeGrowthInside1LastX128 = feeGrowthInside1LastX128;
            // position.liquidity += liquidity;
            position.liquidity += liquidity;
            self._positions.insert(params.tokenId,&position);
            // emit IncreaseLiquidity(params.tokenId, liquidity, amount0, amount1);
            self.env().emit_event(IncreaseLiquidity {
                tokenId:params.tokenId,
                liquidity,
                amount0,
                amount1,
            });
            (liquidity,amount0,amount1)
        }

        #[ink(message)]
        fn tokenURI(&self,tokenId:u128)-> String {
            assert!(self._check_token_exists(&Id::U128(tokenId)).is_ok());
            let manager_address = ink_env::account_id::<DefaultEnvironment>();
            return DescriptorRef::tokenURI(&self._tokenDescriptor,manager_address,tokenId);
        }

        /**
         * @dev Returns whether `spender` is allowed to manage `tokenId`.
         *
         * Requirements:
         *
         * - `tokenId` must exist.
         */
        fn _isApprovedOrOwner(&self, spender: Address, tokenId: u128) -> bool {
            // require(_exists(tokenId), "ERC721: operator query for nonexistent token");
            // address owner = ERC721.ownerOf(tokenId);
            // return (spender == owner || getApproved(tokenId) == spender || ERC721.isApprovedForAll(owner, spender));
            let tokenId: Id = Id::U128(tokenId);
            assert!(
                self._check_token_exists(&tokenId).is_ok(),
                "ERC721: operator query for nonexistent token"
            );
            let owner: Address = self.owner_of(tokenId.clone()).unwrap();
            spender == owner || self._allowance(&owner, &spender, &Some(tokenId.clone()))
        }

        #[ink(message)]
        fn positions(
            &self,
            tokenId: u128,
        ) -> (
            Uint96,
            Address,
            Address,
            Address,
            Uint24,
            Int24,
            Int24,
            u128,
            U256,
            U256,
            u128,
            u128,
        ) {
            //Position memory position = _positions[tokenId];
            // require(position.poolId != 0, 'Invalid token ID');
            // PoolAddress.PoolKey memory poolKey = _poolIdToPoolKey[position.poolId];
            let position: Position = self._positions.get(tokenId).unwrap_or(Default::default());
            assert!(position.poolId != 0, "Invalid token ID");
            let poolKey: PoolAddress::PoolKey = self._poolIdToPoolKey.get(position.poolId).unwrap();
            return (
                position.nonce,
                position.operator,
                poolKey.token0,
                poolKey.token1,
                poolKey.fee,
                position.tickLower,
                position.tickUpper,
                position.liquidity,
                position.feeGrowthInside0LastX128.value,
                position.feeGrowthInside1LastX128.value,
                position.tokensOwed0,
                position.tokensOwed1,
            );
        }

        // pub token0: Address,
        // pub token1: Address,
        // pub fee: Uint24,
        // pub tickLower: Int24,
        // pub tickUpper: Int24,
        // pub amount0Desired: Uint256,
        // pub amount1Desired: Uint256,
        // pub amount0Min: Uint256,
        // pub amount1Min: Uint256,
        // pub recipient: Address,
        // pub deadline: Uint256,
        #[ink(message, payable)]
        fn mint(
            &mut self,
            token0: Address,
            token1: Address,
            fee: Uint24,
            tickLower: Int24,
            tickUpper: Int24,
            amount0Desired: U256,
            amount1Desired: U256,
            amount0Min: U256,
            amount1Min: U256,
            recipient: Address,
            deadline: U256,
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
                token0: token0,
                token1: token1,
                fee: fee,
                recipient: position_manager_address,
                tickLower: tickLower,
                tickUpper: tickUpper,
                amount0Desired: Uint256::new_with_u256(amount0Desired),
                amount1Desired: Uint256::new_with_u256(amount1Desired),
                amount0Min: Uint256::new_with_u256(amount0Min),
                amount1Min: Uint256::new_with_u256(amount1Min),
            };
            // uint128 liquidity,uint256 amount0,uint256 amount1,IUniswapV3Pool pool
            let liquidity: u128;
            let amount0: U256;
            let amount1: U256;

            (liquidity, amount0, amount1, pool) = self.addLiquidity(addLiquidityParams);
            ink_env::debug_println!(
                "liquidity:{:?}, amount0:{:?}, amount1:{:?}",
                liquidity,
                amount0,
                amount1
            );
            self._nextId = self._nextId + 1;
            let tokenId = self._nextId;
            self._mint_to(recipient, Id::U128(tokenId)).unwrap();
            // _mint(params.recipient, (tokenId = _nextId++));
            // bytes32 positionKey = PositionKey.compute(address(this), params.tickLower, params.tickUpper);
            // let positionKey = PositionKey::compute(address_of_this,params.tickLower, params.tickUpper);
            // (, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128, , ) = pool.positions(positionKey);
            let position_info =
                PoolActionRef::positions(&pool, position_manager_address, tickLower, tickUpper);
            let feeGrowthInside0LastX128 = position_info.feeGrowthInside0LastX128;
            let feeGrowthInside1LastX128 = position_info.feeGrowthInside1LastX128;

            // // idempotent set
            // uint80 poolId =
            //     cachePoolKey(
            //         address(pool),
            //         PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee})
            //     );
            let pool_key = PoolAddress::PoolKey {
                token0: token0,
                token1: token1,
                fee: fee,
            };
            let poolId = self.cachePoolKey(position_manager_address, pool_key);

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
                tickLower: tickLower,
                tickUpper: tickUpper,
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
            // ink_lang::codegen::EmitEvent::<PositionMangerContract>::emit_event(self.env(), IncreaseLiquidity {
            //         tokenId,
            //         liquidity,
            //         amount0,
            //         amount1,
            //     });
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
        liquidity: u128,
        amount0: U256,
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
