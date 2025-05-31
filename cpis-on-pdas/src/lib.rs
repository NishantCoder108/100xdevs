use std::collections::binary_heap::Iter;

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::{ProgramResult, entrypoint},
    lamports, msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Instruction Data Length : {}", instruction_data.len());
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let bump = instruction_data[0];
    let mut iter = accounts.iter();
    let user_account = next_account_info(&mut iter)?;
    let pda_account = next_account_info(&mut iter)?;
    let system_program_id = next_account_info(&mut iter)?;

    //For lamports ,we can give directly number of lamports but here we can use Rent method , to necessary to use this
    // 4 bytes = u32
    let (expected_pda, _) =
        Pubkey::find_program_address(&[b"user1", user_account.key.as_ref()], program_id);

    if expected_pda != *pda_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    let rent = Rent::get()?;
    let space = 4;
    let lamports = rent.minimum_balance(space);
    let ix = create_account(
        user_account.key,
        pda_account.key,
        lamports,
        space as u64, //we use more space so initialize first then
        program_id,
    );

    let signer_seed = &[b"user1", user_account.key.as_ref(), &[bump]];

    invoke_signed(
        &ix,
        &[
            user_account.clone(),
            pda_account.clone(),
            system_program_id.clone(),
        ],
        &[signer_seed],
    )
}

/*
 - We will take pda account , user account , program Id from client side to create contract
- create instruction for cpi call ,so for that we will use instruction , if they are not well known ,but they are already known or wrapper so we can directly use this as a instruction


 */
