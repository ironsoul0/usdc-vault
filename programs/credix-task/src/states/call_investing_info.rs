use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct CallInvestingInfo {
    pub deposited_amount: u64,
    pub lp_tokens_claimed: bool,
}

impl CallInvestingInfo {
    pub const LEN: usize = DISCRIMINATOR + U64 + BOOL + BOOL;
}
