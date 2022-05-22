#![allow(non_snake_case)]

use primitives::U256;

/// @notice Returns the index of the most significant bit of the number,
///     where the least significant bit is at index 0 and the most significant bit is at index 255
/// @dev The function satisfies the property:
///     x >= 2**mostSignificantBit(x) and x < 2**(mostSignificantBit(x)+1)
/// @param x the value for which to compute the most significant bit, must be greater than 0
/// @return r the index of the most significant bit
pub fn mostSignificantBit(mut x: U256) -> u8 {
    assert!(x > U256::zero(), "x must bt 0!");
    let mut r: u8 = 0;
    if x >= U256::from_str_radix("100000000000000000000000000000000", 16)
        .expect("exchange str to U256 error!")
    {
        x >>= 128;
        r += 128;
    }
    if x >= U256::from_str_radix("10000000000000000", 16).expect("exchange str to U256 error!") {
        x >>= 64;
        r += 64;
    }
    if x >= U256::from_str_radix("100000000", 16).expect("exchange str to U256 error!") {
        x >>= 32;
        r += 32;
    }
    if x >= U256::from_str_radix("10000", 16).expect("exchange str to U256 error!") {
        x >>= 16;
        r += 16;
    }
    if x >= U256::from_str_radix("100", 16).expect("exchange str to U256 error!") {
        x >>= 8;
        r += 8;
    }
    if x >= U256::from_str_radix("10", 16).expect("exchange str to U256 error!") {
        x >>= 4;
        r += 4;
    }
    if x >= U256::from_str_radix("4", 16).expect("exchange str to U256 error!") {
        x >>= 2;
        r += 2;
    }
    if x >= U256::from_str_radix("2", 16).expect("exchange str to U256 error!") {
        r += 1;
    }
    r
}

/// @notice Returns the index of the least significant bit of the number,
///     where the least significant bit is at index 0 and the most significant bit is at index 255
/// @dev The function satisfies the property:
///     (x & 2**leastSignificantBit(x)) != 0 and (x & (2**(leastSignificantBit(x)) - 1)) == 0)
/// @param x the value for which to compute the least significant bit, must be greater than 0
/// @return r the index of the least significant bit
pub fn leastSignificantBit(mut x: U256) -> u8 {
    assert!(x > U256::zero(), "x must bt 0");
    let mut r: u8 = 255;
    if x & U256::from(u128::MAX) > U256::zero() {
        r -= 128;
    } else {
        x >>= 128;
    }
    if x & U256::from(u64::MAX) > U256::zero() {
        r -= 64;
    } else {
        x >>= 64;
    }
    if x & U256::from(u32::MAX) > U256::zero() {
        r -= 32;
    } else {
        x >>= 32;
    }
    if x & U256::from(u16::MAX) > U256::zero() {
        r -= 16;
    } else {
        x >>= 16;
    }
    if x & U256::from(u8::MAX) > U256::zero() {
        r -= 8;
    } else {
        x >>= 8;
    }
    if x & U256::from("f") > U256::zero() {
        r -= 4;
    } else {
        x >>= 4;
    }
    if x & U256::from("3") > U256::zero() {
        r -= 2;
    } else {
        x >>= 2;
    }
    if x & U256::from("1") > U256::zero() {
        r -= 1
    };
    r
}
