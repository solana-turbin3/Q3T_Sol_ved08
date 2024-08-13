use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub use state::*;
pub mod instructions;
pub use instructions::*;
declare_id!("2veBWq8V8oFw59X1RmjK3WfbfrQYkdJA2DRGrenLHcax");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
