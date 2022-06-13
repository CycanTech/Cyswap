#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

/// @notice Add a signed liquidity delta to liquidity and revert if it overflows or underflows
/// @param x The liquidity before change
/// @param y The delta by which liquidity should be changed
/// @return z The liquidity delta
pub fn addDelta(x: u128, y: i128) -> u128 {
    let z;
    // if (y < 0) {
    //     require((z = x - uint128(-y)) < x, 'LS');
    // } else {
    //     require((z = x + uint128(y)) >= x, 'LA');
    // }

    if y < 0 {
        z = x - u128::try_from(-y).unwrap();
        assert!(z < x, "LS");
    } else {
        z = x + u128::try_from(y).unwrap();
        assert!(z >= x, "LA");
    }
    z
}
