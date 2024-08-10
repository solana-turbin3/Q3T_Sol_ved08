use anchor_lang::prelude::*;

mod contexts;
use contexts::*;

mod state;
use state::*;
declare_id!("4Qxrpu2gE76bzwmap93eR9UgW242BnccTJF1PFWa92SH");


#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, recieve: u64) -> Result<()> {
        ctx.accounts.save_escrow(seed, recieve, ctx.bumps.escrow)?;
        ctx.accounts.deposit_to_vault(amount)
    }
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer()?;
        ctx.accounts.withdraw_and_close()
    }
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()?;
        Ok(())
    }
}
