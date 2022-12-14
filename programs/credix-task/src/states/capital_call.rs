use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct CapitalCall {
    pub deposit_deadline_timestamp: i64,
    pub required_call: u64,
    pub vault: Pubkey,
    pub vault_nonce: u8,
    pub lp_nonce: u8,
}

impl CapitalCall {
    pub const LEN: usize = DISCRIMINATOR + U64 + U64 + PUBKEY + U64;
}
