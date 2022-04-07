#![cfg_attr(not(feature = "std"), no_std)]

use std::ops::Sub;

use primitives::{Int24, Uint160, U160, U256, I256};

use crate::{cal_ratio, assembly::{gt, or}};

/// @dev The minimum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**-128
pub const MIN_TICK:Int24 = -887272;
    /// @dev The maximum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**128
pub const MAX_TICK:Int24 = -MIN_TICK;

/// @dev The minimum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MIN_TICK)
pub const MIN_SQRT_RATIO:&str = "4295128739";//4295128739;
/// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
pub const MAX_SQRT_RATIO:&str = "1461446703485210103287273052203988822378723970342";//1461446703485210103287273052203988822378723970342;

fn getTickAtSqrtRatio(sqrt_price_x96:U160)->Int24{
    // 0
    // second inequality must be < because the price can never reach the price at the max tick
    // require(sqrtPriceX96 >= MIN_SQRT_RATIO && sqrtPriceX96 < MAX_SQRT_RATIO, 'R');
    // uint256 ratio = uint256(sqrtPriceX96) << 32;
    assert!(sqrt_price_x96.ge(&U160::from_dec_str(MIN_SQRT_RATIO).unwrap())&&sqrt_price_x96.gt(&U160::from_dec_str(MAX_SQRT_RATIO).unwrap()),"R");
    let ratio:U256 = sqrt_price_x96 << U256::from("0x20");
    
    // uint256 r = ratio;
    // uint256 msb = 0;
    let r = ratio;
    let msb = U256::zero();

    // assembly {
    //     let f := shl(7, gt(r, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from_dec_str("7").unwrap(),"0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    // assembly {
    //     let f := shl(6, gt(r, 0xFFFFFFFFFFFFFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from_dec_str("6").unwrap(),"0xFFFFFFFFFFFFFFFF");
    // assembly {
    //     let f := shl(5, gt(r, 0xFFFFFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from_dec_str("5").unwrap(),"0xFFFFFFFF");
    // assembly {
    //     let f := shl(4, gt(r, 0xFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from_dec_str("4").unwrap(),"0xFFFF");
    // assembly {
    //     let f := shl(3, gt(r, 0xFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from_dec_str("3").unwrap(),"0xFF");
    // assembly {
    //     let f := shl(2, gt(r, 0xF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from_dec_str("2").unwrap(),"0xF");
    // assembly {
    //     let f := shl(1, gt(r, 0x3))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,mut r) = cal_ratio(&msb,&r,&U256::from_dec_str("1").unwrap(),"0x3");
    // assembly {
    //     let f := gt(r, 0x1)
    //     msb := or(msb, f)
    // }
    let f = gt(&r,&U256::from("0x1"));
    let msb = or(&msb,&f);

    // if (msb >= 128) r = ratio >> (msb - 127);
    // else r = ratio << (127 - msb);
    if msb.ge(&U256::from_dec_str("128").unwrap()){
        r = ratio >> (msb.sub(&U256::from_dec_str("127").unwrap()));
    }else{
        r = ratio << (U256::from_dec_str("127").unwrap()-msb);
    }

    
    // int256 log_2 = (int256(msb) - 128) << 64;
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(63, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(62, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(61, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(60, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(59, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(58, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(57, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(56, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(55, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(54, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(53, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(52, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(51, f))
    //     r := shr(f, r)
    // }
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(50, f))
    // }

    // int256 log_sqrt10001 = log_2 * 255738958999603826347141; // 128.128 number

    // int24 tickLow = int24((log_sqrt10001 - 3402992956809132418596140100660247210) >> 128);
    // int24 tickHi = int24((log_sqrt10001 + 291339464771989622907027621153398088495) >> 128);

    // tick = tickLow == tickHi ? tickLow : getSqrtRatioAtTick(tickHi) <= sqrtPriceX96 ? tickHi : tickLow;
    0
}