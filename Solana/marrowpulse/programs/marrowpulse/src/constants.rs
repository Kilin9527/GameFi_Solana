use anchor_lang::prelude::*;

#[constant]
pub const GENESIS_ADMIN: Pubkey = pubkey!("3iyS9TmCgCFCisUjjnKN1hQEVDhXFrk2b5X5cg9M92gi");

// Used in authority registry initialization.
#[constant]
pub const AUTHORITY_REGISTRY_SEED: &'static [u8] = b"authority_registry";
#[constant]
pub const PROPOSAL_DURATION: i64 = 86400;

// Used in ipfs config.
#[constant]
pub const IPFS_CONFIG_SEED: &'static [u8] = b"ipfs_config";

// Used in equipment config.
#[constant]
pub const EQUIPMENT_CONFIG_SEED: &'static [u8] = b"equipment_config";

// Used in equipment box.
#[constant]
pub const EQUIPMENT_BOX_SEED: &'static [u8] = b"equipment_box";