use anchor_lang::prelude::*;

declare_id!("FYfT9oki9DgNa7WYiqT1UdMeWWSdP8eiqLW9RikhvYsW");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
