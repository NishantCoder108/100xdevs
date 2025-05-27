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