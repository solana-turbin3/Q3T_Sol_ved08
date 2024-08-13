use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::stake_config::StakeConfig;


#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init, 
        payer = signer,
        seeds = [b"config".as_ref()],
        bump,
        space = StakeConfig::INIT_SPACE,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init, 
        payer = signer,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::authority = config,
        mint::decimals = 6,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl <'info> InitializeConfig<'info> {
    pub fn init_config(&mut self, points_per_stake: u8, max_stake: u8, freeze_period: u32, bumps: &InitializeConfigBumps) -> Result<()> {
        self.config.set_inner(StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.config,
        });
        Ok(())
    }
}