use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Auction {
    pub authority: Pubkey,
    pub nft_mint: Pubkey,
    pub floor_price: u64,
    pub current_bid: u64,
    pub current_bidder: Pubkey,
    pub end_time: i64,
    pub bump: u8
}