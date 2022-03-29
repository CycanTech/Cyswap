#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

use brush_derive::declare_derive_storage_trait;

#[cfg(feature = "pool_initial")]
declare_derive_storage_trait!(derive_pool_initial_storage, PoolInitializeStorage, PoolInitializeStorageField);

#[cfg(feature = "erc721_permit")]
declare_derive_storage_trait!(derive_erc721_permit_storage, ERC721PermitStorage, ERC721PermitStorageField);
