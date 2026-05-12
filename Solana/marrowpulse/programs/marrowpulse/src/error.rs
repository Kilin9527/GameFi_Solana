use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // Authority registry errors
    #[msg("Authority registry: Invalid feature ID length.")]
    InvalidAuthorityRegistryFeatureIdLength,
    #[msg("Authority registry: Invalid initializer.")]
    InvalidAuthorityRegistryInitializer,
    #[msg("Authority registry: Invalid update signer.")]
    InvalidAuthorityRegistryUpdateSigner,
    #[msg("Authority registry: Invalid Accept signer.")]
    InvalidAuthorityRegistryAcceptSigner,
    #[msg("Authority registry: Invalid Cancel signer.")]
    InvalidAuthorityRegistryCancelSigner,
    #[msg("Authority registry: Invalid accept expired.")]
    InvalidAuthorityRegistryAcceptExpired,

    // IPFS Config errors
    #[msg("IPFS Config: invalid initializer.")]
    InvalidIPFSConfigInitializer,
    #[msg("IPFS Config: invalid initialize signer.")]
    InvalidIPFSConfigInitializeSigner,
    #[msg("IPFS Config: invalid update signer.")]
    InvalidIPFSConfigUpdateSigner,
    #[msg("IPFS Config: invalid URI length.")]
    InvalidIPFSConfigUriLength,
    #[msg("IPFS Config: invalid feature ID length.")]
    InvalidIPFSConfigFeatureIdLength,

    // Equipment Config errors
    #[msg("Equipment Config: invalid initializer.")]
    InvalidEquipmentConfigInitializeSigner,
    #[msg("Equipment Config: invalid update signer.")]
    InvalidEquipmentConfigUpdateSigner,

    // Equipment Box errors
    #[msg("Equipment Box: invalid address tree.")]
    InvalidEquipmentBoxAddressTree,
    #[msg("Equipment Box: invalid param proof convert.")]
    InvalidEquipmentBoxParamProofConvert,
    #[msg("Equipment Box: invalid param address tree convert.")]
    InvalidEquipmentBoxParamAddressTreeConvert,

    // Common errors
    #[msg("Unwrap option value failed.")]
    UnwrapOptionValueFailed,
}
