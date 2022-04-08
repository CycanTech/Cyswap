#![cfg_attr(not(feature = "std"), no_std)]
pub mod assembly;

use primitives::U256;

pub fn shl(o:&U256,v:&U256)->U256{
    v<<o
}

pub fn gt(o:&U256,v:&U256)->U256{
    if o.gt(v){
        U256::from("1")
    }else{
        U256::from("0")
    }
}

pub fn or(o:&U256,v:&U256)->U256{
    *o|*v
}

pub fn shr(o:&U256,v:&U256)->U256{
    v>>o
}

pub fn mul(o:&U256,v:&U256)->U256{
    o.saturating_mul(*v)
}

#[cfg(test)]
mod tests {
    use primitives::U256;

    use crate::assembly::{shl, shr, gt, or, mul};

    #[test]
    fn test_shl() {
        let o = U256::from("2");
        println!("o is:{:?}",o.to_string());
        let v = U256::from("A");
        println!("v is:{:?}",v.to_string());
        assert!(shl(&o,&v)==U256::from("28"));
        println!("test result is:{:?}",shl(&o,&v).to_string());
    }

    #[test]
    fn test_shr() {
        let o = U256::from("2");
        println!("o is:{:?}",o.to_string());
        let v = U256::from("A");
        println!("v is:{:?}",v.to_string());
        assert!(shr(&o,&v)==U256::from("2"));
        println!("test result is:{:?}",shr(&o,&v).to_string());
    }

    #[test]
    fn test_gt() {
        let o = U256::from("2");
        println!("o is:{:?}",o.to_string());
        let v = U256::from("A");
        println!("v is:{:?}",v.to_string());
        assert!(gt(&o,&v)==U256::from("0"));
        assert!(gt(&v,&o)==U256::from("1"));
        println!("test result is:{:?}",gt(&o,&v).to_string());
    }

    #[test]
    fn test_or() {
        let o = U256::from("2");
        println!("o is:{:?}",o.to_string());
        let v = U256::from("A");
        println!("v is:{:?}",v.to_string());
        assert_eq!(or(&o,&v),U256::from("A"));
        println!("test result is:{:?}",or(&o,&v).to_string());
    }

    #[test]
    fn test_mul() {
        let o = U256::from("2");
        println!("o is:{:?}",o.to_string());
        let v = U256::from("A");
        println!("v is:{:?}",v.to_string());
        assert_eq!(mul(&o,&v),U256::from("14"));
        println!("test result is:{:?}",mul(&o,&v).to_string());
    }
    // #[test]
    // fn test_result() {
    //     let mut r = U256::from("1");
    //     let mut msb = U256::from("0");
    //     let o =U256::from("7");
    //     let v ="FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
    //     (msb,r) = cal_ratio(&r,&msb,&o,v);
    //     println!("test result is2:{:?}",msb.to_string());
    // }
}