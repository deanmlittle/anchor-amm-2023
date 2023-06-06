import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Amm2023 } from "../target/types/amm_2023";

describe("amm-2023", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Amm2023 as Program<Amm2023>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
