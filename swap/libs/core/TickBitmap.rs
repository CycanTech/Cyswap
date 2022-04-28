#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use primitives::{Int24, U256};

/// @notice Flips the initialized state for a given tick from false to true, or vice versa
/// @param self The mapping in which to flip the tick
/// @param tick The tick to flip
/// @param tickSpacing The spacing between usable ticks
pub fn flipTick(tick: Int24, tickSpacing: Int24) -> (i16, U256) {
    assert!(tick % tickSpacing == 0); // ensure that the tick is spaced
    let (wordPos, bitPos) = position(tick / tickSpacing);
    let mask: U256 = U256::from(1) << bitPos;
    (wordPos, mask)
    // self[wordPos] ^= mask;
}

/// @notice Computes the position in the mapping where the initialized bit for a tick lives
/// @param tick The tick for which to compute the position
/// @return wordPos The key in the mapping containing the word in which the bit is stored
/// @return bitPos The bit position in the word where the flag is stored
fn position(tick: Int24) -> (i16, u8) {
    let wordPos = i16::try_from(tick >> 8).unwrap();
    let bitPos = u8::try_from(tick % 256).unwrap();
    (wordPos, bitPos)
}
