
# Proxy Program Contract Test (CPI)
- Proxy contract or we can say middle contract that will interact with other program . 
- At client side , they will interact with proxy contract and pass needed argument like `proxy` program id and accounts list and data . 
- Inside accounts list , we will pass other program's id (double contract's program id) and for doubling number so we will also send data accounts that's owner would be `doubleContract` program id . we can create data account with their programId .
- We call proxy contract and where write instruction where we can pass accounts of `doubleContract` address and data account and call to `invoke` function
- If we are using `PDA` then we call to `invoke_signed` function to signing the signer.


## program-cpi-test

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run index.ts
```

This project was created using `bun init` in bun v1.2.14. [Bun](https://bun.sh) is a fast all-in-one JavaScript runtime.


## Understanding:
- 