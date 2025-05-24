use borsh::{BorshDeserialize, BorshSerialize};
// use solana_program::{
//     account_info::{AccountInfo, next_account_info},
//     entrypoint,
//     entrypoint::ProgramResult,
//     program_error::ProgramError,
//     pubkey::Pubkey,
// };

use solana_account_info::{AccountInfo, next_account_info};
use solana_program::program_error::ProgramError;
use solana_program_entrypoint::{ProgramResult, entrypoint};
use solana_pubkey::Pubkey;
//define the onchain state data structure
#[derive(BorshDeserialize, BorshSerialize)]
struct CounterState {
    count: u32,
}

//define the instruction
#[derive(BorshDeserialize, BorshSerialize)]
enum Instruction {
    Init,
    Double,
    Half,
    Add { amount: u32 },
    Subtract { amount: u32 },
}

//define the main function
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //get the data account from  the accounts array
    let mut iter = accounts.iter();
    let data_account = next_account_info(&mut iter)?;

    if !data_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    //parse the instruction
    let instruction = Instruction::try_from_slice(instruction_data)?;
    let mut counter_state = CounterState::try_from_slice(&data_account.data.borrow())?;

    match instruction {
        Instruction::Init => {
            counter_state.count = 1;
        }
        Instruction::Add { amount } => {
            counter_state.count = counter_state.count + amount;
        }
        Instruction::Double => {
            counter_state.count = counter_state.count * 2;
        }
        Instruction::Half => {
            counter_state.count = counter_state.count / 2;
        }
        Instruction::Subtract { amount } => {
            counter_state.count = counter_state.count - amount;
        }
    }

    counter_state.serialize(&mut *data_account.data.borrow_mut())?;
    Ok(())
}
