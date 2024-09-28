use anchor_lang::prelude::*;

#[account]
pub struct Auction {
    pub ended: bool,
    pub authority: Pubkey,
    pub nft_mint: Pubkey,
    pub floor_price: u64,
    pub current_bid: u64,
    pub current_bidder: Option<Pubkey>,
    pub end_time: i64,
    pub bidder_vault_bump: u8,
    pub auction_bump: u8,
    pub nft_vault_bump: u8,
}
impl Space for Auction {
    const INIT_SPACE: usize = 1 + 32 + 32 + 8 + 8 + (1 + 32) + 8 + 1 + 1 + 1;
}

#[account]
pub struct RewardPubkeys {
    pub reward_list: Vec<Pubkey>,
    pub bump: u8,
}
