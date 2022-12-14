use crate::states::{ErrorCode, *};

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

pub fn invest_capital(ctx: Context<InvestCapital>, deposit_amount: u64) -> Result<()> {
    let clock = Clock::get()?;

    require!(
        clock.unix_timestamp <= ctx.accounts.capital_call_account.deposit_deadline_timestamp,
        ErrorCode::InvestingNotAllowed
    );

    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info().clone(),
        to: ctx.accounts.capital_call_vault.to_account_info().clone(),
        authority: ctx.accounts.initializer.to_account_info().clone(),
    };
    let cpi_program = ctx.accounts.token_program.clone();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, deposit_amount)?;

    ctx.accounts.call_investing_info_pda.deposited_amount += deposit_amount;
    ctx.accounts.call_investing_info_pda.lp_tokens_claimed = false;

    Ok(())
}

#[derive(Accounts)]
pub struct InvestCapital<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(constraint = capital_call_account.vault == capital_call_vault.key())]
    pub capital_call_account: Account<'info, CapitalCall>,
    #[account(mut)]
    /// CHECK: Vault account corresponding to provided capital call.
    pub capital_call_vault: AccountInfo<'info>,
    #[account(mut, constraint = from.owner.key() == initializer.key())]
    pub from: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        seeds = [initializer.to_account_info().key.as_ref(), capital_call_account.to_account_info().key.as_ref()],
        space = CallInvestingInfo::LEN,
        bump
    )]
    pub call_investing_info_pda: Account<'info, CallInvestingInfo>,
    /// CHECK: Token program address for CPI.
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
