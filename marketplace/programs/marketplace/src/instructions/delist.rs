use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, Mint, TokenInterface, TransferChecked, transfer_checked };

use crate::{Listing, Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Delist<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,

    pub maker_mint: Box<InterfaceAccount<'info, Mint>>,
    pub collection_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump
    )]
    pub listing: Box<Account<'info, Listing>>,

    #[account(
        mut,
        associated_token::authority = listing,
        associated_token::mint = maker_mint 
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,


    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&mut self) -> Result<()> {
        let signer_seeds = &[&[
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref()
        ][..]];
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;
        Ok(())
    }
}