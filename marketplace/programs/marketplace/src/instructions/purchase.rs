use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(), listing.mint.as_ref()],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::authority = taker,
        associated_token::mint = maker_mint,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::authority = listing,
        associated_token::mint = maker_mint,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    ///CHECK: This is maker's account.
    pub maker: UncheckedAccount<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn send_sol(&mut self) -> Result<()> {
        let accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, self.listing.price)?;
        Ok(())
    }
    pub fn transfer_nft(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };
        let binding = [self.listing.bump];
        let signer_seeds = &[&[
            self.marketplace.to_account_info().key.as_ref(),
            self.listing.mint.as_ref(),
            &binding,
        ][..]];
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(ctx, 1, 0)?;
        Ok(())
    }
    pub fn close_listing(&mut self) -> Result<()> {
        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let binding = [self.listing.bump];
        let signer_seeds = &[&[
            self.marketplace.to_account_info().key.as_ref(),
            self.listing.mint.as_ref(),
            &binding,
        ][..]];
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        close_account(ctx)?;
        Ok(())
    }
}
