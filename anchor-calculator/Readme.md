# Anchor Calculator
Anchor is a framework that help to write smart contract easily compare to native smart contract. It provide `macro` , `security checks` , automatically `idl` generation , deployment , `deseralization` and `serialization` and testing from client side and also frontend side.
Here ,It is the simple operation using anchor such as doubling , half, subtract , adding.


### What I Learned
- Every instruction in `native contract` will make `struct` and we can add on top of `struct` is `#[derive(Accounts)]` , so writing contract is easy.
- We have some terms that is in actual is `macro` such as `#[derive(Accounts)]` , `#[program]` and `#[account]` ,that is most common we will generally see
- `#[account]` is specially used for creating new account in solana.
- With anchor , we can `intialialize` account but in native contrac we were doing initializing account from client side but here if you have payer , new account and it automatically create new account .