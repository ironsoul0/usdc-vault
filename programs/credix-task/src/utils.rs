pub const DISCRIMINATOR: usize = 8;
pub const PUBKEY: usize = 32;
pub const U64: usize = 32;
pub const BOOL: usize = 1;

pub const VAULT_SEED: &[u8] = b"vault";
pub const MINT_AUTHORITY_SEED: &[u8] = b"mint";

pub const USDC_LIQUIDITY_POOL: f64 = 2435827.0;
pub const CREDIT_OUTSTANDING: f64 = 7348028.0;
pub const LP_SUPPLY: f64 = 9127492.0;
pub const LP_PRICE: f64 = (USDC_LIQUIDITY_POOL + CREDIT_OUTSTANDING) / LP_SUPPLY;
