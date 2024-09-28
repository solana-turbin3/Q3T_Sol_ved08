use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::{Auction, error::AuctionError};

#[derive(Accounts)]
pub struct ResolveAuction<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    ///CHECK: Reciever is the current bidder
    #[account(mut)]
    pub reciever: UncheckedAccount<'info>,

    pub nft_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = nft_mint,
        associated_token::authority = reciever,
    )]
    pub reciever_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"auction", nft_mint.key().as_ref()],
        bump = auction.auction_bump
    )]
    pub auction: Account<'info, Auction>,
    #[account(
        mut,
        seeds = [b"nft_vault", nft_mint.key().as_ref()],
        bump = auction.nft_vault_bump,
        token::authority = auction,
        token::mint = nft_mint
    )]
    pub nft_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"bidderVault", auction.key().as_ref()],
        bump = auction.bidder_vault_bump
    )]
    pub bidder_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> ResolveAuction<'info> {
    // TODO: Add instructions here
    pub fn resolve_auction(
        &mut self
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        require!(self.auction.end_time  <= current_time, AuctionError::AuctionNotEnded);
        require_keys_eq!(self.auction.current_bidder.unwrap().key(), self.reciever.key(), AuctionError::KeysDontMatch);

        self.auction.ended = true;

        // Transfer NFT to Reciever
        let accounts = TransferChecked {
            from: self.nft_vault.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            to: self.reciever_nft_ata.to_account_info(),
            authority: self.auction.to_account_info(),
        };
        let binding = [self.auction.auction_bump];
        let signer_seeds = &[&[
            b"auction",
            self.auction.nft_mint.as_ref(),
            &binding,
        ][..]];
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            accounts, 
            signer_seeds
        );
        transfer_checked(ctx, 1, 0)?;

        // Transfer money to seller
        let accounts = Transfer {
            from: self.bidder_vault.to_account_info(),
            to: self.seller.to_account_info(),
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
            signer_seeds
        );
        let amount = self.bidder_vault.lamports();
        transfer(ctx, amount)?;
        
        Ok(())
    }
}
