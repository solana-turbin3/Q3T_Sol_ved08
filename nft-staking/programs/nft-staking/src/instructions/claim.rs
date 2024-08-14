use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, MintTo, mint_to}};

use crate::{stake_config::StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [
        b"rewards",
        config.key().as_ref(),  
        ],
        bump = config.rewards_bump,
        mint::authority = config,
        mint::decimals = 6
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
    )]
    pub rewards_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


impl <'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        
        let cpi_program = self.token_program.to_account_info();

        let binding = [self.config.bump];
        let signer_seeds = &[&[
            b"config".as_ref(), 
            &binding
        ][..]];
        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            cpi_program, 
            cpi_accounts, 
            signer_seeds
        );
        let amount_to_mint = self.user_account.points as u64 * 10_u64.pow(self.rewards_mint.decimals as u32);
        mint_to(cpi_context, amount_to_mint)?;

        Ok(())
    }
}