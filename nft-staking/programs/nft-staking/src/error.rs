use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Max amount already Staked")]
    MaxAmountStaked
}