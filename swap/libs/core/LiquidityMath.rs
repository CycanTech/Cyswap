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
        println!("x is :{:?}",x);
        z = x.checked_sub(u128::try_from(-y).unwrap());
        match z {
            None=> panic!("LS"),
            Some(r)=> return r,
        }
    } else {
        z = x.checked_add(u128::try_from(y).unwrap());
        match z {
            None => panic!("LA"),
            Some(r)=> return r,
        }
    }
}


#[cfg(test)]
mod LiquidMathTest {

    use crate::core::LiquidityMath;

    //reverts if denominator is 0
    #[test]
    fn testAddDelta() {
        assert_eq!(LiquidityMath::addDelta(1, 0),1);
        assert_eq!(LiquidityMath::addDelta(1, -1),0);
        assert_eq!(LiquidityMath::addDelta(1, 1),2);
    }

    //2**128-15 + 15 overflows
    #[test]
    #[should_panic(expected = "LA")]
    fn test_overflows_add1(){
        LiquidityMath::addDelta(u128::MAX, 1);
    }

    #[test]
    #[should_panic(expected = "LS")]
    fn test_overflows_sub1(){
        LiquidityMath::addDelta(0, -1);
    }

    //3 + -4 underflows
    #[test]
    #[should_panic(expected = "LS")]
    fn test_overflows_3_sub_4(){
        LiquidityMath::addDelta(3, -4);
    }
}
