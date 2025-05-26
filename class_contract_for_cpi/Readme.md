
# 🦀 Cross Program Invocation (CPI)
CPI (Cross-Program Invocation) is a method that allows one program to interact with another program on Solana.

---

## 📚 Table of Contents

- [About](#about)
- [How to Run](#how-to-run)
- [What I Learned](#what-i-learned)
- [Code Structure](#code-structure)
- [Resources](#resources)

---

## 🔍 About

This folder is for learning CPI (Cross-Program Invocation). I'm writing a smart contract that creates a PDA (Program Derived Address). To create the PDA, the program interacts with the System Program using CPI. The user is the signer, but the PDA signs programmatically using invoke_signed() to create the account.

---

## 🧪 How to Run

```bash
# Create new solana program
cargo init --lib

# Add solana dependency
cargo add solana-program

# Build the program
cargo build-bpf
````

---

## 🧠 What I Learned

### 1. `cargo init --lib`

* Creates a library crate, required for Solana programs.
* `cargo init .` is used for CLI apps, not needed here.

---

### 2. solana-program = Like Express in Node.js

* You can build logic from scratch OR use `solana-program` (like using Express).
* Add it like this:

  ```bash
  cargo add solana-program
  ```

---

### 3. instruction\_data: `&[u8]`

* `u8` = 1 byte = value from **0 to 255**
* Example: `[1, 10, 20]` → 3 bytes
* Structure:

  * First byte = **instruction ID**
  * Rest = **data for that instruction**

---

### 4. accounts: `&[AccountInfo]`

* These are all accounts smart contract needs.
* Example: user, PDA, system\_program.
* Solana is fast because all accounts are passed directly.

```rust
let iter = &mut accounts.iter();
let pda = next_account_info(iter)?;
```

* `iter()` = like `.map()` in JS
* `next_account_info()` = fetch account one by one



---

### 5.  `&[&[&[u8]]]`

* This is used to **pass signer seeds** when calling `invoke_signed()`.
* It's a **3D array**:

  * `&[u8]` → one seed (like a string or number in bytes (0-255)).
  * `&[&[u8]]` → one signer (a list of seeds).
  * `&[&[&[u8]]]` → multiple signers (usually we pass one PDA, so one set of seeds).



####   Example:

```rust
let seeds = &[b"pda-seed", user.key.as_ref(), &[bump]];
invoke_signed(
  ...,
  &[&[b"pda-seed", user.key.as_ref(), &[bump]]], // This is &[&[u8]]
)
```

## 📁 Code Structure

```
solana-learning/
├── src/
│   └── lib.rs        # Main program logic
├── Cargo.toml        # Dependencies
├── README.md         # Learning notes
```

---

## 🔗 Resources
* [Solana CPI (Cross Program Invocation)](https://solana.com/docs/core/cpi)
* [Solana Docs](https://docs.solana.com/)
* [Solana Cookbook](https://solanacookbook.com/)
* [Rust Language Book](https://doc.rust-lang.org/book/)

