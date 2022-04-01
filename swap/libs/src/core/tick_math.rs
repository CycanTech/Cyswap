#![cfg_attr(not(feature = "std"), no_std)]

use primitives::Int24;

/// @dev The minimum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**-128
pub const MIN_TICK:Int24 = -887272;
    /// @dev The maximum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**128
pub const MAX_TICK:Int24 = -MIN_TICK;