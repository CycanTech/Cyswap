use ink_env::DefaultEnvironment;
use primitives::{Address, Uint24};
use crabswap::traits::core::factory::FactoryRef;

/// @notice Returns the address of a valid Uniswap V3 Pool
    /// @param factory The contract address of the Uniswap V3 factory
    /// @param tokenA The contract address of either token0 or token1
    /// @param tokenB The contract address of the other token
    /// @param fee The fee collected upon every swap in the pool, denominated in hundredths of a bip
    /// @return pool The V3 pool contract address
    pub fn verifyCallback(
         factory:Address,
         tokenA:Address,
         tokenB:Address,
         fee:Uint24
    ) -> Address {
        // pool = IUniswapV3Pool(PoolAddress.computeAddress(factory, poolKey));
        // require(msg.sender == address(pool));
        let pool:Address = FactoryRef::get_pool(&factory, fee, tokenA, tokenB);
        let msg_sender = ink_env::caller::<DefaultEnvironment>();
        assert!(msg_sender == (pool));
        pool
    }
