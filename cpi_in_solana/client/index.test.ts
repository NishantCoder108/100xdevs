import { test, expect } from "bun:test";
import path from "path";
import fs from "fs";
import { LiteSVM } from "litesvm";
import {
  PublicKey,
  Transaction,
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  TransactionInstruction,
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
  // console.log(
  //   "Payer :",
  //   payer.publicKey.toBase58(),
  //   "Receiver :",
  //   receiver.toBase58()
  // );
  expect(balanceAfter).toBe(transferLamports);
});

test("double the number", () => {
  const programId = PublicKey.unique();
  const litesvm = new LiteSVM();
  const soPath = path.resolve(__dirname, "../target/deploy/cpi_in_solana.so");
  if (!fs.existsSync(soPath)) {
    throw new Error("Program .so file not found.");
  }
  litesvm.addProgramFromFile(programId, soPath);
  const user = new Keypair();
  litesvm.airdrop(user.publicKey, BigInt(5000000000));

  const dataAccount = new Keypair();

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

  //now ,interact dataAccount with onchain
  const ixs2 = new TransactionInstruction({
    programId: programId,
    keys: [
      {
        pubkey: dataAccount.publicKey,
        isSigner: true,
        isWritable: true,
      },
    ],
    data: Buffer.from([]),
  });

  const tx2 = new Transaction().add(ixs2);
  tx2.recentBlockhash = litesvm.latestBlockhash();
  tx2.feePayer = user.publicKey;

  tx2.sign(user, dataAccount);

  litesvm.sendTransaction(tx2);

  const dataAcc = litesvm.getAccount(dataAccount.publicKey);
  console.log("NewDataAcc:", dataAccount.publicKey.toBase58());
  console.log(dataAcc);
  const userBal = litesvm.getBalance(user.publicKey);
  const dataAccBal = litesvm.getBalance(dataAccount.publicKey);

  console.log({ userBal }, { dataAccBal });
  expect(dataAcc?.data[0]).toBe(1);
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
