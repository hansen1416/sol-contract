use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{ Token, TokenAccount, Transfer };

declare_id!("CcXDtgNex3qFycGqSMDzY1dAMrdqLQN5h1RwNkv3PSvF");

#[program]
pub mod my_program {
    use super::*;
    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    // pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> ProgramResult {
    //     let cpi_accounts = Transfer {
    //         from: ctx.accounts.from_token_account.to_account_info(),
    //         to: ctx.accounts.to_token_account.to_account_info(),
    //         authority: ctx.accounts.user.to_account_info(),
    //     };
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
    //     Ok(())
    // }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> ProgramResult {
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
            from: ctx.accounts.source_token_account.to_account_info(),
            to: ctx.accounts.destination_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        });
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct TransferTokens<'info> {
//     #[account(mut)]
//     pub from_token_account: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub to_token_account: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub token_program: Program<'info, Token>,
// }

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub source_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
