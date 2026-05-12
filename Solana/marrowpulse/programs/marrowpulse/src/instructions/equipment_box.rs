use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::ErrorCode;
use light_sdk::{
    sdk_types::*,
    LightDiscriminator,
    account::LightAccount,
    address::v2::derive_address,
    constants::ADDRESS_TREE_V2,
    instruction:: {
        ValidityProof,
        PackedAddressTreeInfo,
    },
    cpi::{
        v2:: {
            CpiAccounts,
            LightSystemProgramCpi,
        },
        LightCpiInstruction,
        InvokeLightSystemProgram,
    },
};

#[event]
#[derive(Clone, Debug, Default, LightDiscriminator)]
pub struct ArtifactBoxAccount {
    pub owner: Pubkey,
    pub artifact_id: [u8; 32],
    pub commitment: [u8; 32],
    // 0 = Locked, 1 = Unlocked
    pub status: u8,
}

pub fn handler_generate_artifact_box<'info>(
    ctx: Context<'_, '_, '_, 'info, GenerateArtifactBoxAccounts<'info>>,
    proof: ValidityProof,
    address_tree_info: PackedAddressTreeInfo,
    output_state_tree_index: u8,
    artifact_id: [u8; 32],
    commitment: [u8; 32],
) -> Result<()> {
    // 1. Generate light cpi accounts.
    let light_cpi_accounts = CpiAccounts::new(
        ctx.accounts.signer.as_ref(),
        ctx.remaining_accounts,
        crate::LIGHT_CPI_SIGNER,
    );

    // 2. Get address tree pubkey.
    let address_tree_pubkey = address_tree_info
        .get_tree_pubkey(&light_cpi_accounts)
        .map_err(|_| ErrorCode::InvalidEquipmentBoxAddressTree)?;
    require!(
        address_tree_pubkey.to_bytes() == ADDRESS_TREE_V2,
        ErrorCode::InvalidEquipmentBoxAddressTree
    );

    let (address, address_seed) = derive_address(
        &[EQUIPMENT_BOX_SEED, ctx.accounts.artifact_owner.key().as_ref(), &artifact_id],
        &address_tree_pubkey,
        &crate::ID,
    );

    let mut compressed_account = LightAccount::<ArtifactBoxAccount>::new_init(
        &crate::ID,
        Some(address),
        output_state_tree_index,
    );
    compressed_account.owner = ctx.accounts.artifact_owner.key();
    compressed_account.artifact_id = artifact_id;
    compressed_account.commitment = commitment;
    compressed_account.status = 0;

    msg!("Starting CPI to mint compressed account...");
    LightSystemProgramCpi::new_cpi(crate::LIGHT_CPI_SIGNER, proof)
        .with_light_account(compressed_account)?
        .with_new_addresses(&[
            address_tree_info.into_new_address_params_assigned_packed(address_seed, Some(0))
        ])
        .invoke(light_cpi_accounts)?;
    msg!("Mint compressed account successful.");
    Ok(())
}

#[derive(Accounts)]
pub struct GenerateArtifactBoxAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: Only storing the pubkey. No signature needed here.
    pub artifact_owner: UncheckedAccount<'info>,
}