## CPI in SOLANA
- Cross Program Invocation is a method that enable one program call to another program . 
- Max call of CPI is 4
---

### About
- This contract is just double the number
   

---
### What I Learn
- Create project `cargo init --lib` and add `solana-program` library
- Add `crate-type=["cdylib", "lib"]` for generating .so file and for solana contract 
- Add `borsh` and `borsh-derive` for serialiazation and deserialization
- For build , `cargo build-sbf`  ,it will generate .so file
- Create separate directory for testing of contract.
  ```bash
  mkdir client
  cd client
  bun init
  bun add litesvm
  bun add @solana/web3.js

  ```

- If same program deploy , it will generate different program_id, but if we use same program's keypair json file then it will generate same program_id
  ```bash
  solana program deploy --program-id <your_program-keypair.json>   target/deploy/your_program.so


  ```  

- `OnChainData::try_from_slice(*data_account.data.borrow_mut())?;`
: Immediately dereference and reference to mutable data
  ```
  let mut counter = OnChainData::try_from_slice(*data_account.data.borrow_mut())?;
  ```

- We should to pass number of account that is needed to contract , might account are using in contract but we will be pass , that is required otherwise test will be fail.  

- After every transaction ,we must to `expireBlockhash` other wise it will take previous blockhash so output will not come as expected.

  ```js
   
  function double() {
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
    litesvm.expireBlockhash();
  }
  ```

- Account data come in bytes not in bit.
  ```js
  const dataAcc = litesvm.getAccount(dataAccount.publicKey);
   console.log(dataAcc); //data: Uint8Array(4) [ 8, 0, 0, 0 ],
  expect(dataAcc?.data[0]).toBe(8);
  ```  

- In my test, i simply seeing `Uint8Array(4) [8,0,0,0]` and expecting result
  ```js
  expect(dataAcc?.data[0]).toBe(8);
  ```
- We can also take reference how can convert in ts for testing by `LiteSVM` using `borsh`
-  - https://github.com/100xdevs-cohort-3/pdas-and-cpis/blob/master/1-client-double/index.test.ts#L16