#![allow(non_snake_case)]

use primitives::{Int256, Uint24, U160, U256};
const E6: Uint24 = 1000000;
use crate::swap::FullMath;

use super::SqrtPriceMath;

/// @notice Computes the result of swapping some amount in, or amount out, given the parameters of the swap
/// @dev The fee, plus the amount in, will never exceed the amount remaining if the swap's `amountSpecified` is positive
/// @param sqrtRatioCurrentX96 The current sqrt price of the pool
/// @param sqrtRatioTargetX96 The price that cannot be exceeded, from which the direction of the swap is inferred
/// @param liquidity The usable liquidity
/// @param amountRemaining How much input or output amount is remaining to be swapped in/out
/// @param feePips The fee taken from the input amount, expressed in hundredths of a bip
/// @return sqrtRatioNextX96 The price after swapping the amount in/out, not to exceed the price target
/// @return amountIn The amount to be swapped in, of either token0 or token1, based on the direction of the swap
/// @return amountOut The amount to be received, of either token0 or token1, based on the direction of the swap
/// @return feeAmount The amount of input that will be taken as a fee
pub fn computeSwapStep(
    sqrtRatioCurrentX96: U160,
    sqrtRatioTargetX96: U160,
    liquidity: u128,
    amountRemaining: Int256,
    feePips: Uint24,
) -> (U160, U256, U256, U256) {
    let sqrtRatioNextX96: U160;
    let mut amountIn: U256 = U256::zero();
    let mut amountOut: U256 = U256::zero();
    let feeAmount: U256;
    let zeroForOne: bool = sqrtRatioCurrentX96 >= sqrtRatioTargetX96;
    let exactIn: bool = amountRemaining >= 0;

    if exactIn {
        let amountRemainingLessFee: U256 = FullMath::mulDiv(
            U256::from(amountRemaining),
            U256::from(E6 - feePips),
            U256::from(E6),
        );
        let amountIn = if zeroForOne {
            SqrtPriceMath::getAmount0DeltaWithRound(
                sqrtRatioTargetX96,
                sqrtRatioCurrentX96,
                liquidity,
                true,
            )
        } else {
            SqrtPriceMath::getAmount0DeltaWithRound(
                sqrtRatioCurrentX96,
                sqrtRatioTargetX96,
                liquidity,
                true,
            )
        };
        if amountRemainingLessFee >= amountIn {
            sqrtRatioNextX96 = sqrtRatioTargetX96;
        } else {
            sqrtRatioNextX96 = SqrtPriceMath::getNextSqrtPriceFromInput(
                sqrtRatioCurrentX96,
                liquidity,
                amountRemainingLessFee,
                zeroForOne,
            );
        }
    } else {
        amountOut = if zeroForOne {
            SqrtPriceMath::getAmount1DeltaWithRound(
                sqrtRatioTargetX96,
                sqrtRatioCurrentX96,
                liquidity,
                false,
            )
        } else {
            SqrtPriceMath::getAmount1DeltaWithRound(
                sqrtRatioCurrentX96,
                sqrtRatioTargetX96,
                liquidity,
                false,
            )
        };
        if U256::from(-amountRemaining) >= amountOut {
            sqrtRatioNextX96 = sqrtRatioTargetX96;
        } else {
            sqrtRatioNextX96 = SqrtPriceMath::getNextSqrtPriceFromOutput(
                sqrtRatioCurrentX96,
                liquidity,
                U256::from(-amountRemaining),
                zeroForOne,
            );
        }
    }

    let max: bool = sqrtRatioTargetX96 == sqrtRatioNextX96;

    // get the input/output amounts
    if zeroForOne {
        amountIn = if max && exactIn {
            amountIn
        } else {
            SqrtPriceMath::getAmount0DeltaWithRound(
                sqrtRatioNextX96,
                sqrtRatioCurrentX96,
                liquidity,
                true,
            )
        };
        amountOut = if max && !exactIn {
            amountOut
        } else {
            SqrtPriceMath::getAmount0DeltaWithRound(
                sqrtRatioNextX96,
                sqrtRatioCurrentX96,
                liquidity,
                false,
            )
        };
    } else {
        amountIn = if max && exactIn {
            amountIn
        } else {
            SqrtPriceMath::getAmount0DeltaWithRound(
                sqrtRatioCurrentX96,
                sqrtRatioNextX96,
                liquidity,
                true,
            )
        };
        amountOut = if max && !exactIn {
            amountOut
        } else {
            SqrtPriceMath::getAmount0DeltaWithRound(
                sqrtRatioCurrentX96,
                sqrtRatioNextX96,
                liquidity,
                false,
            )
        };
    }

    // cap the output amount to not exceed the remaining output amount
    if !exactIn && amountOut > U256::from(-amountRemaining) {
        amountOut = U256::from(-amountRemaining);
    }

    if exactIn && sqrtRatioNextX96 != sqrtRatioTargetX96 {
        // we didn't reach the target, so take the remainder of the maximum input as fee
        feeAmount = U256::from(amountRemaining) - amountIn;
    } else {
        feeAmount =
            FullMath::mulDivRoundingUp(amountIn, U256::from(feePips), U256::from(E6 - feePips));
    }
    (sqrtRatioNextX96, amountIn, amountOut, feeAmount)
}
