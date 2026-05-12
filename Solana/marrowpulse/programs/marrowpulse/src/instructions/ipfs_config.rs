use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::instructions::authority_registry::*;
use crate::constants::*;

#[account]
pub struct IpfsConfigInfo {
    pub authority_registry: Pubkey,
    pub feature_id: String,
    pub config_uri: String,
    pub bump: u8,
}

impl IpfsConfigInfo {
    pub const MAX_FEATURE_ID_LEN: usize = 20;
    pub const BASE_SIZE: usize = 8 + 32 + 1 + (4 + Self::MAX_FEATURE_ID_LEN);
}

#[event]
pub struct IpfsConfigInitializeEvent {
    pub authority_registry: Pubkey,
    pub feature_id: String,
    pub config_uri: String,
    pub slot: u64,
}

#[event]
pub struct IpfsConfigUpdateEvent {
    pub authority_registry: Pubkey,
    pub feature_id: String,
    pub old_config_uri: String,
    pub new_config_uri: String,
    pub slot: u64,
}

// ==========================================
// 1. Initialize ipfs config logic
// ==========================================
pub fn handler_ipfs_config_initialize(ctx: Context<IpfsConfigInitializeAccounts>, feature_id: String, config_uri: String) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.authority_registry = ctx.accounts.registry.key(); 
    config.feature_id = feature_id.clone();
    config.config_uri = config_uri.clone();
    config.bump = ctx.bumps.config;

    emit!(IpfsConfigInitializeEvent {
        authority_registry: config.authority_registry,
        feature_id: feature_id.clone(),
        config_uri: config_uri.clone(),
        slot: Clock::get()?.slot,
    });

    #[cfg(feature = "debug-logging")]
    msg!("Config Initialized by authority: {}; feature_id: {}; config_uri: {}", config.authority_registry, config.feature_id, config.config_uri);

    Ok(())
}

#[derive(Accounts)]
#[instruction(feature_id: String, config_uri: String)]
pub struct IpfsConfigInitializeAccounts<'info> {
    #[account(
        init,
        seeds = [IPFS_CONFIG_SEED, feature_id.as_bytes()],
        bump,
        constraint = feature_id.len() <= 20 @ ErrorCode::InvalidIPFSConfigFeatureIdLength,
        constraint = config_uri.len() <= 200 @ ErrorCode::InvalidIPFSConfigUriLength,
        payer = signer,
        space = IpfsConfigInfo::BASE_SIZE + (4 + config_uri.len()),
    )]
    pub config: Account<'info, IpfsConfigInfo>,
    
    #[account(
        seeds = [AUTHORITY_REGISTRY_SEED, feature_id.as_bytes()],
        bump = registry.bump,
        constraint = registry.admin.key() == signer.key() @ ErrorCode::InvalidIPFSConfigInitializeSigner
    )]
    pub registry: Account<'info, AuthorityRegistry>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ========================================================
// 2. Update ipfs config logic
// ========================================================
pub fn handler_ipfs_config_update(ctx: Context<IpfsConfigUpdateAccounts>, feature_id: String, config_uri: String) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let old_config_uri = config.config_uri.clone();
    config.config_uri = config_uri.clone();

    emit!(IpfsConfigUpdateEvent {
        authority_registry: ctx.accounts.config.authority_registry,
        feature_id: feature_id,
        old_config_uri: old_config_uri,
        new_config_uri: config_uri,
        slot: Clock::get()?.slot,
    });

    #[cfg(feature = "debug-logging")]
    msg!("Config Updated by Admin: {}; feature_id: {}, config_uri: {}", ctx.accounts.admin.key(), config.feature_id, config.config_uri);
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(feature_id: String, config_uri: String)]
pub struct IpfsConfigUpdateAccounts<'info> {
    #[account(
        mut,
        seeds = [IPFS_CONFIG_SEED, feature_id.as_bytes()],
        bump = config.bump,
        constraint = feature_id.len() <= 20 @ ErrorCode::InvalidIPFSConfigFeatureIdLength,
        constraint = config_uri.len() <= 200 @ ErrorCode::InvalidIPFSConfigUriLength,
        realloc = IpfsConfigInfo::BASE_SIZE + (4 + config_uri.len()),
        realloc::payer = signer,
        realloc::zero = false,
    )]
    pub config: Account<'info, IpfsConfigInfo>,

    #[account(
        address = config.authority_registry,
        constraint = registry.admin.key() == signer.key() @ ErrorCode::InvalidIPFSConfigUpdateSigner
    )]
    pub registry: Account<'info, AuthorityRegistry>,
    
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
