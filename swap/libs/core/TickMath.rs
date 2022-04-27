#![cfg_attr(not(feature = "std"), no_std)]

use primitives::{Int24, U160, U256};
use core::str::FromStr;

use crate::{cal_ratio, assembly::{gt, or}, cal_log};

/// @dev The minimum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**-128
pub const MIN_TICK:Int24 = -887272;
    /// @dev The maximum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**128
pub const MAX_TICK:Int24 = -MIN_TICK;

/// @dev The minimum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MIN_TICK)
pub const MIN_SQRT_RATIO:&str = "4295128739";//4295128739;
/// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
pub const MAX_SQRT_RATIO:&str = "1461446703485210103287273052203988822378723970342";//1461446703485210103287273052203988822378723970342;

pub fn get_tick_at_sqrt_ratio(sqrt_price_x96:U160)->Int24{
    // second inequality must be < because the price can never reach the price at the max tick
    // require(sqrtPriceX96 >= MIN_SQRT_RATIO && sqrtPriceX96 < MAX_SQRT_RATIO, 'R');
    // uint256 ratio = uint256(sqrtPriceX96) << 32;
    assert!(sqrt_price_x96.ge(&U160::from_dec_str(MIN_SQRT_RATIO).unwrap())&&sqrt_price_x96.le(&U160::from_dec_str(MAX_SQRT_RATIO).unwrap()),"R");
    let ratio:U256 = sqrt_price_x96 << 32;
    
    // uint256 r = ratio;
    // uint256 msb = 0;
    let r = ratio;
    let msb = U256::zero();

    // assembly {
    //     let f := shl(7, gt(r, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from(7),"0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    // assembly {
    //     let f := shl(6, gt(r, 0xFFFFFFFFFFFFFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from(6),"0xFFFFFFFFFFFFFFFF");
    // assembly {
    //     let f := shl(5, gt(r, 0xFFFFFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from(5),"0xFFFFFFFF");
    // assembly {
    //     let f := shl(4, gt(r, 0xFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from(4),"0xFFFF");
    // assembly {
    //     let f := shl(3, gt(r, 0xFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from(3),"0xFF");
    // assembly {
    //     let f := shl(2, gt(r, 0xF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,&U256::from(2),"0xF");
    // assembly {
    //     let f := shl(1, gt(r, 0x3))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,mut r) = cal_ratio(&msb,&r,&U256::from(1),"0x3");
    // assembly {
    //     let f := gt(r, 0x1)
    //     msb := or(msb, f)
    // }
    let f = gt(&r,&U256::from("0x1"));
    let msb = or(&msb,&f);

    // if (msb >= 128) r = ratio >> (msb - 127);
    // else r = ratio << (127 - msb);
    if msb.ge(&U256::from(128)){
        r = ratio >> (msb.saturating_sub(U256::from(127)));
    }else{
        r = ratio << (U256::from(127).saturating_sub(msb));
    }

    
    // int256 log_2 = (int256(msb) - 128) << 64;
    let mut log_2:U256;
    let log_2_is_position:bool;
    if msb.ge(&U256::from(128)){
        log_2= msb.saturating_sub(U256::from(128u32))<<64;
        log_2_is_position = true;
    }else{
        log_2= U256::from(128u32).saturating_sub(msb)<<64;
        log_2_is_position = false;
    }

    
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(63, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(63u32),log_2_is_position);
    
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(62, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(62u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(61, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(61u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(60, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(60u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(59, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(59u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(58, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(58u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(57, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(57u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(56, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(56u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(55, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(55u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(54, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(54u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(53, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(53u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(52, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(52u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(51, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(51u32),log_2_is_position);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(50, f))
    // }
    let (log_2,r) = cal_log(&r,log_2,&U256::from(50u32),log_2_is_position);
    //20220409 check point TODO

    // int256 log_sqrt10001 = log_2 * 255738958999603826347141; // 128.128 number
    let log_sqrt10001 = log_2.saturating_mul(U256::from_dec_str("255738958999603826347141").unwrap());
    // int24 tickLow = int24((log_sqrt10001 - 3402992956809132418596140100660247210) >> 128);
    let log_sub_const = U256::from_dec_str("3402992956809132418596140100660247210").unwrap();
    // let result_is_position;
    let tick_low:Int24;
    let log_sqrt10001_temp:U256;
    if log_2_is_position{
        if log_sqrt10001.ge(&log_sub_const){
            log_sqrt10001_temp = log_sqrt10001.saturating_sub(log_sub_const)>>128u32;
            tick_low = Int24::try_from(log_sqrt10001_temp.as_u32()).unwrap();
        }else{
            log_sqrt10001_temp=log_sub_const.saturating_sub(log_sqrt10001)>>128u32;
            tick_low = -Int24::try_from(log_sqrt10001_temp.as_u32()).unwrap();
        }
    }else{
        log_sqrt10001_temp = log_sqrt10001.saturating_add(log_sub_const)>>128u32;
        tick_low = -Int24::try_from(log_sqrt10001_temp).unwrap();
    }
    // int24 tickHi = int24((log_sqrt10001 + 291339464771989622907027621153398088495) >> 128);
    let tick_const=U256::from_dec_str("291339464771989622907027621153398088495").unwrap();
    let tick_hi:Int24;
    let log_sqrt10001_temp:U256;
    if log_2_is_position{
        log_sqrt10001_temp = log_sqrt10001.saturating_add(tick_const)>>128u32;
        tick_hi = Int24::try_from(log_sqrt10001_temp.as_u32()).unwrap();

    }else{
        if log_sqrt10001.ge(&tick_const){
            log_sqrt10001_temp = log_sqrt10001.saturating_sub(tick_const)>>128u32;
            tick_hi = -Int24::try_from(log_sqrt10001_temp.as_u32()).unwrap();
        }else{
            log_sqrt10001_temp = tick_const.saturating_sub(log_sqrt10001)>>128u32;
            tick_hi = Int24::try_from(log_sqrt10001_temp.as_u32()).unwrap();
        }
    }
    // tick = tickLow == tickHi ? tickLow : getSqrtRatioAtTick(tickHi) <= sqrtPriceX96 ? tickHi : tickLow;
    let tick:Int24;
    if tick_low == tick_hi{
        tick = tick_low;
    }else{
        if getSqrtRatioAtTick(tick_hi) <= sqrt_price_x96{
            tick = tick_hi;
        }else {
            tick = tick_low;
        }
    }
    tick
}

/// @notice Calculates sqrt(1.0001^tick) * 2^96
/// @dev Throws if |tick| > max tick
/// @param tick The input tick for the above formula
/// @return sqrtPriceX96 A Fixed point Q64.96 number representing the sqrt of the ratio of the two assets (token1/token0)
/// at the given tick
pub fn getSqrtRatioAtTick(tick:Int24) -> U160 {
    // uint256 absTick = tick < 0 ? uint256(-int256(tick)) : uint256(int256(tick));
    let abs_tick:U256;
    if tick < 0 {
        abs_tick=U256::from(-tick);
    } else {
        abs_tick = U256::from(tick);
    }
    // require(absTick <= uint256(MAX_TICK), 'T');
    assert!(abs_tick.le(&U256::from(MAX_TICK)),"T");
    // uint256 ratio = absTick & 0x1 != 0 ? 0xfffcb933bd6fad37aa2d162d1a594001 : 0x100000000000000000000000000000000;
    let mut ratio:U256;
    if (!abs_tick & U256::from(0x1)).is_zero() {
        ratio = U256::from_str("0xfffcb933bd6fad37aa2d162d1a594001").unwrap();
    } else {
        ratio = U256::from_str("0x100000000000000000000000000000000").unwrap();
    }
    // if (absTick & 0x2 != 0) ratio = (ratio * 0xfff97272373d413259a46990580e213a) >> 128;
    if !(abs_tick & U256::from(0x2)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xfff97272373d413259a46990580e213a").unwrap())>>128u32;
    }
    // if (absTick & 0x4 != 0) ratio = (ratio * 0xfff2e50f5f656932ef12357cf3c7fdcc) >> 128;
    if !(abs_tick & U256::from(0x4)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xfff2e50f5f656932ef12357cf3c7fdcc").unwrap())>>128u32;
    }
    // if (absTick & 0x8 != 0) ratio = (ratio * 0xffe5caca7e10e4e61c3624eaa0941cd0) >> 128;
    if !(abs_tick & U256::from(0x8)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xffe5caca7e10e4e61c3624eaa0941cd0").unwrap())>>128u32;
    }
    // if (absTick & 0x10 != 0) ratio = (ratio * 0xffcb9843d60f6159c9db58835c926644) >> 128;
    if !(abs_tick & U256::from(0x10)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xffcb9843d60f6159c9db58835c926644").unwrap())>>128u32;
    }
    // if (absTick & 0x20 != 0) ratio = (ratio * 0xff973b41fa98c081472e6896dfb254c0) >> 128;
    if !(abs_tick & U256::from(0x20)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xff973b41fa98c081472e6896dfb254c0").unwrap())>>128u32;
    }
    // if (absTick & 0x40 != 0) ratio = (ratio * 0xff2ea16466c96a3843ec78b326b52861) >> 128;
    if !(abs_tick & U256::from(0x40)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xff2ea16466c96a3843ec78b326b52861").unwrap())>>128u32;
    }
    // if (absTick & 0x80 != 0) ratio = (ratio * 0xfe5dee046a99a2a811c461f1969c3053) >> 128;
    if !(abs_tick & U256::from(0x80)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xfe5dee046a99a2a811c461f1969c3053").unwrap())>>128u32;
    }
    // if (absTick & 0x100 != 0) ratio = (ratio * 0xfcbe86c7900a88aedcffc83b479aa3a4) >> 128;
    if !(abs_tick & U256::from(0x100)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xfcbe86c7900a88aedcffc83b479aa3a4").unwrap())>>128u32;
    }
    // if (absTick & 0x200 != 0) ratio = (ratio * 0xf987a7253ac413176f2b074cf7815e54) >> 128;
    if !(abs_tick & U256::from(0x200)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xf987a7253ac413176f2b074cf7815e54").unwrap())>>128u32;
    }
    // if (absTick & 0x400 != 0) ratio = (ratio * 0xf3392b0822b70005940c7a398e4b70f3) >> 128;
    if !(abs_tick & U256::from(0x400)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xf3392b0822b70005940c7a398e4b70f3").unwrap())>>128u32;
    }
    // if (absTick & 0x800 != 0) ratio = (ratio * 0xe7159475a2c29b7443b29c7fa6e889d9) >> 128;
    if !(abs_tick & U256::from(0x800)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xe7159475a2c29b7443b29c7fa6e889d9").unwrap())>>128u32;
    }
    // if (absTick & 0x1000 != 0) ratio = (ratio * 0xd097f3bdfd2022b8845ad8f792aa5825) >> 128;
    if !(abs_tick & U256::from(0x1000)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xd097f3bdfd2022b8845ad8f792aa5825").unwrap())>>128u32;
    }
    // if (absTick & 0x2000 != 0) ratio = (ratio * 0xa9f746462d870fdf8a65dc1f90e061e5) >> 128;
    if !(abs_tick & U256::from(0x2000)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0xa9f746462d870fdf8a65dc1f90e061e5").unwrap())>>128u32;
    }
    // if (absTick & 0x4000 != 0) ratio = (ratio * 0x70d869a156d2a1b890bb3df62baf32f7) >> 128;
    if !(abs_tick & U256::from(0x4000)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0x70d869a156d2a1b890bb3df62baf32f7").unwrap())>>128u32;
    }
    // if (absTick & 0x8000 != 0) ratio = (ratio * 0x31be135f97d08fd981231505542fcfa6) >> 128;
    if !(abs_tick & U256::from(0x8000)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0x31be135f97d08fd981231505542fcfa6").unwrap())>>128u32;
    }
    // if (absTick & 0x10000 != 0) ratio = (ratio * 0x9aa508b5b7a84e1c677de54f3e99bc9) >> 128;
    if !(abs_tick & U256::from(0x10000)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0x9aa508b5b7a84e1c677de54f3e99bc9").unwrap())>>128u32;
    }
    // if (absTick & 0x20000 != 0) ratio = (ratio * 0x5d6af8dedb81196699c329225ee604) >> 128;
    if !(abs_tick & U256::from(0x20000)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0x5d6af8dedb81196699c329225ee604").unwrap())>>128u32;
    }
    // if (absTick & 0x40000 != 0) ratio = (ratio * 0x2216e584f5fa1ea926041bedfe98) >> 128;
    if !(abs_tick & U256::from(0x40000)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0x2216e584f5fa1ea926041bedfe98").unwrap())>>128u32;
    }
    // if (absTick & 0x80000 != 0) ratio = (ratio * 0x48a170391f7dc42444e8fa2) >> 128;
    if !(abs_tick & U256::from(0x80000)).is_zero() {
        ratio = ratio.saturating_mul(U256::from_str("0x48a170391f7dc42444e8fa2").unwrap())>>128u32;
    }

    // if (tick > 0) ratio = type(uint256).max / ratio;
    if tick>0 {
        ratio = U256::from_big_endian(&[0xff_u8;32]).checked_div(ratio).unwrap();
    }

    // // this divides by 1<<32 rounding up to go from a Q128.128 to a Q128.96.
    // // we then downcast because we know the result always fits within 160 bits due to our tick input constraint
    // // we round up in the division so getTickAtSqrtRatio of the output price is always consistent
    // sqrtPriceX96 = uint160((ratio >> 32) + (ratio % (1 << 32) == 0 ? 0 : 1));
    let (_,remainder)=ratio.div_mod(U256::from(1_u64<<32_u64));
    let tail:U256;
    if remainder.is_zero() {
        tail = U256::zero();
    } else {
        tail = U256::from(1u32);  
    }
    let sqrt_price_x96:U256 = (ratio>>32u32).saturating_add(tail);
    sqrt_price_x96
}

#[cfg(test)]
mod tests {
    use primitives::U256;
    use crate::{core::TickMath::get_tick_at_sqrt_ratio, getSqrtRatioAtTick};

    #[test]
    fn it_get_tick_at_sqrt_ratio(){
        let result = get_tick_at_sqrt_ratio(U256::from(0x429511231231231231231228739fu128));
        println!("result is:{}",result);
        // pub const MIN_SQRT_RATIO:&str = "4295128739";//4295128739;
        // pub const MAX_SQRT_RATIO:&str = "1461446703485210103287273052203988822378723970342"
        let result = get_tick_at_sqrt_ratio(U256::from_dec_str("4295128739").unwrap());
        assert_eq!(result,-887272);
        println!("result is:{}",result);
        let result = get_tick_at_sqrt_ratio(U256::from_dec_str("1461446703485210103287273052203988822378723970342").unwrap());
        assert_eq!(result,887272);
        println!("result is:{}",result);
    }
    
    #[test]
    fn it_get_sqrt_ratio_at_tick(){
        // pub const MIN_SQRT_RATIO:&str = "4295128739";//4295128739;
        // pub const MAX_SQRT_RATIO:&str = "1461446703485210103287273052203988822378723970342"
        let result = getSqrtRatioAtTick(-887272_i32);
        println!("result is:{}",result);
        let result = getSqrtRatioAtTick(887272_i32);
        println!("result is:{}",result);
    }

    #[test]
    fn it_tick_to_price(){
        // pub const MIN_SQRT_RATIO:&str = "4295128739";//4295128739;
        // pub const MAX_SQRT_RATIO:&str = "1461446703485210103287273052203988822378723970342"
        for tick_index in -887272..= 887272{
            println!("tick_index is:{}",tick_index);
            let price = getSqrtRatioAtTick(tick_index);
            println!("price is:{}",price);
            let tick = get_tick_at_sqrt_ratio(price);
            println!("ticker_index,tick is :{},{}",tick_index,tick);
            assert_eq!(tick_index,tick,"tick calculate is not correct!");
        }
    }

    #[test]
    fn it_work(){
        let (a,b) = U256::from(5).div_mod(U256::from(1u64<<32u64));
        println!("a value is:{},b value is:{}",a,b);
        let i1 = -150i64;
        println!("-100 is:{}",i1);
        let i2 = 123i64;
        let i3 = i1|i2;
        println!("i3 is:{}",i3);
        let mut i4:U256 = U256::from(150u32);

        let i5:U256 = U256::from(123u32);
        i4 = !i4;
        i4 = i4.saturating_add(U256::from(1));
        let mut i6:U256 = i4|i5;
        i6= U256::from_big_endian(&[0xff_u8;32]).saturating_sub(i6).saturating_add(U256::from(1));
        println!("i6 is:{}",i6);
    }

    #[test]
    fn just(){
        let mut u = U256::from(1);
        println!("u is :{}",u);
        u.saturating_add(U256::from(1));
        println!("u is :{}",u);
    }
}