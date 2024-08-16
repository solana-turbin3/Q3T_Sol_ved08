use anchor_lang::prelude::*;
use anchor_spl::{ 
    associated_token::AssociatedToken, 
    metadata::{MasterEditionAccount, MetadataAccount, Metadata, mpl_token_metadata::instructions::{
        ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts
    }}, 
    token_interface::{revoke, Mint, Revoke, TokenAccount, TokenInterface}
};

use crate::{stake_config::StakeConfig, StakeAccount, UserAccount};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        seeds = [b"stake", user.key().as_ref(), config.key().as_ref()],
        bump = stake_account.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {

        let token_program = &self.token_program.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let delegate = &self.stake_account.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        let binding = [self.stake_account.bump];
        let signer_seeds = &[&[
            b"stake",
            self.user.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &binding
        ][..]];

        // CHECK HERE
        
        let last_update = self.stake_account.last_update;
        let current_time = Clock::get()?.unix_timestamp;
        
        let time_passed = ((current_time - last_update) / (60 * 60 * 24)) as u32;
        let points_per_stake = self.config.points_per_stake as u32;
        
        self.user_account.points += time_passed * points_per_stake;
        
        let cpi_accounts = ThawDelegatedAccountCpiAccounts {
            delegate,
            token_account,
            edition,
            mint,
            token_program,
        };
        ThawDelegatedAccountCpi::new(
            metadata_program, 
            cpi_accounts,
        ).invoke_signed(signer_seeds)?;
        
        self.user_account.amount_staked -= 1;
        
        let cpi_accounts = Revoke {
            source: self.mint_ata.to_account_info(),
            authority: self.stake_account.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(), 
            cpi_accounts,
            // signer_seeds
        );
        
        revoke(cpi_ctx)?;
        Ok(())
    }
}