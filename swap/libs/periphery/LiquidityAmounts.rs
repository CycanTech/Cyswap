#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use openbrush::traits::Hash;
use primitives::{U160, U256};

use crate::swap::{FixedPoint96, FullMath};
/// @notice Computes the amount of token1 for a given amount of liquidity and a price range
/// @param sqrtRatioAX96 A sqrt price representing the first tick boundary
/// @param sqrtRatioBX96 A sqrt price representing the second tick boundary
/// @param liquidity The liquidity being valued
/// @return amount1 The amount of token1
pub fn getAmount1ForLiquidity(
    mut sqrtRatioAX96: U160,
    mut sqrtRatioBX96: U160,
    liquidity: u128,
) -> U256 {
    // if (sqrtRatioAX96 > sqrtRatioBX96) (sqrtRatioAX96, sqrtRatioBX96) = (sqrtRatioBX96, sqrtRatioAX96);
    (sqrtRatioAX96, sqrtRatioBX96) = arrangeToken(sqrtRatioAX96, sqrtRatioBX96);
    // return FullMath.mulDiv(liquidity, sqrtRatioBX96 - sqrtRatioAX96, FixedPoint96.Q96);
    let amount1 = FullMath::mulDiv(
        U256::from(liquidity),
        sqrtRatioBX96 - sqrtRatioAX96,
        U256::from(FixedPoint96::Q96),
    );
    amount1
}

/// @notice Computes the maximum amount of liquidity received for a given amount of token0, token1, the current
/// pool prices and the prices at the tick boundaries
/// @param sqrtRatioX96 A sqrt price representing the current pool prices
/// @param sqrtRatioAX96 A sqrt price representing the first tick boundary
/// @param sqrtRatioBX96 A sqrt price representing the second tick boundary
/// @param amount0 The amount of token0 being sent in
/// @param amount1 The amount of token1 being sent in
/// @return liquidity The maximum amount of liquidity received
pub fn getLiquidityForAmounts(
    sqrtRatioX96: U160,
    mut sqrtRatioAX96: U160,
    mut sqrtRatioBX96: U160,
    amount0: U256,
    amount1: U256,
) -> u128 {
    let liquidity: u128;
    // if (sqrtRatioAX96 > sqrtRatioBX96) (sqrtRatioAX96, sqrtRatioBX96) = (sqrtRatioBX96, sqrtRatioAX96);
    (sqrtRatioAX96, sqrtRatioBX96) = arrangeToken(sqrtRatioAX96, sqrtRatioBX96);
    // if (sqrtRatioX96 <= sqrtRatioAX96) {
    if sqrtRatioX96 <= sqrtRatioAX96 {
        // liquidity = getLiquidityForAmount0(sqrtRatioAX96, sqrtRatioBX96, amount0);
        liquidity = getLiquidityForAmount0(sqrtRatioAX96, sqrtRatioBX96, amount0);
    // } else if (sqrtRatioX96 < sqrtRatioBX96) {
    } else if sqrtRatioX96 < sqrtRatioBX96 {
        // uint128 liquidity0 = getLiquidityForAmount0(sqrtRatioX96, sqrtRatioBX96, amount0);
        let liquidity0: u128 = getLiquidityForAmount0(sqrtRatioX96, sqrtRatioBX96, amount0);
        // uint128 liquidity1 = getLiquidityForAmount1(sqrtRatioAX96, sqrtRatioX96, amount1);
        let liquidity1: u128 = getLiquidityForAmount1(sqrtRatioAX96, sqrtRatioX96, amount1);
        // liquidity = liquidity0 < liquidity1 ? liquidity0 : liquidity1;
        liquidity = if liquidity0 < liquidity1 {
            liquidity0
        } else {
            liquidity1
        }
    // } else {
    //     liquidity = getLiquidityForAmount1(sqrtRatioAX96, sqrtRatioBX96, amount1);
    // }
    } else {
        liquidity = getLiquidityForAmount1(sqrtRatioAX96, sqrtRatioBX96, amount1);
    }

    liquidity
}

/// @notice Computes the amount of liquidity received for a given amount of token0 and price range
/// @dev Calculates amount0 * (sqrt(upper) * sqrt(lower)) / (sqrt(upper) - sqrt(lower))
/// @param sqrtRatioAX96 A sqrt price representing the first tick boundary
/// @param sqrtRatioBX96 A sqrt price representing the second tick boundary
/// @param amount0 The amount0 being sent in
/// @return liquidity The amount of returned liquidity
fn getLiquidityForAmount0(mut sqrtRatioAX96: U160, mut sqrtRatioBX96: U160, amount0: U256) -> u128 {
    (sqrtRatioAX96, sqrtRatioBX96) = arrangeToken(sqrtRatioAX96, sqrtRatioBX96);
    let intermediate: U256 =
        FullMath::mulDiv(sqrtRatioAX96, sqrtRatioBX96, U256::from(FixedPoint96::Q96));
    return FullMath::mulDiv(amount0, intermediate, sqrtRatioBX96 - sqrtRatioAX96).as_u128();
}

/// @notice Computes the amount of liquidity received for a given amount of token1 and price range
/// @dev Calculates amount1 / (sqrt(upper) - sqrt(lower)).
/// @param sqrtRatioAX96 A sqrt price representing the first tick boundary
/// @param sqrtRatioBX96 A sqrt price representing the second tick boundary
/// @param amount1 The amount1 being sent in
/// @return liquidity The amount of returned liquidity
fn getLiquidityForAmount1(mut sqrtRatioAX96: U160, mut sqrtRatioBX96: U160, amount1: U256) -> u128 {
    (sqrtRatioAX96, sqrtRatioBX96) = arrangeToken(sqrtRatioAX96, sqrtRatioBX96);
    return FullMath::mulDiv(
        amount1,
        U256::from(FixedPoint96::Q96),
        sqrtRatioBX96 - sqrtRatioAX96,
    )
    .as_u128();
}

fn arrangeToken(mut rationA: U160, mut ratioB: U160) -> (U160, U160) {
    if rationA > ratioB {
        let temp = ratioB;
        ratioB = rationA;
        rationA = temp;
    }
    (rationA, ratioB)
}

#[test]
fn my_temp_test() {
    let mut liquidity1 = U256::from(200);
    let mut liquidity2 = U256::from(100);
    (liquidity1, liquidity2) = arrangeToken(liquidity1, liquidity2);
    println!("liquidity1 is:{:?},liquidity2:{:?}", liquidity1, liquidity2);
    const CODE_HASH_1: [u8; 32] = [1u8; 32];
    let hash = Hash::try_from(CODE_HASH_1).unwrap();
    
    println!("hash is:{:?}",hash);
}
