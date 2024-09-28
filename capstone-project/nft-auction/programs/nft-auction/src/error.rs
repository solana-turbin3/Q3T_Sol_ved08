use anchor_lang::prelude::*;

#[error_code]
pub enum AuctionError {
    #[msg("Bid is not higher than current bid")]
    LowBidError,
    #[msg("Bidder cannot be the same as current bidder")]
    SameBidderError,
    #[msg("Bidders dont match")]
    BiddersMatchError,
    #[msg("Auction has ended")]
    AuctionEnded,
    #[msg("Auction has not ended")]
    AuctionNotEnded,
    #[msg("Keys not equal")]
    KeysDontMatch,
}
