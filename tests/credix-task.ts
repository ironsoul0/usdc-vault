import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { assert } from "chai";
import { SystemProgram } from "@solana/web3.js";

import { MINT_AMOUNT, MS_IN_SECONDS, VAULT_SEED, MINT_AUTHORITY_SEED, getAirdrop, sleep, LP_PRICE } from "./utils";
import { CredixTask } from "../target/types/credix_task";

describe("credix-task", () => {
  const provider = anchor.AnchorProvider.env() as any;
  const program = anchor.workspace.CredixTask as Program<CredixTask>;

  provider.send = provider.sendAndConfirm;
  anchor.setProvider(provider);

  const setupTestingAccounts = async () => {
    const payer = anchor.web3.Keypair.generate();
    await getAirdrop(provider, payer.publicKey);

    const usdcMintAuthority = anchor.web3.Keypair.generate();
    const usdcMint = await Token.createMint(
      provider.connection,
      payer,
      usdcMintAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );
    const [vaultPda, vaultNonce] = anchor.web3.PublicKey.findProgramAddressSync([VAULT_SEED], program.programId);
    const vault = await usdcMint.createAccount(vaultPda);

    const [lpAuthorityPda, lpNonce] = anchor.web3.PublicKey.findProgramAddressSync([MINT_AUTHORITY_SEED], program.programId);
    const lpMint = await Token.createMint(
      provider.connection,
      payer,
      lpAuthorityPda,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    const investor = anchor.web3.Keypair.generate();
    await getAirdrop(provider, investor.publicKey);

    const investorUSDCAccount = await usdcMint.createAccount(investor.publicKey);
    const investorLPTokenAccount = await lpMint.createAccount(investor.publicKey);

    return {
      payer,
      usdcMintAuthority,
      usdcMint,
      vaultPda,
      vaultNonce,
      vault,
      lpAuthorityPda, lpNonce,
      lpMint,
      investor,
      investorUSDCAccount,
      investorLPTokenAccount
    };
  }

  it("Capital call is fulfilled on time and LP tokens are claimed", async () => {
    const {
      usdcMintAuthority,
      usdcMint,
      vaultPda,
      vaultNonce,
      vault,
      lpAuthorityPda, lpNonce,
      lpMint,
      investor,
      investorUSDCAccount,
      investorLPTokenAccount
    } = await setupTestingAccounts();

    const DEADLINE_AMOUNT_SECS = 5;
    const REQUIRED_CALL_AMOUNT = 100;
    const USDC_INVESTMENT = 200;

    const deadline = Math.floor((Date.now() + MS_IN_SECONDS * DEADLINE_AMOUNT_SECS) / MS_IN_SECONDS);
    const capitalCallAccount = anchor.web3.Keypair.generate();

    await program.methods
      .createCapitalCall(
        new anchor.BN(deadline), new anchor.BN(REQUIRED_CALL_AMOUNT), vaultNonce, lpNonce
      )
      .accounts(
        {
          owner: provider.publicKey,
          capitalCallAccount: capitalCallAccount.publicKey,
          vault,
          vaultOwner: vaultPda
        }
      )
      .signers([capitalCallAccount])
      .rpc();

    await usdcMint.mintTo(
      investorUSDCAccount,
      usdcMintAuthority.publicKey,
      [usdcMintAuthority],
      MINT_AMOUNT
    );

    let [callInvestingInfoPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [investor.publicKey.toBuffer(), capitalCallAccount.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .investCapital(new anchor.BN(USDC_INVESTMENT))
      .accounts(
        {
          initializer: investor.publicKey,
          capitalCallAccount: capitalCallAccount.publicKey,
          capitalCallVault: vault,
          from: investorUSDCAccount,
          callInvestingInfoPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        }
      )
      .signers([investor])
      .rpc();

    let callInvestingInfoDetails = await program.account.callInvestingInfo.fetch(callInvestingInfoPda);
    assert.equal(callInvestingInfoDetails.depositedAmount.toNumber(), USDC_INVESTMENT);
    assert.equal(callInvestingInfoDetails.lpTokensClaimed, false);

    await sleep(DEADLINE_AMOUNT_SECS * MS_IN_SECONDS * 2);

    await program.methods
      .claimLpTokens()
      .accounts(
        {
          initializer: investor.publicKey,
          capitalCallAccount: capitalCallAccount.publicKey,
          capitalCallVault: vault,
          to: investorLPTokenAccount,
          callInvestingInfoPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          lpTokenMint: lpMint.publicKey,
          lpMintAuthority: lpAuthorityPda
        }
      )
      .signers([investor])
      .rpc();

    callInvestingInfoDetails = await program.account.callInvestingInfo.fetch(callInvestingInfoPda);
    assert.equal(callInvestingInfoDetails.lpTokensClaimed, true);

    const usdcAmount = (await usdcMint.getAccountInfo(investorUSDCAccount)).amount.toNumber();
    assert.equal(usdcAmount, MINT_AMOUNT - USDC_INVESTMENT);

    const investorLPTokensAmount = (await lpMint.getAccountInfo(investorLPTokenAccount)).amount.toNumber();
    assert.equal(investorLPTokensAmount, Math.floor(USDC_INVESTMENT / LP_PRICE));
  });

  it("Capital call is not fulfilled on time and USDC tokens are withdrawn back", async () => {
    const {
      usdcMintAuthority,
      usdcMint,
      vaultPda,
      vaultNonce,
      vault,
      lpMint,
      lpNonce,
      investor,
      investorLPTokenAccount,
      investorUSDCAccount,
    } = await setupTestingAccounts();

    const DEADLINE_AMOUNT_SECS = 5;
    const REQUIRED_CALL_AMOUNT = 1000;
    const USDC_INVESTMENT = 10;

    const deadline = Math.floor((Date.now() + MS_IN_SECONDS * DEADLINE_AMOUNT_SECS) / MS_IN_SECONDS);
    const capitalCallAccount = anchor.web3.Keypair.generate();

    await program.methods
      .createCapitalCall(
        new anchor.BN(deadline), new anchor.BN(REQUIRED_CALL_AMOUNT), vaultNonce, lpNonce
      )
      .accounts(
        {
          owner: provider.publicKey,
          capitalCallAccount: capitalCallAccount.publicKey,
          vault,
          vaultOwner: vaultPda
        }
      )
      .signers([capitalCallAccount])
      .rpc();

    await usdcMint.mintTo(
      investorUSDCAccount,
      usdcMintAuthority.publicKey,
      [usdcMintAuthority],
      MINT_AMOUNT
    );

    let [callInvestingInfoPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [investor.publicKey.toBuffer(), capitalCallAccount.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .investCapital(new anchor.BN(USDC_INVESTMENT))
      .accounts(
        {
          initializer: investor.publicKey,
          capitalCallAccount: capitalCallAccount.publicKey,
          capitalCallVault: vault,
          from: investorUSDCAccount,
          callInvestingInfoPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        }
      )
      .signers([investor])
      .rpc();

    let callInvestingInfoDetails = await program.account.callInvestingInfo.fetch(callInvestingInfoPda);
    assert.equal(callInvestingInfoDetails.depositedAmount.toNumber(), USDC_INVESTMENT);
    assert.equal(callInvestingInfoDetails.lpTokensClaimed, false);

    await sleep(DEADLINE_AMOUNT_SECS * MS_IN_SECONDS * 2);

    await program.methods
      .withdrawCapital()
      .accounts(
        {
          initializer: investor.publicKey,
          capitalCallAccount: capitalCallAccount.publicKey,
          capitalCallVault: vault,
          to: investorUSDCAccount,
          callInvestingInfoPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          vaultOwner: vaultPda
        }
      )
      .signers([investor])
      .rpc();

    callInvestingInfoDetails = await program.account.callInvestingInfo.fetch(callInvestingInfoPda);
    assert.equal(callInvestingInfoDetails.depositedAmount.toNumber(), 0);
    assert.equal(callInvestingInfoDetails.lpTokensClaimed, false);

    const usdcAmount = (await usdcMint.getAccountInfo(investorUSDCAccount)).amount.toNumber();
    assert.equal(usdcAmount, MINT_AMOUNT);

    const investorLPTokensAmount = (await lpMint.getAccountInfo(investorLPTokenAccount)).amount.toNumber();
    assert.equal(investorLPTokensAmount, 0);
  });
});
