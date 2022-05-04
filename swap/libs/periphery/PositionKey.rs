#![allow(non_snake_case)]

use primitives::{Int24, Address};

/// @dev Returns the key of the position in the core library
pub fn compute(
    owner:Address,
    tickLower:Int24,
    tickUpper:Int24,
) ->Vec<u8> {
    // return keccak256(abi.encodePacked(owner, tickLower, tickUpper));
    scale::Encode::encode(&(owner,tickLower,tickUpper))
}