use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::{self, ProgramResult, entrypoint},
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)]
struct OnChainData {
    count: u8,
}
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let data_account = next_account_info(&mut iter)?; //&[u8]

    //deserialize data for doubling the number
    let mut counter = OnChainData::try_from_slice(&data_account.data.borrow_mut())?;

    if counter.count == 0 {
        counter.count = 1;
    } else {
        counter.count *= 2;
    }
    //*(start), it is dereference the RefMut to get &mut [u8] or the data with actual data */
    counter.serialize(&mut *data_account.data.borrow_mut())?;
    Ok(())
}
