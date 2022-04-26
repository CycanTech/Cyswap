use primitives::U256;

/// @notice Calculates floor(a×b÷denominator) with full precision. Throws if result overflows a uint256 or denominator == 0
/// @param a The multiplicand
/// @param b The multiplier
/// @param denominator The divisor
/// @return result The 256-bit result
/// @dev Credit to Remco Bloemen under MIT license https://xn--2-umb.com/21/muldiv
pub fn mulDiv(a: U256, b: U256, denominator: U256) -> U256 {
    a.saturating_mul(b).checked_div(denominator).unwrap()
}
