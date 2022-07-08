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
    a.checked_mul(b).unwrap().checked_div(denominator).unwrap()
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
mod FullMathTest {

    use primitives::U256;

    use crate::swap::FullMath;

    static Q128: u64 = 128u64;

    //reverts if denominator is 0
    #[test]
    #[should_panic]
    fn testMulDivPannicIsDivZero() {
        let q128 = U256::from(2).pow(U256::from(Q128));
        FullMath::mulDiv(q128, U256::from(5), U256::from(0));
    }

    //reverts if denominator is 0 and numerator overflows
    #[test]
    #[should_panic]
    fn testMulDivPannic_numerator_overflows() {
        // await expect(fullMath.mulDiv(Q128, Q128, 0)).to.be.reverted
        let q128 = U256::from(2).pow(U256::from(Q128));
        // await expect(fullMath.mulDiv(Q128, Q128, 0)).to.be.reverted
        FullMath::mulDiv(q128, q128, U256::from(0));
    }

    // reverts if output overflows uint256
    #[test]
    #[should_panic]
    fn reverts_if_output_overflows_uint256() {
        let q128 = U256::from(2).pow(U256::from(Q128));
        // await expect(fullMath.mulDiv(Q128, Q128, 0)).to.be.reverted
        FullMath::mulDiv(q128, q128, U256::from(1));
    }

    // reverts on overflow with all max inputs
    #[test]
    #[should_panic]
    fn reverts_on_overflow_with_all_max_inputs() {
        // await expect(fullMath.mulDiv(MaxUint256, MaxUint256, MaxUint256.sub(1))).to.be.reverted
        let MaxUint256 = U256::MAX;
        FullMath::mulDiv(
            MaxUint256,
            MaxUint256,
            MaxUint256.checked_sub(U256::one()).unwrap(),
        );
    }

    //all max inputs
    #[test]
    fn all_max_inputs() {
        // expect(await fullMath.mulDiv(MaxUint256, MaxUint256, MaxUint256)).to.eq(MaxUint256)
        let MaxU128 = U256::from(u128::MAX);
        assert_eq!(FullMath::mulDiv(MaxU128, MaxU128, MaxU128), MaxU128);
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


    // accurate without phantom overflow1
    #[test]
    fn accurate_without_phantom_overflow1() {
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
                /*0.5=*/
                U256::from(50)
                    .checked_mul(q128)
                    .unwrap()
                    .checked_div(U256::from(100))
                    .unwrap(),
                /*1.5=*/
                U256::from(150)
                    .checked_mul(q128)
                    .unwrap()
                    .checked_div(U256::from(100))
                    .unwrap(),
            ),
            result
        );
    }

    // accurate with phantom overflow2
    #[test]
    fn accurate_with_phantom_overflow2() {
        let q128 = U256::from(2).pow(U256::from(Q128));
        // const result = BigNumber.from(4375).mul(Q128).div(1000)
        let result = U256::from(35).saturating_mul(U256::from(60));
        // expect(await fullMath.mulDiv(Q128, BigNumber.from(35).mul(Q128), BigNumber.from(8).mul(Q128))).to.eq(result)
        assert_eq!(FullMath::mulDiv(q128, U256::from(35).saturating_mul(U256::from(60)), q128),result)
    }

    // it('accurate with phantom overflow and repeating decimal', async () => {
    //     const result = BigNumber.from(1).mul(Q128).div(3)
    //     expect(await fullMath.mulDiv(Q128, BigNumber.from(1000).mul(Q128), BigNumber.from(3000).mul(Q128))).to.eq(result)
    //   })


    // describe('#mulDivRoundingUp', () => {
    //mulDivRoundingUp
    //     it('reverts if denominator is 0', async () => {
    //       await expect(fullMath.mulDivRoundingUp(Q128, 5, 0)).to.be.reverted
    //     })
    #[test]
    #[should_panic]
    fn reverts_if_denominator_is_0(){
        let q128 = U256::from(2).pow(U256::from(Q128));
        FullMath::mulDivRoundingUp(q128, U256::from(5), U256::zero());
    }

    
    
    //     it('reverts if denominator is 0 and numerator overflows', async () => {
    //       await expect(fullMath.mulDivRoundingUp(Q128, Q128, 0)).to.be.reverted
    //     })
    #[test]
    #[should_panic]
    fn reverts_if_denominator_is_0_and_numerator_overflows(){
        let q128 = U256::from(2).pow(U256::from(Q128));
        FullMath::mulDivRoundingUp(q128, q128, U256::zero());
    }

    //     it('reverts if output overflows uint256', async () => {
    //       await expect(fullMath.mulDivRoundingUp(Q128, Q128, 1)).to.be.reverted
    //     })
    #[test]
    #[should_panic]
    fn reverts_if_output_overflows_uint2561(){
        let q128 = U256::from(2).pow(U256::from(Q128));
        FullMath::mulDivRoundingUp(q128, q128, U256::one());
    }

    //     it('reverts on overflow with all max inputs', async () => {
    //       await expect(fullMath.mulDivRoundingUp(MaxUint256, MaxUint256, MaxUint256.sub(1))).to.be.reverted
    //     })
    #[test]
    #[should_panic]
    fn reverts_on_overflow_with_all_max_inputs1(){
        FullMath::mulDivRoundingUp(U256::MAX, U256::MAX, U256::MAX.saturating_sub(U256::one()));
    }
    
    //     it('reverts if mulDiv overflows 256 bits after rounding up', async () => {
    //       await expect(
    //         fullMath.mulDivRoundingUp(
    //           '535006138814359',
    //           '432862656469423142931042426214547535783388063929571229938474969',
    //           '2'
    //         )
    //       ).to.be.reverted
    //     })
    
    //     it('reverts if mulDiv overflows 256 bits after rounding up case 2', async () => {
    //       await expect(
    //         fullMath.mulDivRoundingUp(
    //           '115792089237316195423570985008687907853269984659341747863450311749907997002549',
    //           '115792089237316195423570985008687907853269984659341747863450311749907997002550',
    //           '115792089237316195423570985008687907853269984653042931687443039491902864365164'
    //         )
    //       ).to.be.reverted
    //     })
    
    //     it('all max inputs', async () => {
    //       expect(await fullMath.mulDivRoundingUp(MaxUint256, MaxUint256, MaxUint256)).to.eq(MaxUint256)
    //     })
    
    //     it('accurate without phantom overflow', async () => {
    //       const result = Q128.div(3).add(1)
    //       expect(
    //         await fullMath.mulDivRoundingUp(
    //           Q128,
    //           /*0.5=*/ BigNumber.from(50).mul(Q128).div(100),
    //           /*1.5=*/ BigNumber.from(150).mul(Q128).div(100)
    //         )
    //       ).to.eq(result)
    //     })
    
    //     it('accurate with phantom overflow', async () => {
    //       const result = BigNumber.from(4375).mul(Q128).div(1000)
    //       expect(await fullMath.mulDivRoundingUp(Q128, BigNumber.from(35).mul(Q128), BigNumber.from(8).mul(Q128))).to.eq(
    //         result
    //       )
    //     })
    
    //     it('accurate with phantom overflow and repeating decimal', async () => {
    //       const result = BigNumber.from(1).mul(Q128).div(3).add(1)
    //       expect(
    //         await fullMath.mulDivRoundingUp(Q128, BigNumber.from(1000).mul(Q128), BigNumber.from(3000).mul(Q128))
    //       ).to.eq(result)
    //     })
    //   })
}

