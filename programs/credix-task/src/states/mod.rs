use anchor_lang::prelude::*;

pub mod call_investing_info;
pub mod capital_call;

pub use call_investing_info::*;
pub use capital_call::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Withdraw is not allowed because capital call is still open")]
    WithdrawNotAllowed,
    #[msg("Investing is not allowed because capital call is already closed")]
    InvestingNotAllowed,
    #[msg("No funds to withdraw")]
    NothingToWithdraw,
    #[msg("Claiming is not allowed because capital call is still open")]
    ClaimNotAllowed,
    #[msg("LP tokens for this capital call were already claimed")]
    LPTokensAlreadyClaimed,
    #[msg("Required amount for capital call was achieved. Not possible to withdraw funds.")]
    RequiredAmountAchieved,
    #[msg("Required amount for capital call was not achieved. Not possible to claim LP tokens.")]
    RequiredAmountNotAchieved,
}
