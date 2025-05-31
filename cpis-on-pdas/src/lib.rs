use std::collections::binary_heap::Iter;

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::{ProgramResult, entrypoint},
    msg,
    program::invoke_signed,
    pubkey::{self, Pubkey},
    system_instruction::create_account,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let user_account = next_account_info(&mut iter)?;
    let pda_account = next_account_info(&mut iter)?;
    let system_program_id = next_account_info(&mut iter)?;

    //For lamports ,we can give directly number of lamports but here we can use Rent method , to necessary to use this
    // 4 bytes = u32
    let (pda, bump) = Pubkey::find_program_address(
        &[b"user1", user_account.key.as_ref()],
        system_program_id.key,
    );

    let ix = create_account(user_account.key, &pda, 1000000000, 4, system_program_id.key);

    let signer_seed = &[b"user1", user_account.key.as_ref(), &[bump]];

    invoke_signed(&ix, accounts, &[signer_seed])
}

/*
 - We will take pda account , user account , program Id from client side to create contract
- create instruction for cpi call ,so for that we will use instruction , if they are not well known ,but they are already known or wrapper so we can directly use this as a instruction


 */
