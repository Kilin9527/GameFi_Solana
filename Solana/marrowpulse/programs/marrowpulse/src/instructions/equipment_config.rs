use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::ErrorCode;
use crate::instructions::authority_registry::*;

#[account]
pub struct EquipmentConfig {
    pub authority_registry: Pubkey,
    pub pause: bool,
    pub mint_equipment_enabled: bool,
    pub bump: u8,
}

impl EquipmentConfig {
    pub const SPACE: usize = 8 + 32 + 1 + 1 + 1;
}

#[event]
pub struct EquipmentConfigInitializeEvent {
    pub authority_registry: Pubkey,
    pub pause: bool,
    pub mint_equipment_enabled: bool,
    pub slot: u64,
}

#[event]
pub struct EquipmentConfigUpdateEvent {
    pub authority_registry: Pubkey,
    pub pause: bool,
    pub mint_equipment_enabled: bool,
    pub slot: u64,
}

// ==========================================
// 1. Initialize equipment config logic
// ==========================================
pub fn handler_equipment_config_initialize(ctx: Context<EquipmentConfigInitializeAccounts>, _feature_id: String) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.authority_registry = ctx.accounts.registry.key(); 
    config.pause = false;
    config.mint_equipment_enabled = true;
    config.bump = ctx.bumps.config;

    emit!(EquipmentConfigInitializeEvent {
        authority_registry: config.authority_registry,
        pause: config.pause,
        mint_equipment_enabled: config.mint_equipment_enabled,
        slot: Clock::get()?.slot,
    });

    #[cfg(feature = "debug-logging")]
    msg!("Config Initialized by authority: {}; pause: {}; mint_equipment_enabled: {}", config.authority_registry, config.pause, config.mint_equipment_enabled);

    Ok(())
}

#[derive(Accounts)]
#[instruction(_feature_id: String)]
pub struct EquipmentConfigInitializeAccounts<'info> {
    #[account(
        init,
        seeds = [EQUIPMENT_CONFIG_SEED, _feature_id.as_bytes()],
        bump,
        payer = signer,
        space = EquipmentConfig::SPACE,
    )]
    pub config: Account<'info, EquipmentConfig>,
    
    #[account(
        seeds = [AUTHORITY_REGISTRY_SEED, _feature_id.as_bytes()],
        bump = registry.bump,
        constraint = registry.admin.key() == signer.key() @ ErrorCode::InvalidEquipmentConfigInitializeSigner
    )]
    pub registry: Account<'info, AuthorityRegistry>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ========================================================
// 2. Update equipment config logic
// ========================================================
pub fn handler_equipment_config_update(ctx: Context<EquipmentConfigUpdateAccounts>, _feature_id: String, pause: Option<bool>, mint_equipment_enabled: Option<bool>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.pause = pause.unwrap_or(config.pause);
    config.mint_equipment_enabled = mint_equipment_enabled.unwrap_or(config.mint_equipment_enabled);

    emit!(EquipmentConfigUpdateEvent {
        authority_registry: ctx.accounts.config.authority_registry,
        pause: ctx.accounts.config.pause,
        mint_equipment_enabled: ctx.accounts.config.mint_equipment_enabled,
        slot: Clock::get()?.slot,
    });

    #[cfg(feature = "debug-logging")]
    msg!("Config Updated by authority: {}; pause: {}; mint_equipment_enabled: {}", config.authority_registry, config.pause, config.mint_equipment_enabled);

    Ok(())
}

#[derive(Accounts)]
#[instruction(_feature_id: String)]
pub struct EquipmentConfigUpdateAccounts<'info> {
    #[account(
        mut,
        seeds = [EQUIPMENT_CONFIG_SEED, _feature_id.as_bytes()],
        bump = config.bump,
    )]
    pub config: Account<'info, EquipmentConfig>,

    #[account(
        address = config.authority_registry,
        constraint = registry.admin.key() == signer.key() @ ErrorCode::InvalidEquipmentConfigUpdateSigner
    )]
    pub registry: Account<'info, AuthorityRegistry>,
    
    pub signer: Signer<'info>,
}
