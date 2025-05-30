use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //Number of accounts is comming so first we will be `iter()` means it will go through one by one.
    let mut iter = accounts.iter();
    //data account is simple account , we can create `new Keypair()`, it will create public and private .
    //We know , we can store Lamports and also data inside account and we can use them
    //`next_account_info` it will provide access specific item that has been iterated
    let data_account = next_account_info(&mut iter)?;
    //`double_program_account` is other smart contract that already deployed we can iteract with double_program_contract ,thats called CPI (one program call to another program , they can do max 4 interanally)
    // We can pass contract through client side for cpi call
    let double_program_account = next_account_info(&mut iter)?;

    //We can create instruction that will take program_id , accounts and data for signing the trasaction
    let instruction = Instruction {
        program_id: *double_program_account.key,
        accounts: vec![AccountMeta::new(*data_account.key, true)],
        data: instruction_data.to_vec(),
    };

    //Not necessary to pass third arguments here , it will be sign internally and
    //If we use pda so we will use ` invoke_signed`
    invoke(&instruction, &[data_account.clone()])?;

    ProgramResult::Ok(())
}
