#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]
use crate::assembly::mulmod;
use primitives::U256;

/// @notice Calculates floor(a×b÷denominator) with full precision. Throws if result overflows a uint256 or denominator == 0
/// @param a The multiplicand
/// @param b The multiplier
/// @param denominator The divisor
/// @return result The 256-bit result
/// @dev Credit to Remco Bloemen under MIT license https://xn--2-umb.com/21/muldiv
pub fn mulDiv(a: U256, b: U256, denominator: U256) -> U256 {
    a.checked_mul(b)
    .unwrap()
    .checked_div(denominator)
    .unwrap()
}

/// @notice Calculates ceil(a×b÷denominator) with full precision. Throws if result overflows a uint256 or denominator == 0
/// @param a The multiplicand
/// @param b The multiplier
/// @param denominator The divisor
/// @return result The 256-bit result
pub fn mulDivRoundingUp(a: U256, b: U256, denominator: U256) -> U256 {
    let mut result = mulDiv(a, b, denominator);
    if mulmod(&a, &b, &denominator) > U256::zero() {
        assert!(result < U256::MAX);
        result = result.saturating_add(U256::one());
    }
    result
}

#[cfg(test)]
mod FullMathTest{
    use primitives::U256;

    use crate::swap::FullMath;

    static Q128:u64 = 128u64;

    //reverts if denominator is 0
    #[test]
    #[should_panic]
    fn testMulDivPannicIsDivZero(){
        let q128 = U256::from(2).pow(U256::from(Q128));
        FullMath::mulDiv(q128, U256::from(5), U256::from(0));
    }
    
    //reverts if denominator is 0 and numerator overflows
    #[test]
    #[should_panic]
    fn testMulDivPannic_numerator_overflows(){
        // await expect(fullMath.mulDiv(Q128, Q128, 0)).to.be.reverted
        let q128 = U256::from(2).pow(U256::from(Q128));
        // await expect(fullMath.mulDiv(Q128, Q128, 0)).to.be.reverted
        FullMath::mulDiv(q128, q128, U256::from(0));
    }

    // reverts if output overflows uint256
    #[test]
    #[should_panic]
    fn reverts_if_output_overflows_uint256(){
        let q128 = U256::from(2).pow(U256::from(Q128));
        // await expect(fullMath.mulDiv(Q128, Q128, 0)).to.be.reverted
        FullMath::mulDiv(q128, q128, U256::from(1));
    }

    // reverts on overflow with all max inputs
    #[test]
    #[should_panic]
    fn reverts_on_overflow_with_all_max_inputs(){
        // await expect(fullMath.mulDiv(MaxUint256, MaxUint256, MaxUint256.sub(1))).to.be.reverted
        let MaxUint256 = U256::MAX;
        FullMath::mulDiv(MaxUint256, MaxUint256, MaxUint256.checked_sub(U256::one()).unwrap());
    }

    //all max inputs
    #[test]
    fn all_max_inputs(){
        // expect(await fullMath.mulDiv(MaxUint256, MaxUint256, MaxUint256)).to.eq(MaxUint256)
        let MaxU128 = U256::from(u128::MAX);
        assert_eq!(FullMath::mulDiv(MaxU128, MaxU128, MaxU128),MaxU128);
    }

    // it('accurate without phantom overflow', async () => {
    //     const result = Q128.div(3)
    //     expect(
    //       await fullMath.mulDiv(
    //         Q128,
    //         /*0.5=*/ BigNumber.from(50).mul(Q128).div(100),
    //         /*1.5=*/ BigNumber.from(150).mul(Q128).div(100)
    //       )
    //     ).to.eq(result)
    //   })

    // accurate without phantom overflow
    #[test]
    fn accurate_without_phantom_overflow(){
        // const result = Q128.div(3)
        // expect(
        //   await fullMath.mulDiv(
        //     Q128,
        //     /*0.5=*/ BigNumber.from(50).mul(Q128).div(100),
        //     /*1.5=*/ BigNumber.from(150).mul(Q128).div(100)
        //   )
        // ).to.eq(result)
        let q128 = U256::from(2).pow(U256::from(Q128));
        let result = q128.checked_div(U256::from(3)).unwrap();
        assert_eq!(
               FullMath::mulDiv(
                q128,
                /*0.5=*/ U256::from(50).checked_mul(q128).unwrap().checked_div(U256::from(100)).unwrap(),
                /*1.5=*/ U256::from(150).checked_mul(q128).unwrap().checked_div(U256::from(100)).unwrap(),
              ),result
            );
    }
}