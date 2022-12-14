use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod utils;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod credix_task {
    use crate::instructions::invest_capital::InvestCapital;

    use super::*;

    pub fn create_capital_call(
        ctx: Context<CreateCapitalCall>,
        deposit_deadline_timestamp: i64,
        required_call: u64,
        vault_nonce: u8,
        lp_nonce: u8,
    ) -> Result<()> {
        instructions::create_capital_call(
            ctx,
            deposit_deadline_timestamp,
            required_call,
            vault_nonce,
            lp_nonce,
        )
    }

    pub fn invest_capital(ctx: Context<InvestCapital>, deposit_amount: u64) -> Result<()> {
        instructions::invest_capital(ctx, deposit_amount)
    }

    pub fn withdraw_capital(ctx: Context<WithdrawCapital>) -> Result<()> {
        instructions::withdraw_capital(ctx)
    }

    pub fn claim_lp_tokens(ctx: Context<ClaimLPTokens>) -> Result<()> {
        instructions::claim_lp_tokens(ctx)
    }
}
