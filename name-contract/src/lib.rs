use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    system_program::ID as SYSTEM_PROGRAM_ID,
    sysvar::{Sysvar, rent::Rent},
};

#[derive(BorshDeserialize, BorshSerialize)]
enum InterestField {
    Blockchain,
    AI,
    Web2,
}

#[derive(BorshDeserialize, BorshSerialize)]
struct UserAccount {
    name: String,
    interest_field: InterestField,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();

    let pda_account = next_account_info(&mut iter)?;
    let signer = next_account_info(&mut iter)?;
    let system_program = next_account_info(&mut iter)?;

    let (pda, bump) = Pubkey::find_program_address(&[b"user", signer.key.as_ref()], program_id);
    if *pda_account.key != pda {
        return Err(ProgramError::InvalidSeeds);
    }

    // Create account using invoke_signed
    let rent = Rent::get()?.minimum_balance(1000);
    let ix =
        system_instruction::create_account(signer.key, pda_account.key, rent, 1000, program_id);

    invoke_signed(
        &ix,
        &[signer.clone(), pda_account.clone(), system_program.clone()],
        &[&[b"user", signer.key.as_ref(), &[bump]]],
    )?;

    // Store the user data
    let user_data = UserAccount::try_from_slice(instruction_data)?;
    user_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}

// use borsh::{BorshDeserialize, BorshSerialize};
// use solana_program::{
//     account_info::{AccountInfo, next_account_info},
//     entrypoint,
//     entrypoint::ProgramResult,
//     program::invoke_signed,
//     pubkey::Pubkey,
//     system_instruction::create_account,
//     system_program::ID as SYSTEM_PROGRAM_ID,
// };

// #[derive(BorshDeserialize, BorshSerialize)]
// enum InterestField {
//     Blockchain,
//     AI,
//     Web2,
// }

// #[derive(BorshDeserialize, BorshSerialize)]
// struct UserAccount {
//     name: String,
//     interest_field: InterestField,
// }
// entrypoint!(process_instruction);

// fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     let mut iter = accounts.iter();
//     let user_account = next_account_info(&mut iter)?;

//     let (user_pda, bump) =
//         Pubkey::find_program_address(&[b"user", user_account.key.as_ref()], program_id);

//     let ix = create_account(
//         user_account.key,
//         &user_pda,
//         1000000000,
//         4,
//         &SYSTEM_PROGRAM_ID,
//     );
//     let signer_seeds = &[b"user", user_account.key.as_ref(), &[bump]];

//     invoke_signed(&ix, accounts, &[signer_seeds])?;

//     // Deserialize instruction data to get the new name
//     let payload = UserAccount::try_from_slice(instruction_data)?;
//     let mut user_data = UserAccount::try_from_slice(&user_account.data.borrow())?;
//     user_data.name = payload.name;
//     user_data.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

//     Ok(())
// }
