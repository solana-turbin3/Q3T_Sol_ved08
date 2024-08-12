use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Config {
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16, // 100% => 10000
    pub bump: u8,
    pub seed: u64,
    pub lp_bump: u8,
} 