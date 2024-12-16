use anchor_lang::prelude::*;

declare_id!("CcXDtgNex3qFycGqSMDzY1dAMrdqLQN5h1RwNkv3PSvF");

#[program]
pub mod sol_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
