use primitives::U256;

use super::{gt, shl, or, shr, mul};

pub fn cal_ratio(msb:&U256,r:&U256,o:&U256,v:&'static str)->(U256,U256){
    // let f := shl(7, gt(r, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
    //         msb := or(msb, f)
    //         r := shr(f, r)
    let v = U256::from(v);
    let f = shl(o,&gt(r,&v));
    let msb = or(msb,&f);
    let r = shr(&f,r); 
    (msb,r)
}



pub fn cal_log(r:&U256,log_2:U256,w:&U256,log_2_is_position:bool)->(U256,U256){
    // r := shr(127, mul(r, r))
    //         let f := shr(128, r)
    //         log_2 := or(log_2, shl(63, f))
    //         r := shr(f, r)
    let r = shr(&U256::from(127),&mul(r,r));
    let f = shr(&U256::from(128),&r);
    
    if log_2_is_position{
        let log_2 = or(&log_2,&shl(w, &f));
        let r = shr(&f,&r);
        return (log_2,r);
    }else{
        // let i5:U256 = U256::from(123u32);
        // i4 = !i4;
        // i4 = i4.saturating_add(U256::from(1));
        // let mut i6:U256 = i4|i5;
        // i6= U256::from_big_endian(&[0xff_u8;32]).saturating_sub(i6).saturating_add(U256::from(1));
        let log_2 = !log_2;
        let log_2 = log_2.saturating_add(U256::from(1u32));
        let log_2 = or(&log_2,&shl(w, &f));
        let log_2 = U256::from_big_endian(&[0xff_u8;32]).saturating_sub(log_2).saturating_add(U256::from(1));
        let r = shr(&f,&r);
        return (log_2,r);
    }

    
}


#[cfg(test)]
mod tests {
    use primitives::U256;

    use super::cal_ratio;

    #[test]
    fn it_works() {
        let s = U256::from("FF");
        println!("test result is:{:?}",s.to_string());
        println!("test hex result is:{:?}",hex::decode("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"));
    }

    #[test]
    fn test_result() {
        let mut r = U256::from("11");
        let mut msb = U256::from("22");
        let o =U256::from("2");
        let v ="0xF";
        (msb,r) = cal_ratio(&r,&msb,&o,v);
        println!("test msb is:{:?}",msb.to_string());
        println!("test r is:{:?}",r.to_string());
    }

    #[test]
    fn it_work(){
        let result = U256::from_dec_str("100").unwrap();
        println!("result is:{:?}",result);
        let u1:u32 = 12345;
        let u2:u32=45678;
        let u3:u32 = u1|u2;
        println!("result is:{:?}",u3);
        let i1:i32 = 12345;
        let i2:i32 = 45678;
        let i3:i32 = i1|i2;
        println!("result is:{:?}",i3);
    }

    #[test]
    fn my_temp_test(){
        let result = U256::from(1);
        let result = result << 128;
        println!("result is:{:?}",result);
    }
}