use primitives::U256;
use crate::assembly::{add,div,gt,div_mod};
/// @title Math functions that do not check inputs or outputs
/// @notice Contains methods that perform common math functions but do not do any overflow or underflow checks
    /// @notice Returns ceil(x / y)
    /// @dev division by 0 has unspecified behavior, and must be checked externally
    /// @param x The dividend
    /// @param y The divisor
    /// @return z The quotient, ceil(x / y)
    pub fn divRoundingUp(x:U256, y:U256) -> U256 {
        // z := add(div(x, y), gt(mod(x, y), 0))
          let  z = add(&div(&x, &y), &gt(&div_mod(&x, &y), &U256::zero()));
          z
    }