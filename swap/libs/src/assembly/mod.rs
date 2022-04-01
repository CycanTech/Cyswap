#![cfg_attr(not(feature = "std"), no_std)]
pub mod assembly;

use primitives::U256;

pub fn shl(o:&U256,v:&U256)->U256{
    v<<o
}

pub fn gt(o:&U256,v:&U256)->U256{
    if o.ge(v){
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