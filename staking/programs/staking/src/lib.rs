use anchor_lang::prelude::*;

declare_id!("Gsm81nM88odQUjdnKSDAbvBAEgPh9hZ3dqbeWciNQmQz");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
