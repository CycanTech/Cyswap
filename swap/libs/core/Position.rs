#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]
use primitives::Uint256;


/// @title Position
/// @notice Positions represent an owner address' liquidity between a lower and upper tick boundary
/// @dev Positions store additional state for tracking fees owed to the position

// info stored for each user's position
pub struct Info {
    // the amount of liquidity owned by this position
    pub liquidity:u128,
    // fee growth per unit of liquidity as of the last update to liquidity or fees owed
    pub feeGrowthInside0LastX128:Uint256,
    pub feeGrowthInside1LastX128:Uint256,
    // the fees owed to the position owner in token0/token1
    pub tokensOwed0:u128,
    pub tokensOwed1:u128,
}