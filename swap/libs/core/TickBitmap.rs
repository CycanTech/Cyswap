#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use ink_storage::Mapping;
use primitives::{Int24, U256, Uint256};

use super::BitMath;


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

/// @notice Returns the next initialized tick contained in the same word (or adjacent word) as the tick that is either
/// to the left (less than or equal to) or right (greater than) of the given tick
/// @param self The mapping in which to compute the next initialized tick
/// @param tick The starting tick
/// @param tickSpacing The spacing between usable ticks
/// @param lte Whether to search for the next initialized tick to the left (less than or equal to the starting tick)
/// @return next The next initialized or uninitialized tick up to 256 ticks away from the current tick
/// @return initialized Whether the next tick is initialized, as the function only searches within up to 256 ticks
pub fn nextInitializedTickWithinOneWord(
      tickMapMap:&mut Mapping<i16, Uint256>,
     tick:Int24,
     tickSpacing:Int24,
     lte:bool,
) -> (Int24 , bool) {
    let next;
    let initialized;
    // int24 compressed = tick / tickSpacing;
    let mut compressed:Int24 = tick/tickSpacing;
    // if (tick < 0 && tick % tickSpacing != 0) compressed--; // round towards negative infinity
    if tick < 0 && tick % tickSpacing != 0 {
        compressed -=1; // round towards negative infinity
    }
    

    // if (lte) {
    //     (int16 wordPos, uint8 bitPos) = position(compressed);
    //     // all the 1s at or to the right of the current bitPos
    //     uint256 mask = (1 << bitPos) - 1 + (1 << bitPos);
    //     uint256 masked = self[wordPos] & mask;
    if lte {
       let ( wordPos, bitPos): (i16, u8) = position(compressed);
        // all the 1s at or to the right of the current bitPos
        let mask:U256 = (U256::from(1) << bitPos) - 1 + (U256::from(1) << bitPos);
        let masked:U256 = tickMapMap.get(wordPos).expect("get wordPos error!").value & mask;

    //     // if there are no initialized ticks to the right of or at the current tick, return rightmost in the word
    //     initialized = masked != 0;
      initialized = !masked.is_zero();
    //     // overflow/underflow is possible, but prevented externally by limiting both tickSpacing and tick
    //     next = initialized
    //         ? (compressed - int24(bitPos - BitMath.mostSignificantBit(masked))) * tickSpacing
    //         : (compressed - int24(bitPos)) * tickSpacing;
        next = if initialized{
            (compressed - Int24::from(bitPos - BitMath::mostSignificantBit(masked))) * tickSpacing
        }else{
            (compressed - Int24::from(bitPos)) * tickSpacing
        };
    } else {
    //     // start from the word of the next tick, since the current tick state doesn't matter
    //     (int16 wordPos, uint8 bitPos) = position(compressed + 1);
        let (wordPos, bitPos) = position(compressed + 1);
    //     // all the 1s at or to the left of the bitPos
    //     uint256 mask = ~((1 << bitPos) - 1);
       let  mask:U256 = !((U256::from(1) << bitPos) - 1);
    //     uint256 masked = self[wordPos] & mask;
    let masked:U256 = tickMapMap.get(wordPos).expect("get wordPos error!").value & mask;
    //     // if there are no initialized ticks to the left of the current tick, return leftmost in the word
    //     initialized = masked != 0;
    initialized = masked != U256::zero();
    //     // overflow/underflow is possible, but prevented externally by limiting both tickSpacing and tick
    //     next = initialized
    //         ? (compressed + 1 + int24(BitMath.leastSignificantBit(masked) - bitPos)) * tickSpacing
    //         : (compressed + 1 + int24(type(uint8).max - bitPos)) * tickSpacing;
        next = if initialized{
            (compressed + 1 + Int24::from(BitMath::leastSignificantBit(masked) - bitPos)) * tickSpacing
        }else{
            (compressed + 1 + Int24::from(u8::MAX - bitPos)) * tickSpacing
        };
    }
    (next,initialized)
}
