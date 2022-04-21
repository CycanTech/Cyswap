#![allow(non_snake_case)]
use ink_storage::traits::{SpreadLayout, PackedLayout, SpreadAllocate};
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use scale::{Encode, Decode};
use primitives::{Uint160, Uint16, Address, Uint24, U160};
use primitives::Int24;
use primitives::Uint8;


#[brush::wrapper]
pub type PoolRef = dyn Pool;

#[derive(Default,Debug,Clone, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout,SpreadAllocate)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Slot0 {
    // the current price
    pub sqrtPriceX96: Uint160,
    // the current tick
    pub tick: Int24,
    // the most-recently updated index of the observations array
    pub observationIndex: Uint16,
    // the current maximum number of observations that are being stored
    pub observationCardinality: Uint16,
    // the next maximum number of observations to store, triggered in observations.write
    pub observationCardinalityNext: Uint16,
    // the current protocol fee as a percentage of the swap fee taken on withdrawal
    // represented as an integer denominator (1/x)%
    pub feeProtocol: Uint8,
    // whether the pool is locked
    pub unlocked: bool,
}

#[brush::trait_definition]
pub trait Pool{

    // fn new(factory:Address,token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24) -> Self;
    /// @inheritdoc IUniswapV3PoolActions
    /// @dev not locked because it initializes unlocked
    #[ink(message, payable)]
    fn initialize(&mut self,sqrtPriceX96:U160);

    #[ink(message)]
    fn slot0(&self)->Slot0;
}