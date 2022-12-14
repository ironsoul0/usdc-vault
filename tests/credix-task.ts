import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CredixTask } from "../target/types/credix_task";

describe("credix-task", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CredixTask as Program<CredixTask>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
