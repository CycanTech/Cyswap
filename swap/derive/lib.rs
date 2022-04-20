#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

use brush_derive::declare_derive_storage_trait;

#[cfg(feature = "pool_initial")]
declare_derive_storage_trait!(derive_pool_initial_storage, PoolInitializeStorage, PoolInitializeStorageField);

#[cfg(feature = "erc721_permit")]
declare_derive_storage_trait!(derive_erc721_permit_storage, ERC721PermitStorage, ERC721PermitStorageField);

#[cfg(feature = "psp34_base")]
declare_derive_storage_trait!(derive_psp34_base_storage, PSP34BaseStorage, PSP34BaseStorageField);

#[cfg(feature = "pool_deployer")]
declare_derive_storage_trait!(derive_pool_deployer_storage, PoolDeployerStorage, PoolDeployerStorageField);

declare_derive_storage_trait!(derive_position_descriptor_storage, PositionDescriptorStorage, PositionDescriptorStorageField);

#[cfg(feature = "position_manager")]
declare_derive_storage_trait!(derive_position_storage, PositionStorage, PositionStorageField);
