use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name is longer than 32 bytes")]
    NameTooLong,
}
