#![allow(non_snake_case)]

use ink_env::AccountId;
use primitives::{Address, U256};
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
pub fn hasMultiplePools(path: String) -> bool {
    return path.len() >= MULTIPLE_POOLS_MIN_LENGTH;
}

/// @notice Returns the number of pools in the path
/// @param path The encoded swap path
/// @return The number of pools in the path
pub fn numPools(path: String) -> usize {
    // Ignore the first token address. From then on every fee and token offset indicates a pool.
    (path.len() - ADDR_SIZE) / NEXT_OFFSET
}

/// @notice Decodes the first pool in path
/// @param path The bytes encoded swap path
/// @return tokenA The first token of the given pool
/// @return tokenB The second token of the given pool
/// @return fee The fee level of the pool
pub fn decodeFirstPool(path: String) -> (Address,Address, usize) {
    let tokenA = path.toAddress(0);
    let fee = path.toUint24(ADDR_SIZE);
    let tokenB = path.toAddress(NEXT_OFFSET);
    (tokenA,tokenB, fee)
}

/// @notice Gets the segment corresponding to the first pool in the path
/// @param path The bytes encoded swap path
/// @return The segment containing all data necessary to target the first pool in the path
pub fn getFirstPool(path:String) -> String {
    let mut path = path.clone();
    path.truncate(POP_OFFSET);
    path
}

/// @notice Skips a token + fee element from the buffer and returns the remainder
/// @param path The swap path
/// @return The remaining token + fee elements in the path
pub fn skipToken(path:String) -> (String) {
    // return path.slice(NEXT_OFFSET, path.length - NEXT_OFFSET);
    let left_str = String::from_utf8(path.as_bytes()[NEXT_OFFSET.. path.len() - NEXT_OFFSET].into()).expect("exchange str to String failed");
    left_str
}

pub trait BytesLib {
    fn toAddress(&self, _start: usize) -> AccountId;
    fn toUint24(&self, _start: usize) -> usize;
}

impl BytesLib for String {
    fn toAddress(&self, _start: usize) -> AccountId {
        // require(_start + 20 >= _start, 'toAddress_overflow');
        assert!(_start + 20 >= _start, "toAddress_overflow");
        // require(_bytes.length >= _start + 20, 'toAddress_outOfBounds');
        assert!(self.len() >= _start + 20, "toAddress_outOfBounds");
        // address tempAddress;
        // assembly {
        //     tempAddress := div(mload(add(add(_bytes, 0x20), _start)), 0x1000000000000000000000000)
        // }
        let tempAddress: [u8; ADDR_SIZE] = self.as_bytes()[_start.._start + ADDR_SIZE]
            .try_into()
            .expect("exchange &[u8] to [u8;32] error!");
        // return tempAddress;
        AccountId::from(tempAddress)
    }

    fn toUint24(&self, _start: usize) -> usize {
        // require(_start + 3 >= _start, 'toUint24_overflow');
        assert!(_start + FEE_SIZE >= _start, "toUint24_overflow");
        // require(_bytes.length >= _start + 3, 'toUint24_outOfBounds');
        assert!(self.len() >= _start + FEE_SIZE, "toUint24_outOfBounds");
        // uint24 tempUint;

        // assembly {
        //     tempUint := mload(add(add(_bytes, 0x3), _start))
        // }
        let temp_num = &self.as_bytes()[_start.._start + FEE_SIZE];
        let temp_str =
            String::from_utf8(temp_num.into()).expect("exchange the [u8] to string error!");
        U256::from_str_radix(&temp_str, 16)
            .expect("u256 exchange error!")
            .as_usize()
    }
}
