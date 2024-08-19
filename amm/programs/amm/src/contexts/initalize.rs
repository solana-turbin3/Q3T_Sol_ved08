
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::{Mint, TokenAccount, Token},
};

use crate::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initalize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_x: Box<Account<'info, Mint>>,
    pub mint_y: Box<Account<'info, Mint>>,
    
    #[account(
        init,
        payer = maker,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"amm", mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump 
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = maker,
        mint::authority = config,
        mint::decimals = 6,
        seeds = [b"mint", config.key().as_ref()],
        bump
    )]
    pub mint_lp: Account<'info, Mint>,
    // #[account(
    //     mut,
    //     associated_token::mint = mint_x,
    //     associated_token::authority = maker,
    // )]
    // pub maker_ata_x: Box<Account<'info, TokenAccount>>,
    // #[account(
    //     mut,
    //     associated_token::mint = mint_y,
    //     associated_token::authority = maker,
    // )]
    // pub maker_ata_y: Box<Account<'info, TokenAccount>>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initalize<'info> {
    pub fn save_config(&mut self, seed: u64, fee: u16, bump: u8, lp_bump: u8) -> Result<()> {
        self.config.set_inner(Config {
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            bump,
            seed,
            lp_bump,
        });
        Ok(())
    }
    
}