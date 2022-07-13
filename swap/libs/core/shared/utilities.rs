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
    reserve1 / (reserve0).integer_sqrt() * (U256::from(2).pow(U256::from(96)))
}
