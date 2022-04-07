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

pub fn cal_log(r:&U256,log_2:&U256,w:&U256)->(U256,U256){
    // r := shr(127, mul(r, r))
    //         let f := shr(128, r)
    //         log_2 := or(log_2, shl(63, f))
    //         r := shr(f, r)
    let r = shr(&U256::from("7F"),&mul(r,r));
    let f = shr(&U256::from("80"),&r);
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
        let result = U256::from_dec_str("1461446703485210103287273052203988822378723970342");
        println!("result is:{:?}",result);
    }
}