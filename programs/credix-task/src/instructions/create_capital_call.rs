use crate::states::*;
use crate::utils::*;

use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

pub fn create_capital_call(
    ctx: Context<CreateCapitalCall>,
    deposit_deadline_timestamp: i64,
    required_call: u64,
    vault_nonce: u8,
    lp_nonce: u8,
) -> Result<()> {
    let capital_call_account = &mut ctx.accounts.capital_call_account;

    capital_call_account.deposit_deadline_timestamp = deposit_deadline_timestamp;
    capital_call_account.required_call = required_call;
    capital_call_account.vault = ctx.accounts.vault.key();
    capital_call_account.vault_nonce = vault_nonce;
    capital_call_account.lp_nonce = lp_nonce;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateCapitalCall<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        space = CapitalCall::LEN,
        payer = owner,
    )]
    pub capital_call_account: Account<'info, CapitalCall>,
    #[account(constraint = vault.owner.key() == vault_owner.key())]
    pub vault: Account<'info, TokenAccount>,
    /// CHECK: PDA owning vault allowing program to transfer tokens under its own authority
    #[account(seeds = [VAULT_SEED], bump)]
    pub vault_owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
