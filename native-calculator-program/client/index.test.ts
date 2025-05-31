import * as path from "path";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { LiteSVM } from "litesvm";
import { describe, beforeEach, test, expect, beforeAll } from "bun:test";
import { serialize } from "v8";

describe("Counter Program Tests", () => {
  let svm: LiteSVM;
  let programId: PublicKey;
  let dataAccount: Keypair;
  let userAccount: Keypair;

  // const programPath = path.join(import.meta.dir, "sumcontract.so");
  const programPath = path.resolve(
    __dirname,
    "../target/deploy/native_calculator_program.so"
  );
  beforeAll(() => {
    svm = new LiteSVM();

    programId = PublicKey.unique();

    svm.addProgramFromFile(programId, programPath);

    dataAccount = new Keypair();

    userAccount = new Keypair();

    svm.airdrop(userAccount.publicKey, BigInt(LAMPORTS_PER_SOL));

    let ix = SystemProgram.createAccount({
      fromPubkey: userAccount.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),
      space: 4,
      programId,
    });
    let tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(userAccount, dataAccount);
    svm.sendTransaction(tx);
    svm.expireBlockhash();
  });

  test("ini", () => {
    const instruction = new TransactionInstruction({
      programId,
      keys: [
        { pubkey: dataAccount.publicKey, isSigner: true, isWritable: true },
      ],
      data: Buffer.from([0]),
    });

    const transaction = new Transaction().add(instruction);
    transaction.recentBlockhash = svm.latestBlockhash();
    transaction.feePayer = userAccount.publicKey;
    transaction.sign(dataAccount, userAccount);
    let signature = svm.sendTransaction(transaction);

    const updatedAccountData = svm.getAccount(dataAccount.publicKey);
    if (!updatedAccountData) {
      throw new Error("Account not found");
    }

    expect(updatedAccountData.data[0]).toBe(1);
    expect(updatedAccountData.data[1]).toBe(0);
    expect(updatedAccountData.data[2]).toBe(0);
    expect(updatedAccountData.data[3]).toBe(0);
  });

  test("It doubles the counter value", () => {
    const instruction = new TransactionInstruction({
      programId,
      keys: [
        { pubkey: dataAccount.publicKey, isSigner: true, isWritable: true },
      ],
      data: Buffer.from([1]),
    });

    const transaction = new Transaction().add(instruction);
    transaction.recentBlockhash = svm.latestBlockhash();
    transaction.feePayer = userAccount.publicKey;
    transaction.sign(dataAccount, userAccount);
    let signature = svm.sendTransaction(transaction);

    const updatedAccountData = svm.getAccount(dataAccount.publicKey);
    if (!updatedAccountData) {
      throw new Error("Account not found");
    }

    expect(updatedAccountData.data[0]).toBe(2);
    expect(updatedAccountData.data[1]).toBe(0);
    expect(updatedAccountData.data[2]).toBe(0);
    expect(updatedAccountData.data[3]).toBe(0);
  });

  test("It halves the counter value", () => {
    const instruction = new TransactionInstruction({
      programId,
      keys: [
        { pubkey: dataAccount.publicKey, isSigner: true, isWritable: true },
      ],
      data: Buffer.from([2]),
    });

    const transaction = new Transaction().add(instruction);
    transaction.recentBlockhash = svm.latestBlockhash();
    transaction.feePayer = userAccount.publicKey;
    transaction.sign(dataAccount, userAccount);
    let signature = svm.sendTransaction(transaction);

    const updatedAccountData = svm.getAccount(dataAccount.publicKey);
    if (!updatedAccountData) {
      throw new Error("Account not found");
    }

    expect(updatedAccountData.data[0]).toBe(1);
    expect(updatedAccountData.data[1]).toBe(0);
    expect(updatedAccountData.data[2]).toBe(0);
    expect(updatedAccountData.data[3]).toBe(0);
  });

  //   test("Add 2 to the counter value", async () => {
  // //     class AddInstruction {
  // //       instruction = 3; // 3 = Add
  // //       amount: number;

  // //       constructor(props: { amount: number }) {
  // //         this.amount = props.amount;
  // //       }
  // //     }

  // //     const schema = new Map([
  // //       [AddInstruction, {
  // //         kind: "struct",
  // //         fields: [
  // //           ["instruction", "u8"],
  // //           ["amount", "u32"],
  // //         ],
  // //       }],
  // //     ]);

  // //     const addInstructionData = serialize(schema, new AddInstruction({ amount: 2 }));

  // //     const instruction = new TransactionInstruction({
  // //       programId,
  // //       keys: [
  // //         {
  // //           pubkey: dataAccount.publicKey,
  // //           isSigner: true,
  // //           isWritable: true,
  // //         },
  // //       ],
  // //       data: Buffer.from(addInstructionData),
  // //     });

  // //     const tx = new Transaction().add(instruction);
  // //     tx.feePayer = dataAccount.publicKey;
  // //     tx.recentBlockhash = svm.latestBlockhash();
  // //     tx.sign(dataAccount);

  // //     const res = svm.sendTransaction(tx);
  // //     console.log(res.toString());

  // //     const accountInfo = svm.getAccount(dataAccount.publicKey);
  // //     const counter = borsh.deserialize(schema, AddInstruction, accountInfo.data);
  // //     console.log("New count:", counter.amount);
  // //   });

  //   });
});
