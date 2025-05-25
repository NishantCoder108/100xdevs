import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";

const connection = new Connection("http://127.0.0.1:8899");

async function main() {
  const kp = new Keypair();
  const dataAccount = new Keypair();

  const sign = await connection.requestAirdrop(
    kp.publicKey,
    3 * LAMPORTS_PER_SOL
  );

  console.log({ sign });

  await connection.confirmTransaction(sign);
  const bal = await connection.getBalance(kp.publicKey);

  console.log({ bal });

  //write instruction for data account where we store both data and sol
  const instruction = SystemProgram.createAccount({
    fromPubkey: kp.publicKey,
    newAccountPubkey: dataAccount.publicKey,
    lamports: 1 * LAMPORTS_PER_SOL,
    space: 8,
    programId: SystemProgram.programId,
  });

  let tx = new Transaction().add(instruction);
  tx.feePayer = kp.publicKey;
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

  await connection.sendTransaction(tx, [kp, dataAccount]);

  console.log("Data Account Public Key : ", dataAccount.publicKey.toBase58());
}

main();
