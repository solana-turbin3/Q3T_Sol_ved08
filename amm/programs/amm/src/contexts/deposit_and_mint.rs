use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,token::{Mint, TokenAccount, Token, transfer_checked, MintTo, mint_to, TransferChecked}
};

use crate::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct DepositAndMint<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        seeds = [b"amm", mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump 
    )]
    pub config: Account<'info, Config>,
    #[account(
        mint::authority = config,
        mint::decimals = 6,
        seeds = [b"mint", config.key().as_ref()],
        bump
    )]
    pub mint_lp: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Box<Account<'info, TokenAccount>>,
    #[account(
        associated_token::mint = mint_x,
        associated_token::authority = maker,
    )]
    pub maker_ata_x: Box<Account<'info, TokenAccount>>,
    #[account(
        associated_token::mint = mint_y,
        associated_token::authority = maker,
    )]
    pub maker_ata_y: Box<Account<'info, TokenAccount>>,
    #[account(
        associated_token::mint = mint_lp,
        associated_token::authority = maker,
    )]
    pub maker_ata_lp: Box<Account<'info, TokenAccount>>,


    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> DepositAndMint<'info> {
    pub fn deposit(&self, amount: u64, is_x: bool) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (self.maker_ata_x.to_account_info(), self.vault_x.to_account_info(), self.mint_x.to_account_info(), self.mint_x.decimals),
            false => (self.maker_ata_y.to_account_info(), self.vault_y.to_account_info(), self.mint_y.to_account_info(), self.mint_y.decimals)
        };
        let accounts = TransferChecked {
            from,
            mint: mint.to_account_info(),
            to,
            authority: self.maker.to_account_info(),
        };
        let ctx = CpiContext::new(
            self.token_program.to_account_info(), 
            accounts
        );

        transfer_checked(ctx, amount, decimals)?;

        Ok(())
    }
    pub fn mint_lp_tokens(&self, amount_x: u64, amount_y: u64) -> Result<()> {
        let amount = amount_x.checked_mul(amount_y).ok_or(ProgramError::ArithmeticOverflow)?;
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"amm",
            self.mint_x.to_account_info().key.as_ref(),
            self.mint_y.to_account_info().key.as_ref(),
            &self.config.seed.to_le_bytes()[..],
            &[self.config.bump],
        ]];
        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.maker_ata_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            accounts, 
            &signer_seeds
        );
        mint_to(ctx, amount)?;
        Ok(())
    }
}