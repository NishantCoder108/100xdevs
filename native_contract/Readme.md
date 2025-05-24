## Native Contract

- Creating new project : `cargo new native_contract`
- For Existing directory , we can create inside :` cargo init --lib`
- For build :  `cargo build-sbf`
- For deploy : `solana program deploy target/deploy/native_contract.so`


#### Understanding:
- lib : `crate-type = ["lib", "cdylib"]` Here lib is used for writing the solana smart contract and we can write tests inside (dot).rs file together , or we can import file where we can write unit tests . It is give standard to build the smart contract to deploy on solana blockchain. cdylib is used for deployment , so basically it will create (.so) file to deploy on solana blockchain and frontend can interact with this smart contract. 