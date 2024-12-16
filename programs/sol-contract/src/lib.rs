use anchor_lang::prelude::*;

declare_id!("YourProgramIDHere"); // Replace with your program ID

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        greeting_account.message = "Hello, World!".to_string();
        Ok(())
    }

    pub fn get_greeting(ctx: Context<GetGreeting>) -> Result<String> {
        let greeting_account = &ctx.accounts.greeting_account;
        Ok(greeting_account.message.clone())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub greeting_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetGreeting<'info> {
    pub greeting_account: Account<'info, GreetingAccount>,
}

#[account]
pub struct GreetingAccount {
    pub message: String,
}
