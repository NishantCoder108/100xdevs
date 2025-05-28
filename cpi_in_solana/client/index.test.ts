import { test, expect } from "bun:test";

import { LiteSVM } from "litesvm";
import {
  PublicKey,
  Transaction,
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

test("one transfer", () => {
  const svm = new LiteSVM();
  const payer = new Keypair();
  svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));
  const receiver = PublicKey.unique();
  const blockhash = svm.latestBlockhash();
  const transferLamports = 1_000_000n;
  const ixs = [
    SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: receiver,
      lamports: transferLamports,
    }),
  ];
  const tx = new Transaction();
  tx.recentBlockhash = blockhash;
  tx.add(...ixs);
  tx.sign(payer);
  svm.sendTransaction(tx);
  const balanceAfter = svm.getBalance(receiver);
  console.log(
    "Payer :",
    payer.publicKey.toBase58(),
    "Receiver :",
    receiver.toBase58()
  );
  expect(balanceAfter).toBe(transferLamports);
});

test("double the number", () => {
  const litesvm = new LiteSVM();
  const user = new Keypair();
  litesvm.airdrop(user.publicKey, BigInt(5000000000));

  const dataAccount = new Keypair();
  const programId = PublicKey.unique();

  const blockhash = litesvm.latestBlockhash();
  const ixs = SystemProgram.createAccount({
    fromPubkey: user.publicKey,
    lamports: Number(litesvm.minimumBalanceForRentExemption(BigInt(4))),
    newAccountPubkey: dataAccount.publicKey,
    space: 4,
    programId: programId,
  });

  const tx = new Transaction().add(ixs);
  tx.recentBlockhash = blockhash;
  tx.feePayer = user.publicKey;

  tx.sign(user, dataAccount);

  litesvm.sendTransaction(tx);
});

/**
 * 0  Programatically (user), doubling the counter everytime
 * 1. create user account that will sign
 * 2. create data account where store the data
 * 3. Add lamports to user
 * 4. Create instruction where we can pass lamports and size to create data account
 *    - SystemProgram.createAccount()
 * 5. Sign the trasaction
 * 6. We can write test logic to double
 *
 *
 * *Question:
 * 1. When i just create function like process_instruction() and here we pass argument like program_id, accounts, instruction_data , so for program_id , when i deploy that will be program_id , so how can i test by liteSVM and pass program_id of system program
 */
