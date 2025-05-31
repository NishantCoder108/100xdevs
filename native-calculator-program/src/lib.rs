use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::{ProgramResult, entrypoint},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize)]
struct CounterState {
    count: u32,
}

//impl is where we can write logic about struct i.e. structure
// impl CounterState {
//     fn increment(&mut self) {
//         self.count += 1
//     }
// }

//define the instruction
#[derive(BorshDeserialize, BorshSerialize)]
enum Instruction {
    Init,
    Double,
    Half,
    Add { amount: u32 },
    Subtract { amount: u32 },
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //get the acccount
    let mut acc_iter = accounts.iter();
    let data_account = next_account_info(&mut acc_iter)?;

    if !data_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!("Data Account : {}", data_account.key);

    //instruction we can access here
    let instruction = Instruction::try_from_slice(instruction_data)?;
    let mut counter_state = CounterState::try_from_slice(&data_account.data.borrow())?;

    match instruction {
        Instruction::Init => {
            counter_state.count = 1;
        }
        Instruction::Double => counter_state.count *= 2,
        Instruction::Half => counter_state.count /= 2,
        Instruction::Add { amount } => counter_state.count += amount,
        Instruction::Subtract { amount } => counter_state.count -= amount,
    }
    counter_state.serialize(&mut *data_account.data.borrow_mut())?;
    Ok(())
}
