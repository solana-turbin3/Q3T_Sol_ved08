use anchor_lang::prelude::*;

declare_id!("BzSik1LnEB5Tutpf5DXS4JpqWyFuZFJNFX2gF93jRPGW");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
