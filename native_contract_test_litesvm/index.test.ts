import path from "path";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { LiteSVM } from "litesvm";
import { describe, beforeAll, beforeEach, test, expect } from "bun:test";

describe("Native Contract Test", () => {
  let svm: LiteSVM;
  let programId: PublicKey;
  let dataAccount: Keypair;
  let userAccount: Keypair;

  const PROGRAM_PATH = path.join(import.meta.dir, "native_contract.so");

  beforeAll(() => {
    svm = new LiteSVM();
    programId = PublicKey.unique();
    dataAccount = Keypair.generate();
    userAccount = Keypair.generate();

    svm.addProgramFromFile(programId, PROGRAM_PATH);

    svm.airdrop(userAccount.publicKey, BigInt(LAMPORTS_PER_SOL));

    let instruction = SystemProgram.createAccount({
      fromPubkey: userAccount.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),
      space: 4,
      programId: programId,
    });

    let tx = new Transaction().add(instruction);
    // tx.feePayer = userAccount.publicKey;
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(userAccount, dataAccount);

    svm.sendTransaction(tx);
    svm.expireBlockhash();
  });

  test("Initialize", () => {
    const instruction = new TransactionInstruction({
      programId,
      keys: [
        {
          pubkey: dataAccount.publicKey,
          isSigner: true,
          isWritable: true,
        },
      ],
      data: Buffer.from([0]),
    });
  });
});
