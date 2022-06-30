#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use primitives::{Int256, Uint128, Uint160, U160, U256};

use crate::core::UnsafeMath;
use crate::swap::{FixedPoint96, FullMath};

/// @notice Gets the next sqrt price given a delta of token0
/// @dev Always rounds up, because in the exact output case (increasing price) we need to move the price at least
/// far enough to get the desired output amount, and in the exact input case (decreasing price) we need to move the
/// price less in order to not send too much output.
/// The most precise formula for this is liquidity * sqrtPX96 / (liquidity +- amount * sqrtPX96),
/// if this is impossible because of overflow, we calculate liquidity / (liquidity / sqrtPX96 +- amount).
/// @param sqrtPX96 The starting price, i.e. before accounting for the token0 delta
/// @param liquidity The amount of usable liquidity
/// @param amount How much of token0 to add or remove from virtual reserves
/// @param add Whether to add or remove the amount of token0
/// @return The price after adding or removing amount, depending on add
pub fn getNextSqrtPriceFromAmount0RoundingUp(
    sqrtPX96: U160,
    liquidity: u128,
    amount: U256,
    add: bool,
) -> U160 {
    // we short circuit amount == 0 because the result is otherwise not guaranteed to equal the input price
    if amount == U256::zero() {
        return sqrtPX96;
    }
    let numerator1: U256 = U256::from(liquidity) << FixedPoint96::RESOLUTION;
    // if (add) {
    //     uint256 product;
    //     if ((product = amount * sqrtPX96) / amount == sqrtPX96) {
    //         uint256 denominator = numerator1 + product;
    //         if (denominator >= numerator1)
    //             // always fits in 160 bits
    //             return uint160(FullMath.mulDivRoundingUp(numerator1, sqrtPX96, denominator));
    //     }

    //     return uint160(UnsafeMath.divRoundingUp(numerator1, (numerator1 / sqrtPX96).add(amount)));
    // } else {
    //     uint256 product;
    //     // if the product overflows, we know the denominator underflows
    //     // in addition, we must check that the denominator does not underflow
    //     require((product = amount * sqrtPX96) / amount == sqrtPX96 && numerator1 > product);
    //     uint256 denominator = numerator1 - product;
    //     return FullMath.mulDivRoundingUp(numerator1, sqrtPX96, denominator).toUint160();
    // }
    if add {
        let product: U256 = amount * sqrtPX96;
        if product / amount == sqrtPX96 {
            let denominator = numerator1 + product;
            if denominator >= numerator1 {
                // always fits in 160 bits
                return FullMath::mulDivRoundingUp(numerator1, sqrtPX96, denominator);
            }
        }

        return UnsafeMath::divRoundingUp(numerator1, (numerator1 / sqrtPX96) + amount);
    } else {
        let product: U256 = amount * sqrtPX96;
        // if the product overflows, we know the denominator underflows
        // in addition, we must check that the denominator does not underflow
        assert!(
            product / amount == sqrtPX96 && numerator1 > product,
            "product/amount bt sqrtPX96!"
        );
        let denominator: U256 = numerator1 - product;
        return FullMath::mulDivRoundingUp(numerator1, sqrtPX96, denominator);
    }
}

/// @notice Gets the next sqrt price given a delta of token1
/// @dev Always rounds down, because in the exact output case (decreasing price) we need to move the price at least
/// far enough to get the desired output amount, and in the exact input case (increasing price) we need to move the
/// price less in order to not send too much output.
/// The formula we compute is within <1 wei of the lossless version: sqrtPX96 +- amount / liquidity
/// @param sqrtPX96 The starting price, i.e., before accounting for the token1 delta
/// @param liquidity The amount of usable liquidity
/// @param amount How much of token1 to add, or remove, from virtual reserves
/// @param add Whether to add, or remove, the amount of token1
/// @return The price after adding or removing `amount`
pub fn getNextSqrtPriceFromAmount1RoundingDown(
    sqrtPX96: U160,
    liquidity: u128,
    amount: U256,
    add: bool,
) -> U160 {
    // if we're adding (subtracting), rounding down requires rounding the quotient down (up)
    // in both cases, avoid a mulDiv for most inputs
    if add {
        let quotient: U256 = if amount <= U160::MAX {
            (amount << FixedPoint96::RESOLUTION) / liquidity
        } else {
            FullMath::mulDiv(amount, U256::from(FixedPoint96::Q96), U256::from(liquidity))
        };

        return U256::from(sqrtPX96) + quotient;
    } else {
        let quotient: U256 = if amount <= U160::MAX {
            UnsafeMath::divRoundingUp(amount << FixedPoint96::RESOLUTION, U256::from(liquidity))
        } else {
            FullMath::mulDivRoundingUp(amount, U256::from(FixedPoint96::Q96), U256::from(liquidity))
        };

        assert!(sqrtPX96 > quotient, "sqrtPX96 > quotient");
        // always fits 160 bits
        return U160::from(sqrtPX96 - quotient);
    }
}

/// @notice Gets the next sqrt price given an input amount of token0 or token1
/// @dev Throws if price or liquidity are 0, or if the next price is out of bounds
/// @param sqrtPX96 The starting price, i.e., before accounting for the input amount
/// @param liquidity The amount of usable liquidity
/// @param amountIn How much of token0, or token1, is being swapped in
/// @param zeroForOne Whether the amount in is token0 or token1
/// @return sqrtQX96 The price after adding the input amount to token0 or token1
pub fn getNextSqrtPriceFromInput(
    sqrtPX96: U160,
    liquidity: u128,
    amountIn: U256,
    zeroForOne: bool,
) -> U160 {
    assert!(sqrtPX96 > U256::zero(), "sqrtPX96 should bt 0");
    assert!(liquidity > 0, "liquidity should bt 0");

    // round to make sure that we don't pass the target price
    if zeroForOne {
        getNextSqrtPriceFromAmount0RoundingUp(sqrtPX96, liquidity, amountIn, true)
    } else {
        getNextSqrtPriceFromAmount1RoundingDown(sqrtPX96, liquidity, amountIn, true)
    }
}

/// @notice Gets the next sqrt price given an output amount of token0 or token1
/// @dev Throws if price or liquidity are 0 or the next price is out of bounds
/// @param sqrtPX96 The starting price before accounting for the output amount
/// @param liquidity The amount of usable liquidity
/// @param amountOut How much of token0, or token1, is being swapped out
/// @param zeroForOne Whether the amount out is token0 or token1
/// @return sqrtQX96 The price after removing the output amount of token0 or token1
pub fn getNextSqrtPriceFromOutput(
    sqrtPX96: U160,
    liquidity: u128,
    amountOut: U256,
    zeroForOne: bool,
) -> U160 {
    assert!(sqrtPX96 > U256::zero(), "sqrtPX96 > 0");
    assert!(liquidity > 0, "liquidity > 0");

    // round to make sure that we pass the target price
    if zeroForOne {
        getNextSqrtPriceFromAmount1RoundingDown(sqrtPX96, liquidity, amountOut, false)
    } else {
        getNextSqrtPriceFromAmount0RoundingUp(sqrtPX96, liquidity, amountOut, false)
    }
}

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
/// getAmount0Delta with a bool parameter
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

#[cfg(test)]
mod SqrtPriceMathTest {

    use core::ops::{Div, Mul, Shl, Sub};

    use primitives::U256;

    use crate::core::SqrtPriceMath;

    fn expandTo18Decimals(n: &U256) -> U256 {
        return n
            .checked_mul(U256::from(10).checked_pow(U256::from(18)).unwrap())
            .unwrap();
    }

    // returns the sqrt price as a 64x96
    fn encodePriceSqrt(reserve1: U256, reserve0: U256) -> U256 {
        reserve1
            .div(reserve0)
            .integer_sqrt()
            .mul(U256::from(2).pow(U256::from(96)))
    }

    //#getNextSqrtPriceFromInput
    #[test]
    #[should_panic(expected = "sqrtPX96 should bt 0")]
    fn testgetNextSqrtPriceFromInput() {
        SqrtPriceMath::getNextSqrtPriceFromInput(
            U256::zero(),
            0,
            expandTo18Decimals(&U256::from(1))
                .checked_div(U256::from(10))
                .unwrap(),
            false,
        );
    }

    //#fails if liquidity is zero
    #[test]
    #[should_panic(expected = "liquidity should bt 0")]
    fn fails_if_liquidity_is_zero() {
        // sqrtPriceMath.getNextSqrtPriceFromInput(1, 0, expandTo18Decimals(1).div(10), true)
        SqrtPriceMath::getNextSqrtPriceFromInput(
            U256::one(),
            0,
            expandTo18Decimals(&U256::one())
                .checked_div(U256::from(10))
                .unwrap(),
            true,
        );
    }

    //fails if input amount overflows the price
    #[test]
    #[should_panic]
    fn fails_if_input_amount_overflows_the_price() {
        // const price = BigNumber.from(2).pow(160).sub(1)
        // const liquidity = 1024
        // const amountIn = 1024
        let price = U256::from(2)
            .checked_pow(U256::from(160))
            .unwrap()
            .checked_sub(U256::one())
            .unwrap();
        let liquidity = 1024;
        let amountIn = U256::from(1024);
        // sqrtPriceMath.getNextSqrtPriceFromInput(price, liquidity, amountIn, false)
        SqrtPriceMath::getNextSqrtPriceFromInput(price, liquidity, amountIn, false);
    }

    #[test]
    fn testGetNextSqrtPriceFromInput() {
        // any input amount cannot underflow the price
        let price = U256::one();
        let liquidity = 1;
        let amountIn = U256::from(2).checked_pow(U256::from(255)).unwrap();
        //   expect(await sqrtPriceMath.getNextSqrtPriceFromInput(price, liquidity, amountIn, true)).to.eq(1)
        assert_eq!(
            SqrtPriceMath::getNextSqrtPriceFromInput(price, liquidity, amountIn, true),
            U256::from(1)
        );

        // returns input price if amount in is zero and zeroForOne = true
        let price = encodePriceSqrt(U256::one(), U256::one());
        // expect(await sqrtPriceMath.getNextSqrtPriceFromInput(price, expandTo18Decimals(1).div(10), 0, false)).to.eq(price)
        assert_eq!(
            SqrtPriceMath::getNextSqrtPriceFromInput(
                price,
                expandTo18Decimals(&U256::one()).div(10).as_u128(),
                U256::zero(),
                false
            ),
            price
        );

        //returns input price if amount in is zero and zeroForOne = false
        let price = encodePriceSqrt(U256::one(), U256::one());
        assert_eq!(
            SqrtPriceMath::getNextSqrtPriceFromInput(
                price,
                expandTo18Decimals(&U256::one()).div(10).as_u128(),
                U256::zero(),
                false
            ),
            price
        );

        //returns the minimum price for max inputs
        let sqrtP = U256::from(2).pow(U256::from(160)).sub(1);
        let liquidity = u128::MAX;
        let maxAmountNoOverflow = U256::MAX.sub(U256::from(liquidity).shl(96).div(sqrtP));
        print!("maxAmountNoOverflow is:{:?}",maxAmountNoOverflow);
        assert_eq!(
            SqrtPriceMath::getNextSqrtPriceFromInput(sqrtP, liquidity, maxAmountNoOverflow, true),
            U256::from(1)
        );
    }
}
