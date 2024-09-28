pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AJmPgdfkPmCjndPPKcG9TdUaatrcEgLxyJQ8yC5D7u7p");

#[program]
pub mod nft_auction {

    use super::*;

    pub fn initialize_auction(
        ctx: Context<InitalizeAuction1>,
        floor_price: u64,
        end_time: i64,
    ) -> Result<()> {
        ctx.accounts
            .initalize_auction(floor_price, end_time, &ctx.bumps)?;
        Ok(())
    }
    pub fn bid(ctx: Context<Bid>, amount: u64) -> Result<()> {
        ctx.accounts.bid(amount)?;
        Ok(())
    }
    pub fn resolve_auction(ctx: Context<ResolveAuction>) -> Result<()> {
        ctx.accounts.resolve_auction()?;
        Ok(())
    }
}
