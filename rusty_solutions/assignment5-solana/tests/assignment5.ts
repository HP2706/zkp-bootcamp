import * as anchor from "@coral-xyz/anchor";

import { Assignment5 } from "../target/types/assignment5";
import { Program } from "@coral-xyz/anchor";

describe("assignment5", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Assignment5 as Program<Assignment5>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    const txDetails = await program.provider.connection.getTransaction(tx, {
      maxSupportedTransactionVersion: 0,
      commitment: "confirmed",
    });
  
    const logs = txDetails?.meta?.logMessages || null;
    console.log("txdetails", txDetails);
    if (!logs) {
      console.log("No logs found");
    } else {
      console.log("Logs found", logs);
      // You can then assert on the presence of specific log messages
    }
  });
});