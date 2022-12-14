use crate::states::{ErrorCode, *};
use crate::utils::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

pub fn withdraw_capital(ctx: Context<WithdrawCapital>) -> Result<()> {
    let clock = Clock::get()?;

    require!(
        clock.unix_timestamp > ctx.accounts.capital_call_account.deposit_deadline_timestamp,
        ErrorCode::WithdrawNotAllowed
    );

    require!(
        ctx.accounts.call_investing_info_pda.deposited_amount > 0,
        ErrorCode::NothingToWithdraw
    );

    require!(
        ctx.accounts.capital_call_vault.amount < ctx.accounts.capital_call_account.required_call,
        ErrorCode::RequiredAmountAchieved
    );

    let call_investing_info = &mut ctx.accounts.call_investing_info_pda;
    let seeds = [VAULT_SEED, &[ctx.accounts.capital_call_account.vault_nonce]];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.capital_call_vault.to_account_info().clone(),
        to: ctx.accounts.to.to_account_info().clone(),
        authority: ctx.accounts.vault_owner.clone(),
    };

    let cpi_program = ctx.accounts.token_program.clone();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::transfer(cpi_ctx, call_investing_info.deposited_amount)?;
    call_investing_info.deposited_amount = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawCapital<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(constraint = capital_call_account.vault == capital_call_vault.key())]
    pub capital_call_account: Account<'info, CapitalCall>,
    #[account(mut, constraint = capital_call_vault.owner.key() == vault_owner.key())]
    /// CHECK: Vault account corresponding to provided capital call.
    pub capital_call_vault: Account<'info, TokenAccount>,
    #[account(
      mut,
      seeds = [initializer.to_account_info().key.as_ref(), capital_call_account.to_account_info().key.as_ref()],
      bump
    )]
    pub call_investing_info_pda: Account<'info, CallInvestingInfo>,
    /// CHECK: Token program address for CPI.
    pub token_program: AccountInfo<'info>,
    /// CHECK: PDA owning vault allowing program to transfer tokens under its own authority
    #[account(seeds = [VAULT_SEED], bump = capital_call_account.vault_nonce)]
    pub vault_owner: AccountInfo<'info>,
    #[account(mut, constraint = to.owner.key() == initializer.key())]
    pub to: Account<'info, TokenAccount>,
}
