use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::ErrorCode;

#[account]
pub struct AuthorityRegistry {
    pub admin: Pubkey,
    pub pending_admin: Option<Pubkey>,
    pub proposal_expiry: Option<i64>,
    pub bump: u8,
    pub feature_id: String,
}

impl AuthorityRegistry {
    pub const MAX_FEATURE_ID_LEN: usize = 20;
    pub const SPACE: usize = 8 + 32 + 33 + 9 + 1 + (4 + Self::MAX_FEATURE_ID_LEN);
}

#[event]
pub struct AuthorityRegistryInitializeEvent {
    pub admin: Pubkey,
    pub feature_id: String,
    pub slot: u64,
}

#[event]
pub struct AuthorityRegistryUpdateEvent {
    pub admin: Pubkey,
    pub pending_admin: Pubkey,
    pub feature_id: String,
    pub slot: u64,
}

#[event]
pub struct AuthorityRegistryAcceptEvent {
    pub admin: Pubkey,
    pub feature_id: String,
    pub slot: u64,
}

#[event]
pub struct AuthorityRegistryCancelEvent {
    pub denied_admin: Pubkey,
    pub feature_id: String,
    pub slot: u64,
}

// =========================================================
// 1. Initialize authority registry logic
// =========================================================
pub fn handler_authority_registry_initialize(ctx: Context<AuthorityRegistryInitializeAccounts>, feature_id: String) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    registry.admin = ctx.accounts.authority_signer.key();
    registry.pending_admin = None;
    registry.proposal_expiry = None;
    registry.bump = ctx.bumps.registry;
    registry.feature_id = feature_id.clone();

    emit!(AuthorityRegistryInitializeEvent {
        admin: registry.admin,
        feature_id: feature_id.clone(),
        slot: Clock::get()?.slot,
    });

    #[cfg(feature = "debug-logging")]
    msg!("Authority Registry Initialized for feature: {}; admin: {}", feature_id, registry.admin);

    Ok(())
}

#[derive(Accounts)]
#[instruction(feature_id: String)]
pub struct AuthorityRegistryInitializeAccounts<'info> {
    #[account(
        init,
        seeds = [AUTHORITY_REGISTRY_SEED, feature_id.as_bytes()],
        bump,
        constraint = feature_id.len() <= 20 @ ErrorCode::InvalidAuthorityRegistryFeatureIdLength,
        constraint = authority_signer.key() == GENESIS_ADMIN @ ErrorCode::InvalidAuthorityRegistryInitializer,
        space = AuthorityRegistry::SPACE + (4 + feature_id.len()),
        payer = authority_signer,
    )]
    pub registry: Account<'info, AuthorityRegistry>,

    #[account(mut)]
    pub authority_signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// =========================================================
// 2. Propose transfer authority registry logic
// =========================================================
pub fn handler_authority_registry_propose(ctx: Context<AuthorityRegistryProposeAccounts>, feature_id: String) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    let clock = Clock::get()?;

    registry.pending_admin = Some(ctx.accounts.new_admin.key());
    registry.proposal_expiry = Some(clock.unix_timestamp + PROPOSAL_DURATION);

    let pending_admin = ctx.accounts.new_admin.key();
    emit!(AuthorityRegistryUpdateEvent {
        admin: registry.admin,
        pending_admin: pending_admin,
        feature_id: feature_id.clone(),
        slot: Clock::get()?.slot,
    });

    #[cfg(feature = "debug-logging")]
    msg!("Authority Registry update for feature: {}; admin: {}, pending admin: {}", feature_id, registry.admin, pending_admin);

    Ok(())
}

#[derive(Accounts)]
#[instruction(feature_id: String)]
pub struct AuthorityRegistryProposeAccounts<'info> {
    #[account(
        mut,
        seeds = [AUTHORITY_REGISTRY_SEED, feature_id.as_bytes()],
        bump = registry.bump,
        constraint = feature_id.len() <= 20 @ ErrorCode::InvalidAuthorityRegistryFeatureIdLength,
        has_one = admin @ ErrorCode::InvalidAuthorityRegistryUpdateSigner,        
    )]
    pub registry: Account<'info, AuthorityRegistry>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: Only storing the pubkey. No signature needed here.
    pub new_admin: UncheckedAccount<'info>,
}

// =========================================================
// 3. Accept transfer authority registry logic
// =========================================================
pub fn handler_authority_registry_accept(ctx: Context<AuthorityRegistryAcceptAccounts>, feature_id: String) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    let clock = Clock::get()?;

    require!(
        clock.unix_timestamp <= registry.proposal_expiry.ok_or(ErrorCode::UnwrapOptionValueFailed)?,
        ErrorCode::InvalidAuthorityRegistryAcceptExpired
    );
    let pending_admin = registry.pending_admin.ok_or(ErrorCode::UnwrapOptionValueFailed)?;
    registry.admin = pending_admin;
    registry.pending_admin = None;
    registry.proposal_expiry = None;

    emit!(AuthorityRegistryAcceptEvent {
        admin: registry.admin,
        feature_id: feature_id.clone(),
        slot: clock.slot,
    });

    #[cfg(feature = "debug-logging")]
    msg!("Authority Registry accept for feature: {}; admin: {}", feature_id, registry.admin);

    Ok(())
}

#[derive(Accounts)]
#[instruction(feature_id: String)]
pub struct AuthorityRegistryAcceptAccounts<'info> {
    #[account(
        mut,
        seeds = [AUTHORITY_REGISTRY_SEED, feature_id.as_bytes()],
        bump = registry.bump,
        constraint = feature_id.len() <= 20 @ ErrorCode::InvalidAuthorityRegistryFeatureIdLength,
        constraint = registry.pending_admin == Some(pending_admin.key()) @ ErrorCode::InvalidAuthorityRegistryAcceptSigner,
    )]
    pub registry: Account<'info, AuthorityRegistry>,

    #[account(mut)]
    pub pending_admin: Signer<'info>,
}

// =========================================================
// 4. Cancel transfer authority registry logic
// =========================================================
pub fn handler_authority_registry_cancel(ctx: Context<AuthorityRegistryCancelAccounts>, feature_id: String) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    let signer = &ctx.accounts.signer;
    let clock = Clock::get()?;

    registry.pending_admin = None;
    registry.proposal_expiry = None;

    emit!(AuthorityRegistryCancelEvent {
        denied_admin: signer.key(),
        feature_id: feature_id.clone(),
        slot: clock.slot,
    });

    #[cfg(feature = "debug-logging")]
    msg!("Authority Registry cancel for feature: {}; denied admin: {}", feature_id, signer.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(feature_id: String)]
pub struct AuthorityRegistryCancelAccounts<'info> {
    #[account(
        mut,
        seeds = [AUTHORITY_REGISTRY_SEED, feature_id.as_bytes()],
        bump = registry.bump,
        constraint = feature_id.len() <= 20 @ ErrorCode::InvalidAuthorityRegistryFeatureIdLength,
        constraint = registry.pending_admin == Some(signer.key()) || registry.admin == signer.key() @ ErrorCode::InvalidAuthorityRegistryCancelSigner,
    )]
    pub registry: Account<'info, AuthorityRegistry>,

    #[account(mut)]
    pub signer: Signer<'info>,
}