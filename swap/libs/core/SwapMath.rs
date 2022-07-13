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
    ink_env::debug_println!("sqrtRatioCurrentX96,sqrtRatioTargetX96,liquidity,amountRemaining,feePips is:{:?},{:?},{:?},{:?},{:?}",sqrtRatioCurrentX96,sqrtRatioTargetX96,liquidity,amountRemaining,feePips);

    let sqrtRatioNextX96: U160;
    let mut amountIn: U256 = U256::zero();
    let mut amountOut: U256 = U256::zero();
    let feeAmount: U256;
    // bool zeroForOne = sqrtRatioCurrentX96 >= sqrtRatioTargetX96;
    // bool exactIn = amountRemaining >= 0;
    let zeroForOne: bool = sqrtRatioCurrentX96 >= sqrtRatioTargetX96;
    let exactIn: bool = amountRemaining >= 0;

    if exactIn {
        // uint256 amountRemainingLessFee = FullMath.mulDiv(uint256(amountRemaining), 1e6 - feePips, 1e6);
        let amountRemainingLessFee: U256 = FullMath::mulDiv(
            U256::from(amountRemaining),
            U256::from(E6 - feePips),
            U256::from(E6),
        );
        // amountIn = zeroForOne
        //     ? SqrtPriceMath.getAmount0Delta(sqrtRatioTargetX96, sqrtRatioCurrentX96, liquidity, true)
        //     : SqrtPriceMath.getAmount1Delta(sqrtRatioCurrentX96, sqrtRatioTargetX96, liquidity, true);
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
        // if (amountRemainingLessFee >= amountIn) sqrtRatioNextX96 = sqrtRatioTargetX96;
        // else
        //     sqrtRatioNextX96 = SqrtPriceMath.getNextSqrtPriceFromInput(
        //         sqrtRatioCurrentX96,
        //         liquidity,
        //         amountRemainingLessFee,
        //         zeroForOne
        //     );
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
        // amountOut = zeroForOne
        //     ? SqrtPriceMath.getAmount1Delta(sqrtRatioTargetX96, sqrtRatioCurrentX96, liquidity, false)
        //     : SqrtPriceMath.getAmount0Delta(sqrtRatioCurrentX96, sqrtRatioTargetX96, liquidity, false);
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
        // if (uint256(-amountRemaining) >= amountOut) sqrtRatioNextX96 = sqrtRatioTargetX96;
        if U256::from(-amountRemaining) >= amountOut {
            sqrtRatioNextX96 = sqrtRatioTargetX96;
        } else {
            // else
            //     sqrtRatioNextX96 = SqrtPriceMath.getNextSqrtPriceFromOutput(
            //         sqrtRatioCurrentX96,
            //         liquidity,
            //         uint256(-amountRemaining),
            //         zeroForOne
            //     );
            sqrtRatioNextX96 = SqrtPriceMath::getNextSqrtPriceFromOutput(
                sqrtRatioCurrentX96,
                liquidity,
                U256::from(-amountRemaining),
                zeroForOne,
            );
        }
    }
    // bool max = sqrtRatioTargetX96 == sqrtRatioNextX96;
    let max: bool = sqrtRatioTargetX96 == sqrtRatioNextX96;

    // get the input/output amounts
    // if (zeroForOne) {
    if zeroForOne {
        //     amountIn = max && exactIn
        //         ? amountIn
        //         : SqrtPriceMath.getAmount0Delta(sqrtRatioNextX96, sqrtRatioCurrentX96, liquidity, true);
        //     amountOut = max && !exactIn
        //         ? amountOut
        //         : SqrtPriceMath.getAmount1Delta(sqrtRatioNextX96, sqrtRatioCurrentX96, liquidity, false);
        // } else {

        // }
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
        //     amountIn = max && exactIn
        //         ? amountIn
        //         : SqrtPriceMath.getAmount1Delta(sqrtRatioCurrentX96, sqrtRatioNextX96, liquidity, true);
        //     amountOut = max && !exactIn
        //         ? amountOut
        //         : SqrtPriceMath.getAmount0Delta(sqrtRatioCurrentX96, sqrtRatioNextX96, liquidity, false);
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
    // if (!exactIn && amountOut > uint256(-amountRemaining)) {
    //     amountOut = uint256(-amountRemaining);
    // }
    if !exactIn && amountOut > U256::from(-amountRemaining) {
        amountOut = U256::from(-amountRemaining);
    }
    // if (exactIn && sqrtRatioNextX96 != sqrtRatioTargetX96) {
    //     // we didn't reach the target, so take the remainder of the maximum input as fee
    //     feeAmount = uint256(amountRemaining) - amountIn;
    // } else {
    //     feeAmount = FullMath.mulDivRoundingUp(amountIn, feePips, 1e6 - feePips);
    // }
    if exactIn && sqrtRatioNextX96 != sqrtRatioTargetX96 {
        // we didn't reach the target, so take the remainder of the maximum input as fee
        feeAmount = U256::from(amountRemaining) - amountIn;
    } else {
        feeAmount =
            FullMath::mulDivRoundingUp(amountIn, U256::from(feePips), U256::from(E6 - feePips));
    }
    (sqrtRatioNextX96, amountIn, amountOut, feeAmount)
}

#[cfg(test)]
mod test {
    use primitives::U256;

    use crate::core::shared::utilities::encodePriceSqrt;

    use super::computeSwapStep;

    #[test]
    fn test_work() {
        // sqrtRatioCurrentX96: U160,sqrtRatioTargetX96: U160,liquidity: u128,amountRemaining: Int256,feePips: Uint24,
        let (sqrtPriceX96, amountIn, amountOut, feeAmount) = computeSwapStep(
            U256::from_dec_str("120621891405341611593710811006").unwrap(),
            U256::from_dec_str("12062189140534161159371081100").unwrap(),
            2125,
            50,
            500,
        );
        println!(
            "sqrtPriceX96, amountIn, amountOut, feeAmount is:{:?},{:?},{:?},{:?}",
            sqrtPriceX96, amountIn, amountOut, feeAmount
        );
    }

    #[test]
    fn testComputeSwapStep() {
        //exact amount in that gets capped at price target in one for zero
        // const price = encodePriceSqrt(1, 1)
        let price = encodePriceSqrt(1, 1);
        // const priceTarget = encodePriceSqrt(101, 100)
        // const liquidity = expandTo18Decimals(2)
        // const amount = expandTo18Decimals(1)
        // const fee = 600
        // const zeroForOne = false

        // const { amountIn, amountOut, sqrtQ, feeAmount } = await swapMath.computeSwapStep(
        //     price,
        //     priceTarget,
        //     liquidity,
        //     amount,
        //     fee
        // )

        // expect(amountIn).to.eq('9975124224178055')
        // expect(feeAmount).to.eq('5988667735148')
        // expect(amountOut).to.eq('9925619580021728')
        // expect(amountIn.add(feeAmount), 'entire amount is not used').to.lt(amount)

        // const priceAfterWholeInputAmount = await sqrtPriceMath.getNextSqrtPriceFromInput(
        //     price,
        //     liquidity,
        //     amount,
        //     zeroForOne
        // )

        // expect(sqrtQ, 'price is capped at price target').to.eq(priceTarget)
        // expect(sqrtQ, 'price is less than price after whole input amount').to.lt(priceAfterWholeInputAmount)
    }
}
