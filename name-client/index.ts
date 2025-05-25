import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

const connection = new Connection("http://127.0.0.1:8899");

async function main() {
  const kp = new Keypair();

  const sign = await connection.requestAirdrop(
    kp.publicKey,
    3 * LAMPORTS_PER_SOL
  );

  console.log({ sign });

  await connection.confirmTransaction(sign);
  const bal = await connection.getBalance(kp.publicKey);

  console.log({ bal });
}

main();
