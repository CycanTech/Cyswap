use ink_storage::traits::{SpreadLayout, PackedLayout, SpreadAllocate,StorageLayout};
use primitives::{Address, Int24, Uint24};
use scale::{Encode, Decode};


#[brush::wrapper]
pub type PoolDeployerRef = dyn PoolDeployer;

pub const  ACCUMULATOR_CODE_HASH:&str = "52ea1e3471f4d4b8e41c34dfbb79db8b899a3f93be7bcb53cc16f011b81d3ffb";

#[derive(Default,Debug,Clone, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout,SpreadAllocate)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct Parameters {
    pub factory:Address,
    pub token0:Address,
    pub token1:Address,
    pub fee:Uint24,
    pub tick_spacing:Int24,
}


/// @title An interface for a contract that is capable of deploying Uniswap V3 Pools
/// @notice A contract that constructs a pool must implement this to pass arguments to the pool
/// @dev This is used to avoid having constructor arguments in the pool contract, which results in the init code hash
/// of the pool being constant allowing the CREATE2 address of the pool to be cheaply computed on-chain
#[brush::trait_definition]
pub trait PoolDeployer {
    /// @notice Get the parameters to be used in constructing the pool, set transiently during pool creation.
    /// @dev Called by the pool constructor to fetch the parameters of the pool
    /// Returns factory The factory address
    /// Returns token0 The first token of the pool by address sort order
    /// Returns token1 The second token of the pool by address sort order
    /// Returns fee The fee collected upon every swap in the pool, denominated in hundredths of a bip
    /// Returns tickSpacing The minimum number of ticks between initialized ticks
    #[ink(message)]
    fn parameters(
        &self,
    ) -> Parameters;

    // /// @dev Deploys a pool with the given parameters by transiently setting the parameters storage slot and then
    // /// clearing it after deploying the pool.
    // /// @param factory The contract address of the Uniswap V3 factory
    // /// @param token0 The first token of the pool by address sort order
    // /// @param token1 The second token of the pool by address sort order
    // /// @param fee The fee collected upon every swap in the pool, denominated in hundredths of a bip
    // /// @param tickSpacing The spacing between usable ticks
    // #[ink(message)]
    // fn deploy(
    //     &mut self,
    //     factory: Address,
    //     token0: Address,
    //     token1: Address,
    //     fee: Uint24,
    //     tick_spacing: Int24,
    // ) -> Address;
}
