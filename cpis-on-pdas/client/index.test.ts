import { test, expect, beforeAll, describe } from "bun:test";
import { LiteSVM } from "litesvm";
import path from "path";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import fs from "fs";

describe("Create pda from client", () => {
  let liveSvm: LiteSVM;
  let pda: PublicKey;
  let bump: number;
  let programId: PublicKey;
  let payer: Keypair;

  beforeAll(() => {
    liveSvm = new LiteSVM();
    programId = PublicKey.unique();
    payer = Keypair.generate();

    const cpiProgramPath = path.resolve(
      __dirname,
      "../target/deploy/cpis_on_pdas.so"
    );
    if (!fs.existsSync(cpiProgramPath)) {
      throw new Error("Program .so file not found.");
    }
    liveSvm.addProgramFromFile(programId, cpiProgramPath);
    liveSvm.airdrop(payer.publicKey, BigInt(100000000000));

    [pda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("user1"), payer.publicKey.toBuffer()],
      programId
    );

    let ix = new TransactionInstruction({
      keys: [
        {
          pubkey: payer.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: pda,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId,
      data: Buffer.from([bump]),
    });

    const tx = new Transaction().add(ix);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = liveSvm.latestBlockhash();
    tx.sign(payer);
    let res = liveSvm.sendTransaction(tx);
    console.log(res.toString());
  });

  test("should create pda", () => {
    const balance = liveSvm.getBalance(pda);
    console.log(balance);
    expect(Number(balance)).toBeGreaterThan(0);
    expect(Number(balance)).toBe(918720);
  });
});
