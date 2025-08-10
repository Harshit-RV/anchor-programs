import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Staking } from "../target/types/staking";

describe("staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.staking as Program<Staking>;

  it("Is initialized!", async () => {
    const tx = await program.methods.createPda().rpc();
    console.log("Your transaction signature", tx);
  });

  const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("client1"), anchor.getProvider().publicKey.toBuffer()],
    program.programId
  );

  const userPubkey = anchor.getProvider().publicKey;

  it("Get initial points!", async () => {
    const simulation = await program.methods
      .getPoints()
      .accounts({
        user: userPubkey,
      })
      .simulate();
    
    console.log("Simulation result:", simulation);
    
    const tx = await program.methods
      .getPoints()
      .accounts({
        user: userPubkey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Stake some SOL", async () => {
    const tx = await program.methods.stake(new anchor.BN(1_000_000_000)).accounts({
      user: userPubkey,
    }).rpc()
  })

  it("Get initial points!", async () => {
    const simulation = await program.methods
      .getPoints()
      .accounts({
        user: userPubkey,
      })
      .simulate();
    
    console.log("Simulation result:", simulation);
    
    const tx = await program.methods
      .getPoints()
      .accounts({
        user: userPubkey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
