use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitalizeAuction<'info> {
    #[account(mut)]
    pub authority: Signer<'info>
}
impl<'info> InitalizeAuction<'info> {
    // TODO
}
