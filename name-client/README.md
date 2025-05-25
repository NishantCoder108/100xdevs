### Understanding:
- Initialize bun project `bun init`
  

  
---

### What i want :
- I want to interact with `Solana Program`
- Here , I will use LiteSVM (Recommended to testing)


---
### What Steps :
- First setup bun project 
- Add @solana/web3.js `bun add @solana/web3.js`
- For 8 bytes data , how much sol is needed `solana rent 8`
- I want to store data (Name) so for need `0.00094656 SOL` much solana.
- For running validator locally , we run `solana-test-validator` , it will run locally and simulate the network.
- We can connect with network , so we use `Connection` and here we can add url of network (localnet, devnet, mainnet)
- That we are creating `data account` , we must to pass as sign `await connection.sendTransaction(tx, [kp, dataAccount]);`  , so they will also sign the signature.


### Important Stuff:
- For creating simple account we can use `kp = new Keypair()` they will have both private and public key and it's have `SOL` balances . 
- Another account  where they have `SOL` and `SPACE` for bytes data 
   ```js
   const instruction = SystemProgram.createAccount({
    fromPubkey: kp.publicKey,
    newAccountPubkey: dataAccount.publicKey,
    lamports: 1 * LAMPORTS_PER_SOL,
    space: 8,
    programId: SystemProgram.programId,
  });
   ```

- Other account , they will have both `SOL` and `SPACE` but they have not private key , so they will not `sign` the transaction , so program will be sign on behalf of `PDA` 
---

### Project Setup:

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run index.ts
```

This project was created using `bun init` in bun v1.2.14. [Bun](https://bun.sh) is a fast all-in-one JavaScript runtime.


