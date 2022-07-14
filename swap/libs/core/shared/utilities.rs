#![allow(non_snake_case)]
use primitives::U256;

pub fn encodePriceSqrt(reserve1: U256, reserve0: U256) -> U256 {
    // return BigNumber.from(
    //     new bn(reserve1.toString())
    //       .div(reserve0.toString())
    //       .sqrt()
    //       .multipliedBy(new bn(2).pow(96))
    //       .integerValue(3)
    //       .toString()
    //   )
    reserve1 / reserve0.integer_sqrt() * (U256::from(2).pow(U256::from(96)))
}

// export function expandTo18Decimals(n: number): BigNumber {
//     return BigNumber.from(n).mul(BigNumber.from(10).pow(18))
//   }

pub fn expandTo18Decimals(n:u128)->u128{
    n*(10u128.pow(18))
}

#[cfg(test)]
mod test{
    use super::expandTo18Decimals;

    #[test]
    fn tesTexpandTo18Decimals(){
        let result = expandTo18Decimals(2);
        println!("result is:{}",result);
        println!("u128 max is:{}",u128::MAX);
    }
}