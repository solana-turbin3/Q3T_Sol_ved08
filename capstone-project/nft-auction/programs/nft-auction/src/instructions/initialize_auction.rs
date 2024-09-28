use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Auction, 
    // RewardPubkeys
};

#[derive(Accounts)]
pub struct InitalizeAuction1<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    pub seller_nft_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = seller_nft_mint,
        associated_token::authority = seller,
    )]
    pub seller_nft_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = seller,
        space = 8 + Auction::INIT_SPACE,
        seeds = [b"auction", seller_nft_mint.key().as_ref()],
        bump
    )]
    pub auction: Box<Account<'info, Auction>>,
    // TODO
    // #[account(
    //     init,
    //     payer = seller,
    //     seeds = [b"rewards", auction.key().as_ref()],
    //     space = 8 + 1 + (4 + (32 * 0)), // Space for 0 pubkeys in vector
    //     bump,
    // )]
    // pub reward_pubkeys: Account<'info, RewardPubkeys>,
    #[account(
        init_if_needed,
        payer = seller,
        seeds = [b"nft_vault", seller_nft_mint.key().as_ref()],
        bump,
        token::authority = auction,
        token::mint = seller_nft_mint
    )]
    pub nft_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"bidderVault", auction.key().as_ref()],
        bump
    )]
    pub bidder_vault: SystemAccount<'info>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            seller_nft_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            seller_nft_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}
impl<'info> InitalizeAuction1<'info> {
    // TODO: Add instructions here
    pub fn initalize_auction(
        &mut self,
        floor_price: u64,
        end_time: i64,
        bumps: &InitalizeAuction1Bumps,
    ) -> Result<()> {
        self.auction.set_inner(Auction {
            ended: false,
            authority: self.seller.key(),
            nft_mint: self.seller_nft_mint.key(),
            floor_price,
            current_bid: 0,
            current_bidder: None,
            end_time,
            bidder_vault_bump: bumps.bidder_vault,
            auction_bump: bumps.auction,
            nft_vault_bump: bumps.nft_vault,
        });

        let accounts = TransferChecked {
            from: self.seller_nft_ata.to_account_info(),
            mint: self.seller_nft_mint.to_account_info(),
            to: self.nft_vault.to_account_info(),
            authority: self.seller.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        transfer_checked(ctx, 1, 0)?;
        // self.reward_pubkeys.reward_list = Vec::new();
        // self.reward_pubkeys.bump = bumps.reward_pubkeys;

        // Transfer rent to BidderVault
        let rent = Rent::get()?;
        let rent_amount = rent.minimum_balance(self.bidder_vault.data_len());
        let accounts = Transfer {
            from: self.seller.to_account_info(),
            to: self.bidder_vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, rent_amount)?;

        Ok(())
    }
}
