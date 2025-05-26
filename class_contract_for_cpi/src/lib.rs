use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::{self, ProgramResult, entrypoint},
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction::create_account,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //We create new PDA on chain
    //pda , useraccount , system program account
    let iter = &mut accounts.iter();
    let pda = next_account_info(iter)?;
    let user_acc = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    //create instruction for creating PDA
    //for pda, we use `invoke` function so behalf of pda account program will signed the transaction

    let seeds = &[user_acc.key.as_ref(), b"user"];

    let (pda_pub_key, bump) = Pubkey::find_program_address(seeds, program_id);
    //we can also use `Public::create_program_address` so it will create pda , where we can pass `bump` explicitely to create pda like

    let ix = create_account(user_acc.key, pda.key, 1000000000, 8, program_id);
    // invoke_signed(&ix, accounts, &[seeds, &[&[bump]]]);
    invoke_signed(&ix, accounts, &[&[user_acc.key.as_ref(), b"user", &[bump]]])?;

    Ok(())
}
