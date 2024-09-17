use anchor_lang::{
    prelude::*,
    system_program::{Transfer, transfer}
};

use crate::{Auction, error::AuctionError};

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        seeds = [b"auction", auction.nft_mint.key().as_ref()],
        bump = auction.bump
    )]
    pub auction: Account<'info, Auction>,
    #[account(
        mut,
        seeds = [b"bidderVault", auction.key().as_ref()],
        bump
    )]
    pub bidder_vault: SystemAccount<'info>,
    
    /// CHECK: Previous bidder account
    pub previous_bidder: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl <'info> Bid<'info> {
    pub fn bid(&mut self, amount: u64) -> Result<()> {
        // Check if bid is higher than current bid
        require!(self.auction.current_bid < amount, AuctionError::LowBidError);
        require_keys_neq!(self.auction.current_bidder.unwrap(), self.bidder.key(), AuctionError::SameBidderError);
        require_keys_eq!(self.auction.current_bidder.unwrap(), self.previous_bidder.key(), AuctionError::BiddersMatchError);

        // Transfer old bid amount back to previous bidder
        let accounts = Transfer {
            from: self.bidder_vault.to_account_info(),
            to: self.previous_bidder.to_account_info(),
        };
        let binding = [self.auction.bump];
        let signer_seeds = &[&[
            b"bidderVault", 
            self.auction.to_account_info().key.as_ref(), 
            &binding
        ][..]];
        let ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);

        transfer(ctx, self.auction.current_bid)?;

        // Transfer new bid amount
        let accounts = Transfer {
            from: self.bidder.to_account_info(),
            to: self.bidder_vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, amount)?;
        Ok(())
    }
}