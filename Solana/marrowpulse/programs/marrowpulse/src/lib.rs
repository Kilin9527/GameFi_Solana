use anchor_lang::prelude::*;

pub mod instructions;
pub mod constants;
pub mod error;

use instructions::*;
use error::ErrorCode;

use light_sdk::{
    cpi::CpiSigner,
    derive_light_cpi_signer,
};

use light_sdk::instruction:: {
    ValidityProof,
    PackedAddressTreeInfo,
};

declare_id!("6SYf4dsV62UpunfNDNgSTTuxiT1y6FPJHathQXgStwqa");

pub const LIGHT_CPI_SIGNER: CpiSigner =
    derive_light_cpi_signer!("6SYf4dsV62UpunfNDNgSTTuxiT1y6FPJHathQXgStwqa");

#[program]
pub mod marrowpulse {
    use super::*;

    pub fn ipfs_config_initialize(ctx: Context<IpfsConfigInitializeAccounts>, feature_id: String, config_uri: String) -> Result<()>{
        handler_ipfs_config_initialize(ctx, feature_id, config_uri)
    }

    pub fn ipfs_config_update(ctx: Context<IpfsConfigUpdateAccounts>, feature_id: String, config_uri: String) -> Result<()>{
        handler_ipfs_config_update(ctx, feature_id, config_uri)
    }

    pub fn authority_registry_initialize(ctx: Context<AuthorityRegistryInitializeAccounts>, feature_id: String) -> Result<()>{
        handler_authority_registry_initialize(ctx, feature_id)
    }

    pub fn authority_registry_propose(ctx: Context<AuthorityRegistryProposeAccounts>, feature_id: String) -> Result<()>{
        handler_authority_registry_propose(ctx, feature_id)
    }

    pub fn authority_registry_accept(ctx: Context<AuthorityRegistryAcceptAccounts>, feature_id: String) -> Result<()>{
        handler_authority_registry_accept(ctx, feature_id)
    }

    pub fn authority_registry_cancel(ctx: Context<AuthorityRegistryCancelAccounts>, feature_id: String) -> Result<()>{
        handler_authority_registry_cancel(ctx, feature_id)
    }

    pub fn equipment_config_initialize(ctx: Context<EquipmentConfigInitializeAccounts>, feature_id: String) -> Result<()>{
        handler_equipment_config_initialize(ctx, feature_id)
    }

    pub fn equipment_config_update(ctx: Context<EquipmentConfigUpdateAccounts>, feature_id: String, pause: Option<bool>, mint_equipment_enabled: Option<bool>) -> Result<()>{
        handler_equipment_config_update(ctx, feature_id, pause, mint_equipment_enabled)
    }

    pub fn generate_artifact_box<'info>(
        ctx: Context<'_, '_, '_, 'info, GenerateArtifactBoxAccounts<'info>>, 
        proof_bytes: Vec<u8>, 
        address_tree_info_bytes: Vec<u8>, 
        output_state_tree_index: u8, 
        artifact_id: [u8; 32],
        commitment: [u8; 32]) -> Result<()>{
            let proof = ValidityProof::try_from_slice(&proof_bytes)
                .map_err(|_| error!(ErrorCode::InvalidEquipmentBoxParamProofConvert))?;
            let address_tree_info = PackedAddressTreeInfo::try_from_slice(&address_tree_info_bytes)
                .map_err(|_| error!(ErrorCode::InvalidEquipmentBoxParamAddressTreeConvert))?;

            handler_generate_artifact_box(ctx, proof, address_tree_info, output_state_tree_index, artifact_id, commitment)
    }
}
