pub use super::data::*;
pub use crate::traits::periphery::erc721_permit::*;

use brush::{
    contracts::{
        traits::{
            psp34::Id,
        },
    },
    modifiers,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Timestamp,
        ZERO_ADDRESS,
    },
};
use ink_env::{DefaultEnvironment, hash::Blake2x256,hash::CryptoHash};
use primitives::U256;
use scale::Encode;
use ink_prelude::string::String;
use ink_prelude::vec;

impl<T:ERC721PermitStorage> IERC721Permit for T{
    default fn PERMIT_TYPEHASH(&self) ->String{
        "0x49ecf333e5b8c95c40fdafc95c1ad136e8914a8fb55e9dc8bb01eaa83a2df9ad".into()
    }

    /// @notice The domain separator used in the permit signature
    /// @return The domain seperator used in encoding of permit signature
    default fn DOMAIN_SEPARATOR(&self) -> [u8;32]{
        // return
        //     keccak256(
        //         abi.encode(
        //             // keccak256('EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)')
        //             0x8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f,
        //             nameHash,
        //             versionHash,
        //             ChainId.get(),
        //             address(this)
        //         )
        //     );
        let mut s_vec= "0x8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f".encode();
        let mut name_hash_vec = ERC721PermitStorage::get(self).nameHash.encode();
        let mut version_hash_vec = ERC721PermitStorage::get(self).versionHash.encode();
        let mut chain_id_vec = vec!(1u8);
        let mut address_vec = ink_env::account_id::<DefaultEnvironment>().encode();
        s_vec.append(&mut name_hash_vec);
        s_vec.append(&mut version_hash_vec);
        s_vec.append(&mut chain_id_vec);
        s_vec.append(&mut address_vec);
        let mut result = [0u8;32];

        Blake2x256::hash(&s_vec,&mut result);
        result
    }

    /// @notice Approve of a specific token ID for spending by spender via signature
    /// @param spender The account that is being approved
    /// @param tokenId The ID of the token that is being approved for spending
    /// @param deadline The deadline timestamp by which the call must be mined for the approve to work
    /// @param v Must produce valid secp256k1 signature from the holder along with `r` and `s`
    /// @param r Must produce valid secp256k1 signature from the holder along with `v` and `s`
    /// @param s Must produce valid secp256k1 signature from the holder along with `r` and `v`
    default fn permit(
        &self,
        spender:AccountId,
        token_id:U256,
        deadline:U256,
        v:u8,
        r:String,
        s:String 
    ){
        // require(_blockTimestamp() <= deadline, 'Permit expired');

        // bytes32 digest =
        //     keccak256(
        //         abi.encodePacked(
        //             '\x19\x01',
        //             DOMAIN_SEPARATOR(),
        //             keccak256(abi.encode(PERMIT_TYPEHASH, spender, tokenId, _getAndIncrementNonce(tokenId), deadline))
        //         )
        //     );
        // address owner = ownerOf(tokenId);
        // require(spender != owner, 'ERC721Permit: approval to current owner');

        // if (Address.isContract(owner)) {
        //     require(IERC1271(owner).isValidSignature(digest, abi.encodePacked(r, s, v)) == 0x1626ba7e, 'Unauthorized');
        // } else {
        //     address recoveredAddress = ecrecover(digest, v, r, s);
        //     require(recoveredAddress != address(0), 'Invalid signature');
        //     require(recoveredAddress == owner, 'Unauthorized');
        // }

        // _approve(spender, tokenId);
        let block_timestamp = ink_env::block_timestamp::<DefaultEnvironment>();
        assert!(block_timestamp<deadline.as_u64(),"Permit expired");
        // TODO finish

    }
}