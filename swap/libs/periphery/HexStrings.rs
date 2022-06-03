#![allow(non_snake_case)]
use primitives::U256;
use ink_prelude::string::ToString;
use ink_prelude::string::String;
use ink_prelude::format;

/// @notice Converts a `uint256` to its ASCII `string` hexadecimal representation with fixed length.
/// @dev Credit to Open Zeppelin under MIT license https://github.com/OpenZeppelin/openzeppelin-contracts/blob/243adff49ce1700e0ecb99fe522fb16cff1d1ddc/contracts/utils/Strings.sol#L55
pub fn toHexString(value: U256, length: usize) -> String {
    let result = format!("{:x}", value);
    let len = result.len();
    let mut result = result[len - length..].to_string();
    result.insert_str(0, "0x");
    result.clone()
}

pub fn toHexStringNoPrefix(value: U256, length: usize) -> String {
    let result = format!("{:x}", value);
    let len = result.len();
    let result = &result[len - length..];
    result.to_string()
}

#[cfg(test)]
mod tests {
    use primitives::U256;

    #[test]
    fn it_works() {
        let test_hex_str = super::toHexStringNoPrefix(U256::from("abc127121273126ef"), 3);
        println!("{:?}", test_hex_str);
        let test_hex_str = super::toHexString(U256::from("abc127121273126ef"), 4);
        println!("{:?}", test_hex_str);
    }
}

