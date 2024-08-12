use anchor_lang::prelude::*;

pub mod state;
pub use state::*;
pub mod contexts;
pub use contexts::*;
declare_id!("BzSik1LnEB5Tutpf5DXS4JpqWyFuZFJNFX2gF93jRPGW");

#[program]
pub mod amm {

    use super::*;

    // Initalize a pool
    pub fn initialize(ctx: Context<Initalize>, seed: u64, fee: u16, amount_x: u64, amount_y: u64) -> Result<()> {
        ctx.accounts.save_config(seed, fee, ctx.bumps.config, ctx.bumps.mint_lp)?;
        ctx.accounts.deposit(amount_x, true)?;
        ctx.accounts.deposit(amount_y, false)?;
        ctx.accounts.mint_lp_tokens(amount_x, amount_y)?;
        Ok(())
    }

    // // deposit liquidity and mint lp tokens
    // pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
    //     // deposit_tokens(amount);
    //     // mint_lp_token(amount);
    //     Ok(())
    // }

    // // wburn lp to withdraw liquidity
    // pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
    //     // withdraw_tokens(amount);
    //     // burn_lp_token(amount);
    //     Ok(())
    // }

    // pub fn swap(ctx: Context<Swap>, amount_in: u64, min_out: u64, is_x: bool) -> Result<()> {
    //     // deposit_token();
    //     // withdraw_token();
    //     Ok(())
    // }
}
