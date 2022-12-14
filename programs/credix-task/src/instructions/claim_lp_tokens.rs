use crate::states::{ErrorCode, *};
use crate::utils::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, TokenAccount};

pub fn claim_lp_tokens(ctx: Context<ClaimLPTokens>) -> Result<()> {
    let clock = Clock::get()?;

    require!(
        clock.unix_timestamp > ctx.accounts.capital_call_account.deposit_deadline_timestamp,
        ErrorCode::ClaimNotAllowed
    );

    require!(
        !ctx.accounts.call_investing_info_pda.lp_tokens_claimed,
        ErrorCode::LPTokensAlreadyClaimed
    );

    require!(
        ctx.accounts.capital_call_vault.amount >= ctx.accounts.capital_call_account.required_call,
        ErrorCode::RequiredAmountNotAchieved
    );

    let seeds = [
        MINT_AUTHORITY_SEED,
        &[ctx.accounts.capital_call_account.lp_nonce],
    ];
    let signer = &[&seeds[..]];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.lp_token_mint.clone(),
        to: ctx.accounts.to.to_account_info().clone(),
        authority: ctx.accounts.lp_mint_authority.clone(),
    };

    let cpi_program = ctx.accounts.token_program.clone();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    let lp_tokens_amount =
        (ctx.accounts.call_investing_info_pda.deposited_amount as f64) / LP_PRICE;
    token::mint_to(cpi_ctx, lp_tokens_amount as u64)?;

    ctx.accounts.call_investing_info_pda.lp_tokens_claimed = true;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimLPTokens<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(constraint = capital_call_account.vault == capital_call_vault.key())]
    pub capital_call_account: Account<'info, CapitalCall>,
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
    #[account(mut, constraint = to.owner.key() == initializer.key())]
    pub to: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Mint address for LP tokens.
    pub lp_token_mint: AccountInfo<'info>,
    /// CHECK: Authority for minting LP tokens.
    #[account(seeds = [MINT_AUTHORITY_SEED], bump = capital_call_account.lp_nonce)]
    pub lp_mint_authority: AccountInfo<'info>,
}
