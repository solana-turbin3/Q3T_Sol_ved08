use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{error::AuctionError, Auction};

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        seeds = [b"auction", auction.nft_mint.key().as_ref()],
        bump = auction.auction_bump
    )]
    pub auction: Account<'info, Auction>,
    #[account(
        mut,
        seeds = [b"bidderVault", auction.key().as_ref()],
        bump = auction.bidder_vault_bump
    )]
    pub bidder_vault: SystemAccount<'info>,

    // TODO
    // #[account(
    //     mut,
    //     realloc = 8 + 1 + (4 + (32 * reward_pubkeys.reward_list.len() + 1)),
    //     realloc::payer = bidder,
    //     realloc::zero = false,
    //     seeds = [b"rewards", auction.key().as_ref()],
    //     bump = reward_pubkeys.bump,

    // )]
    // pub reward_pubkeys: Account<'info, RewardPubkeys>,

    /// CHECK: Previous bidder account
    pub previous_bidder: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

impl<'info> Bid<'info> {
    pub fn bid(&mut self, amount: u64) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        if self.auction.current_bidder.is_none() == false {
            
            require_keys_neq!(
                self.auction.current_bidder.unwrap(),
                self.bidder.key(),
                AuctionError::SameBidderError
            );
            match &self.previous_bidder {
                None => msg!("No previos bidder"),
                Some(pubkey) => require_keys_eq!(
                    self.auction.current_bidder.unwrap(),
                    pubkey.key(),
                    AuctionError::BiddersMatchError
                )
            }
            
        }
        // Check auction conditions
        require!(self.auction.ended == false, AuctionError::AuctionEnded);
        require!(self.auction.current_bid < amount, AuctionError::LowBidError);
        require!(
            current_time < self.auction.end_time,
            AuctionError::AuctionEnded
        );

        // Transfer old bid amount back to previous bidder
        if self.previous_bidder.is_none() == false {

            let accounts = Transfer {
                from: self.bidder_vault.to_account_info(),
                to: self.previous_bidder.clone().unwrap().to_account_info(),
            };
            let binding = [self.auction.bidder_vault_bump];
            let signer_seeds = &[&[
                b"bidderVault",
                self.auction.to_account_info().key.as_ref(),
                &binding,
            ][..]];
            let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer_seeds,
            );
    
            transfer(ctx, self.auction.current_bid)?;
        }

        // Transfer new bid amount
        let accounts = Transfer {
            from: self.bidder.to_account_info(),
            to: self.bidder_vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, amount)?;

        // Update auction state
        self.auction.current_bid = amount;
        self.auction.current_bidder = Some(self.bidder.key());

        // Check rewards eligibility
        // if self.auction.floor_price.checked_mul(2).unwrap() < amount {
        //     self.reward_pubkeys.to_account_info().realloc(
        //         8 + 1 + (4 + (32 * self.reward_pubkeys.reward_list.len() + 1)), 
        //         false
        //     )?;
        //     self.reward_pubkeys.reward_list.push(self.bidder.key());
        // }

        Ok(())
    }
}
