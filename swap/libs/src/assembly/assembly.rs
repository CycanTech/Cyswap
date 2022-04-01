#![cfg_attr(not(feature = "std"), no_std)]

use primitives::U256;

use super::{gt, shl, or, shr, mul};

pub fn cal_ratio(msb:&U256,r:&U256,o:&U256,v:&str)->(U256,U256){
    // let f := shl(7, gt(r, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
    //         msb := or(msb, f)
    //         r := shr(f, r)
    let v = U256::from_big_endian(&hex::decode(v).unwrap());
    let f = shl(o,&gt(r,&v));
    let msb = or(msb,&f);
    let r = shr(&f,r); 
    (msb,r)
}

pub fn cal_log(r:&U256,log_2:&U256,w:&U256)->(U256,U256){
    // r := shr(127, mul(r, r))
    //         let f := shr(128, r)
    //         log_2 := or(log_2, shl(63, f))
    //         r := shr(f, r)
    let r = shr(&U256::from("127"),&mul(r,r));
    let f = shr(&U256::from("128"),&r);
    let log_2 = or(&log_2,&shl(w, &f));
    let r = shr(&f,&r);
    (log_2,r)
}


#[cfg(test)]
mod tests {
    use primitives::U256;

    use super::cal_ratio;

    #[test]
    fn it_works() {
        let s = U256::from_big_endian(&[255u8;16]);
        println!("test result is:{:?}",s.to_string());
        let s = U256::from_little_endian(&[255u8;16]);
        println!("test result is:{:?}",s.to_string());
        println!("test hex result is:{:?}",hex::decode("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"));
    }

    #[test]
    fn test_result() {
        let mut r = U256::from("1");
        let mut msb = U256::from("0");
        let o =U256::from("7");
        let v ="FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        (msb,r) = cal_ratio(&r,&msb,&o,v);
        println!("test result is2:{:?}",msb.to_string());
    }
}