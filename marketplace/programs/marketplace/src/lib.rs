pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Dvctqo1oRNDkNKTozmLKR9KyKPM6SptikdBCoQLAkPXq");

#[program]
pub mod marketplace {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fees: u16) -> Result<()> {
        ctx.accounts.init(name, fees, &ctx.bumps)?;
        Ok(())
    }
    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        Ok(())
    }
    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.transfer_nft()?;
        ctx.accounts.close_listing()?;
        Ok(())
    }
}
