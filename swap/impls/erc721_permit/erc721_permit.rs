pub use super::data::*;
pub use crate::traits::periphery::erc721_permit::*;

use openbrush::{contracts::traits::psp34::Id, traits::AccountId};
use ink_env::{
    hash::CryptoHash,
    hash::{Blake2x256, HashOutput, Sha2x256},
    DefaultEnvironment,
};
use ink_prelude::string::String;
use ink_prelude::vec;
use scale::Encode;
use openbrush::contracts::traits::psp34::PSP34;
/// @inheritdoc IERC721Permit
/// @dev Value is equal to keccak256("Permit(address spender,uint256 tokenId,uint256 nonce,uint256 deadline)");
const PERMIT_TYPEHASH: &'static str =
    "49ecf333e5b8c95c40fdafc95c1ad136e8914a8fb55e9dc8bb01eaa83a2df9ad";

impl<T: ERC721PermitStorage+PSP34> IERC721Permit for T {
    /// @notice The domain separator used in the permit signature
    /// @return The domain seperator used in encoding of permit signature
    default fn DOMAIN_SEPARATOR(&self) -> [u8; 32] {
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
        let mut s_vec = "8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f".encode();
        let mut name_hash_vec = ERC721PermitStorage::get(self).nameHash.encode();
        let mut version_hash_vec = ERC721PermitStorage::get(self).versionHash.encode();
        let mut chain_id_vec = vec![1u8];
        let mut address_vec = ink_env::account_id::<DefaultEnvironment>().encode();
        s_vec.append(&mut name_hash_vec);
        s_vec.append(&mut version_hash_vec);
        s_vec.append(&mut chain_id_vec);
        s_vec.append(&mut address_vec);
        let mut result = [0u8; 32];

        Blake2x256::hash(&s_vec, &mut result);
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
        &mut self,
        spender: AccountId,
        tokenId: Id,
        deadline: u64,
        v: u8,
        r: String,
        s: String,
    ) {
        // require(_blockTimestamp() <= deadline, 'Permit expired');
        assert!(
            ink_env::block_timestamp::<DefaultEnvironment>() <= deadline,
            "Permit expired"
        );
        // bytes32 digest =
        //     keccak256(
        //         abi.encodePacked(
        //             '\x19\x01',
        //             DOMAIN_SEPARATOR(),
        //             keccak256(abi.encode(PERMIT_TYPEHASH, spender, tokenId, _getAndIncrementNonce(tokenId), deadline))
        //         )
        //     );
        let mut buffer = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
        let encodable = scale::Encode::encode(&(
            hex::decode(PERMIT_TYPEHASH).unwrap(),
            spender,
            tokenId.clone(),
            self._getAndIncrementNonce(tokenId.clone()),
            deadline,
        ));
        ink_env::hash_encoded::<Sha2x256, _>(&encodable, &mut buffer);
        let encodable = scale::Encode::encode(&("\x19\x01", self.DOMAIN_SEPARATOR(), buffer));
        ink_env::hash_encoded::<Sha2x256, _>(&encodable, &mut buffer);

        // address owner = ownerOf(tokenId);
        let owner = <Self as PSP34>::owner_of(&self,tokenId.clone()).expect("get owner by tokenId failed");
        // require(spender != owner, 'ERC721Permit: approval to current owner');
        assert!(spender!= owner,"ERC721Permit: approval to current owner");
        // if (Address.isContract(owner)) {
        //     require(IERC1271(owner).isValidSignature(digest, abi.encodePacked(r, s, v)) == 0x1626ba7e, 'Unauthorized');
        // } else {
        //     address recoveredAddress = ecrecover(digest, v, r, s);
        //     require(recoveredAddress != address(0), 'Invalid signature');
        //     require(recoveredAddress == owner, 'Unauthorized');
        // }

        <Self as PSP34>::allowance(&self,owner,spender,Some(tokenId.clone()));
        // _approve(spender, tokenId);
    }

    // implement by NonfungiblePositionManager
    default fn _getAndIncrementNonce(&mut self, _tokenId: Id) -> u128 {
        unimplemented!();
    }
}
