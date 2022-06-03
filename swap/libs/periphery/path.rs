#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use primitives::{Address};
use ink_prelude::vec::Vec;

/// @dev The length of the bytes encoded address
// uint256 private constant ADDR_SIZE = 20;
const ADDR_SIZE: usize = 32;
/// @dev The length of the bytes encoded fee
// uint256 private constant FEE_SIZE = 3;
const FEE_SIZE: usize = 4;

/// @dev The offset of a single token address and pool fee
// uint256 private constant NEXT_OFFSET = ADDR_SIZE + FEE_SIZE;
const NEXT_OFFSET: usize = ADDR_SIZE + FEE_SIZE;
/// @dev The offset of an encoded pool key
// uint256 private constant POP_OFFSET = NEXT_OFFSET + ADDR_SIZE;
const POP_OFFSET: usize = NEXT_OFFSET + ADDR_SIZE;
/// @dev The minimum length of an encoding that contains 2 or more pools
// uint256 private constant MULTIPLE_POOLS_MIN_LENGTH = POP_OFFSET + NEXT_OFFSET;
const MULTIPLE_POOLS_MIN_LENGTH: usize = POP_OFFSET + NEXT_OFFSET;

/// @notice Returns true iff the path contains two or more pools
/// @param path The encoded swap path
/// @return True if path contains two or more pools, otherwise false
pub fn hasMultiplePools(path: &Vec<u8>) -> bool {
    return path.len() >= MULTIPLE_POOLS_MIN_LENGTH;
}

/// @notice Returns the number of pools in the path
/// @param path The encoded swap path
/// @return The number of pools in the path
pub fn numPools(path: &Vec<u8>) -> usize {
    // Ignore the first token address. From then on every fee and token offset indicates a pool.
    (path.len() - ADDR_SIZE) / NEXT_OFFSET
}

/// @notice Decodes the first pool in path
/// @param path The bytes encoded swap path
/// @return tokenA The first token of the given pool
/// @return tokenB The second token of the given pool
/// @return fee The fee level of the pool
pub fn decodeFirstPool(path: &Vec<u8>) -> (Address,u32,Address) {
    let tokenA = path.toAddress(0);
    let fee = path.toUint24(ADDR_SIZE);
    let tokenB = path.toAddress(NEXT_OFFSET);
    (tokenA,fee,tokenB)
}

/// @notice Gets the segment corresponding to the first pool in the path
/// @param path The bytes encoded swap path
/// @return The segment containing all data necessary to target the first pool in the path
pub fn getFirstPool(path:&Vec<u8>) -> Vec<u8> {
    let mut path = path.clone();
    path.truncate(POP_OFFSET);
    path
}

/// @notice Skips a token + fee element from the buffer and returns the remainder
/// @param path The swap path
/// @return The remaining token + fee elements in the path
pub fn skipToken(path:&Vec<u8>) -> Vec<u8> {
    // return path.slice(NEXT_OFFSET, path.length - NEXT_OFFSET);
    let left_str = &path[NEXT_OFFSET.. path.len() - NEXT_OFFSET];
    left_str.to_vec()
}

// pub fn formant_fee(fee:u32)->String{
//     let result = format!("{:0FEE_SIZE$}",fee.to_string());
//     result
// }


pub trait BytesLib {
    fn toAddress(&self, _start: usize) -> Address;
    fn toUint24(&self, _start: usize) -> u32;
}

impl BytesLib for Vec<u8> {

    

    fn toAddress(&self, _start: usize) -> Address {
        // require(_start + 20 >= _start, 'toAddress_overflow');
        assert!(_start + 20 >= _start, "toAddress_overflow");
        // require(_bytes.length >= _start + 20, 'toAddress_outOfBounds');
        assert!(self.len() >= _start + 20, "toAddress_outOfBounds");
        // address tempAddress;
        // assembly {
        //     tempAddress := div(mload(add(add(_bytes, 0x20), _start)), 0x1000000000000000000000000)
        // }
        let tempAddress: [u8; ADDR_SIZE] = self.as_slice()[_start.._start + ADDR_SIZE]
            .try_into()
            .expect("exchange &[u8] to [] error!");
        // return tempAddress;
        Address::from(tempAddress)
    }

    fn toUint24(&self, _start: usize) -> u32 {
        // require(_start + 3 >= _start, 'toUint24_overflow');
        assert!(_start + FEE_SIZE >= _start, "toUint24_overflow");
        // require(_bytes.length >= _start + 3, 'toUint24_outOfBounds');
        assert!(self.len() >= _start + FEE_SIZE, "toUint24_outOfBounds");
        // uint24 tempUint;

        // assembly {
        //     tempUint := mload(add(add(_bytes, 0x3), _start))
        // }
        let mut temp_num = &self[_start.._start + FEE_SIZE];
        let output = scale::Decode::decode(&mut temp_num).expect("u8 array to u32 error!");
        output
    }
}

#[cfg(test)]
mod tests {
    use brush::traits::AccountId;

    use crate::periphery::path::{decodeFirstPool};


    #[test]
    fn it_works() {
        
        let a:AccountId = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32].into();
        let b:AccountId = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,33].into();
        let fee:u32 = 500;
        let s1 = scale::Encode::encode(&(a, fee, b));
        println!("s1 is:{:?}",s1);
        let len = s1.len();
        println!("len is:{:?}",len);
        let (a1,f1,a2) = decodeFirstPool(&s1);
        println!("a1 is:{:?}",a1);
        println!("a2 is:{:?}",a2);
        println!("f1 is:{:?}",f1);
    }

}