import * as anchor from "@project-serum/anchor";

export const MINT_AMOUNT = 1000000;
export const MS_IN_SECONDS = 1000;
export const LAMPORTS_PER_SOL = 1e9;
export const VAULT_SEED = anchor.utils.bytes.utf8.encode("vault");
export const MINT_AUTHORITY_SEED = anchor.utils.bytes.utf8.encode("mint");

export const getAirdrop = async (
  provider: anchor.AnchorProvider,
  pk: anchor.web3.PublicKey,
  amount: number = 10
) => {
  const airdropSig = await provider.connection.requestAirdrop(pk, amount * LAMPORTS_PER_SOL);
  const latestSellerBlockhash = await provider.connection.getLatestBlockhash();
  await provider.connection.confirmTransaction({
    blockhash: latestSellerBlockhash.blockhash,
    lastValidBlockHeight: latestSellerBlockhash.lastValidBlockHeight,
    signature: airdropSig,
  });
};

export const sleep = async (sleepMs: number) => {
  return new Promise((resolve) => setTimeout(resolve, sleepMs));
}

const USDC_LIQUIDITY_POOL = 2435827.0;
const CREDIT_OUTSTANDING = 7348028.0;
const LP_SUPPLY = 9127492.0;

export const LP_PRICE = (USDC_LIQUIDITY_POOL + CREDIT_OUTSTANDING) / LP_SUPPLY;
