#![allow(non_snake_case)]

use ink_env::hash::{HashOutput, Sha2x256};
use ink_storage::traits::{SpreadAllocate, SpreadLayout};
use primitives::{Address, Uint24};
use scale::{Decode, Encode};


const POOL_INIT_CODE_HASH: &str =
    "0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54";

#[derive(Default, Clone, Decode, Encode, Debug, SpreadAllocate, SpreadLayout)]
pub struct PoolKey {
    pub token0: Address,
    pub token1: Address,
    pub fee: Uint24,
}

/// @notice Returns PoolKey: the ordered tokens with the matched fee levels
/// @param tokenA The first token of a pool, unsorted
/// @param tokenB The second token of a pool, unsorted
/// @param fee The fee level of the pool
/// @return Poolkey The pool details with ordered token0 and token1 assignments
fn getPoolKey(tokenA: Address, tokenB: Address, fee: Uint24) -> PoolKey {
    let token0: Address;
    let token1: Address;

    if tokenA > tokenB {
        token0 = tokenB;
        token1 = tokenA;
    } else {
        token0 = tokenA;
        token1 = tokenB;
    }
    return PoolKey {
        token0,
        token1,
        fee,
    };
}

/// @notice Deterministically computes the pool address given the factory and PoolKey
/// @param factory The Uniswap V3 factory contract address
/// @param key The PoolKey
/// @return pool The contract address of the V3 pool
pub fn computeAddress(factory: Address, key: PoolKey) -> Address {
    // require(key.token0 < key.token1);
    assert!(key.token0 < key.token1, "token1 bt token1");
    let pool: Address;
    let mut hash = <Sha2x256 as HashOutput>::Type::default();
    let encodable = (key.token0, key.token1, key.fee);
    ink_env::hash_encoded::<Sha2x256, _>(&encodable, &mut hash);
    let encodable = ("ff", factory, hash, POOL_INIT_CODE_HASH);
    ink_env::hash_encoded::<Sha2x256, _>(&encodable, &mut hash);
    pool = hash.into();
    // pool = address(
    //     uint256(
    //         keccak256(
    //             abi.encodePacked(
    //                 hex'ff',
    //                 factory,
    //                 keccak256(abi.encode(key.token0, key.token1, key.fee)),
    //                 POOL_INIT_CODE_HASH
    //             )
    //         )
    //     )
    // );
    pool
}
