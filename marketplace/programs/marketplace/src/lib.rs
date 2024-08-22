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
        ctx.accounts.init(name, fees, &ctx.bumps)
    }
}
