use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{
        mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts},
        MasterEditionAccount,
        Metadata,
        MetadataAccount
    }, token_interface::{approve, Approve, Mint, TokenAccount, TokenInterface}
};

use crate::{{stake_config::StakeConfig, StakeAccount, UserAccount}, error::ErrorCode};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub collection: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub signer_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
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

    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), signer.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init,
        payer = signer,
        seeds = [b"stake".as_ref(), signer.key().as_ref(), config.key().as_ref()],
        bump,
        space = StakeAccount::INIT_SPACE,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
}



impl <'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        
        require!(self.user_account.amount_staked < self.config.max_stake, ErrorCode::MaxAmountStaked);

        let cpi_program = self.token_program.to_account_info();


        let cpi_accounts = Approve{
            to: self.signer_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.signer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            cpi_program, 
            cpi_accounts
        );
        approve(cpi_ctx, 1)?;

        let cpi_accounts = FreezeDelegatedAccountCpiAccounts {
            delegate: &self.stake_account.to_account_info(),
            token_account: &self.signer_ata.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.mint.to_account_info(),
            token_program: &self.token_program.to_account_info(),
        };

        let binding = [self.stake_account.bump];
        let signer_seeds = &[&[
            b"stake",
            self.signer.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &binding
        ][..]];

        FreezeDelegatedAccountCpi::new(&self.metadata_program.to_account_info(), cpi_accounts)
        .invoke_signed(signer_seeds)?;

        self.stake_account.set_inner(StakeAccount {
            owner: self.signer.key(),
            mint: self.mint.key(),
            last_update: Clock::get()?.unix_timestamp,
            bump: bumps.stake_account,
        });
        self.user_account.amount_staked += 1;
        Ok(())
    }
}