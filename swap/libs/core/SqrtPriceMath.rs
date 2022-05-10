#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use primitives::{Int256, Uint160, U160, U256};

use crate::core::UnsafeMath;
use crate::swap::{FixedPoint96, FullMath};

/// @notice Helper that gets signed token0 delta
/// @param sqrtRatioAX96 A sqrt price
/// @param sqrtRatioBX96 Another sqrt price
/// @param liquidity The change in liquidity for which to compute the amount0 delta
/// @return amount0 Amount of token0 corresponding to the passed liquidityDelta between the two prices
pub fn getAmount0Delta(sqrtRatioAX96: U160, sqrtRatioBX96: U160, liquidity: i128) -> Int256 {
    // return
    //     liquidity < 0
    //         ? -getAmount0Delta(sqrtRatioAX96, sqrtRatioBX96, uint128(-liquidity), false).toInt256()
    //         : getAmount0Delta(sqrtRatioAX96, sqrtRatioBX96, uint128(liquidity), true).toInt256();
    let result: i128;
    if liquidity < 0 {
        result = -i128::try_from(
            getAmount0DeltaWithRound(
                sqrtRatioAX96,
                sqrtRatioBX96,
                u128::try_from(-liquidity).unwrap(),
                false,
            )
            .as_u128(),
        )
        .unwrap();
    } else {
        result = i128::try_from(
            getAmount0DeltaWithRound(
                sqrtRatioAX96,
                sqrtRatioBX96,
                u128::try_from(liquidity).unwrap(),
                false,
            )
            .as_u128(),
        )
        .unwrap();
    }
    result
}

/// @notice Gets the amount0 delta between two prices
/// @dev Calculates liquidity / sqrt(lower) - liquidity / sqrt(upper),
/// i.e. liquidity * (sqrt(upper) - sqrt(lower)) / (sqrt(upper) * sqrt(lower))
/// @param sqrtRatioAX96 A sqrt price
/// @param sqrtRatioBX96 Another sqrt price
/// @param liquidity The amount of usable liquidity
/// @param roundUp Whether to round the amount up or down
/// @return amount0 Amount of token0 required to cover a position of size liquidity between the two passed prices
pub fn getAmount0DeltaWithRound(
    mut sqrtRatioAX96: U256,
    mut sqrtRatioBX96: U256,
    liquidity: u128,
    roundUp: bool,
) -> U256 {
    //amount0
    // if (sqrtRatioAX96 > sqrtRatioBX96) (sqrtRatioAX96, sqrtRatioBX96) = (sqrtRatioBX96, sqrtRatioAX96);
    if sqrtRatioAX96 > sqrtRatioBX96 {
        let temp = sqrtRatioAX96;
        sqrtRatioAX96 = sqrtRatioBX96;
        sqrtRatioBX96 = temp;
    }

    let numerator1: U256 = U256::from(liquidity) << FixedPoint96::RESOLUTION;
    let numerator2: U256 = sqrtRatioBX96 - sqrtRatioAX96;

    assert!(
        sqrtRatioAX96 > U256::zero(),
        "sqrtRatioAX96 must big than 0"
    );

    // return
    //     roundUp
    //         ? UnsafeMath.divRoundingUp(
    //             FullMath.mulDivRoundingUp(numerator1, numerator2, sqrtRatioBX96),
    //             sqrtRatioAX96
    //         )
    //         : FullMath.mulDiv(numerator1, numerator2, sqrtRatioBX96) / sqrtRatioAX96;
    if roundUp {
        UnsafeMath::divRoundingUp(
            FullMath::mulDivRoundingUp(numerator1, numerator2, sqrtRatioBX96),
            sqrtRatioAX96,
        )
    } else {
        FullMath::mulDiv(numerator1, numerator2, sqrtRatioBX96) / sqrtRatioAX96
    }
}

/// @notice Helper that gets signed token1 delta
/// @param sqrtRatioAX96 A sqrt price
/// @param sqrtRatioBX96 Another sqrt price
/// @param liquidity The change in liquidity for which to compute the amount1 delta
/// @return amount1 Amount of token1 corresponding to the passed liquidityDelta between the two prices
pub fn getAmount1Delta(sqrtRatioAX96: U160, sqrtRatioBX96: U160, liquidity: i128) -> Int256 {
    // return
    //     liquidity < 0
    //         ? -getAmount1Delta(sqrtRatioAX96, sqrtRatioBX96, uint128(-liquidity), false).toInt256()
    //         : getAmount1Delta(sqrtRatioAX96, sqrtRatioBX96, uint128(liquidity), true).toInt256();
    let result: i128;
    if liquidity < 0 {
        result = -i128::try_from(
            getAmount1DeltaWithRound(
                sqrtRatioAX96,
                sqrtRatioBX96,
                u128::try_from(-liquidity).unwrap(),
                false,
            )
            .as_u128(),
        )
        .unwrap();
    } else {
        result = i128::try_from(
            getAmount1DeltaWithRound(
                sqrtRatioAX96,
                sqrtRatioBX96,
                u128::try_from(liquidity).unwrap(),
                true,
            )
            .as_u128(),
        )
        .unwrap();
    }
    result
}

/// @notice Gets the amount1 delta between two prices
/// @dev Calculates liquidity * (sqrt(upper) - sqrt(lower))
/// @param sqrtRatioAX96 A sqrt price
/// @param sqrtRatioBX96 Another sqrt price
/// @param liquidity The amount of usable liquidity
/// @param roundUp Whether to round the amount up, or down
/// @return amount1 Amount of token1 required to cover a position of size liquidity between the two passed prices
pub fn getAmount1DeltaWithRound(
    mut sqrtRatioAX96: U160,
    mut sqrtRatioBX96: U160,
    liquidity: u128,
    roundUp: bool,
) -> U256 {
    // if (sqrtRatioAX96 > sqrtRatioBX96) (sqrtRatioAX96, sqrtRatioBX96) = (sqrtRatioBX96, sqrtRatioAX96);
    if sqrtRatioAX96 > sqrtRatioBX96 {
        let temp = sqrtRatioAX96;
        sqrtRatioAX96 = sqrtRatioBX96;
        sqrtRatioBX96 = temp;
    }

    // return
    //     roundUp
    //         ? FullMath.mulDivRoundingUp(liquidity, sqrtRatioBX96 - sqrtRatioAX96, FixedPoint96.Q96)
    //         : FullMath.mulDiv(liquidity, sqrtRatioBX96 - sqrtRatioAX96, FixedPoint96.Q96);

    if roundUp {
        FullMath::mulDivRoundingUp(
            U256::from(liquidity),
            sqrtRatioBX96 - sqrtRatioAX96,
            U256::from(FixedPoint96::Q96),
        )
    } else {
        FullMath::mulDiv(
            U256::from(liquidity),
            sqrtRatioBX96 - sqrtRatioAX96,
            U256::from(FixedPoint96::Q96),
        )
    }
}
