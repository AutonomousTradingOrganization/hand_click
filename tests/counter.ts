import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { Hand } from "../target/types/hand";
import { expect } from "chai";

describe("CPI from Hand to Counter", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const counterProgram     = anchor.workspace.Counter as Program<Counter>;
  const handProgram        = anchor.workspace.Hand as Program<Hand>;
  const dataAccountKeypair = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await counterProgram.methods
      .initialize()
      .accounts({
        counterDataAccount: dataAccountKeypair.publicKey,
        signer            : provider.wallet.publicKey,
        systemProgram     : anchor.web3.SystemProgram.programId,
      })
      .signers([dataAccountKeypair])
      .rpc();
  });

  it("Can increment counter!", async () => {
    // Add your test here.
    const tx = await handProgram.methods
      .handClick()
      .accounts({
        counterDataAccount: dataAccountKeypair.publicKey,
        counterProgram    : counterProgram.programId,
      })
      .rpc();
  });

   it("Can assert value in Counter's data account equals 1", async () => {

    const CounterAccountValue = (
      await counterProgram.account.counterData.fetch(dataAccountKeypair.publicKey)
    ).value.toNumber();

    expect(CounterAccountValue).to.equal(1);
  });
});
