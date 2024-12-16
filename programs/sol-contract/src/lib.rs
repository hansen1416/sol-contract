use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{ Token, TokenAccount, Transfer };

declare_id!("CcXDtgNex3qFycGqSMDzY1dAMrdqLQN5h1RwNkv3PSvF");

#[program]
pub mod my_token_forwarder {
    use super::*;

    pub fn forward_tokens(ctx: Context<ForwardTokens>, amount: u64) -> ProgramResult {
        let token_program = &ctx.accounts.token_program;
        let source_account = &ctx.accounts.source_account;
        let destination_account = &ctx.accounts.destination_account;
        let authority = &ctx.accounts.authority;

        let cpi_ctx = CpiContext::new(token_program.to_account_info(), Transfer {
            from: source_account.to_account_info(),
            to: destination_account.to_account_info(),
            authority: authority.to_account_info(),
        });
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ForwardTokens<'info> {
    #[account(mut)]
    pub source_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

// use anchor_lang::prelude::*;
// use anchor_spl::token::{ self, Mint, Token, TokenAccount, Transfer };

// declare_id!("CcXDtgNex3qFycGqSMDzY1dAMrdqLQN5h1RwNkv3PSvF");

// #[program]
// pub mod token_relay {
//     use super::*;

//     /// Transfer tokens upon detection in the vault account
//     pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
//         msg!("Initiating token transfer...");

//         // Perform the token transfer
//         let cpi_accounts = Transfer {
//             from: ctx.accounts.vault_account.to_account_info(),
//             to: ctx.accounts.recipient_account.to_account_info(),
//             authority: ctx.accounts.vault_authority.to_account_info(),
//         };
//         let cpi_program = ctx.accounts.token_program.to_account_info();

//         let seeds = &[
//             b"vault".as_ref(),
//             ctx.accounts.mint.key().as_ref(),
//             ctx.accounts.vault_authority.key().as_ref(),
//             &[ctx.accounts.bump],
//         ];
//         let signer_seeds = &[&seeds[..]];

//         token::transfer(
//             CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds),
//             amount
//         )?;

//         msg!("Token transfer successful");
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct TransferTokens<'info> {
//     /// The vault account where tokens are stored
//     #[account(
//         mut,
//         constraint = vault_account.owner == token_program.key()
//     )]
//     pub vault_account: Account<'info, TokenAccount>,

//     /// The recipient account to receive tokens
//     #[account(
//         mut,
//         constraint = recipient_account.owner == token_program.key()
//     )]
//     pub recipient_account: Account<'info, TokenAccount>,

//     /// The authority of the vault
//     pub vault_authority: Signer<'info>,

//     /// The token mint of the SPL token being transferred
//     pub mint: Account<'info, Mint>,

//     /// The Token Program (SPL Token)
//     pub token_program: Program<'info, Token>,

//     /// Bump seed for PDA
//     pub system_program: Program<'info, System>,

//     #[account(seeds = [b"vault", mint.key().as_ref(), vault_authority.key().as_ref()], bump)]
//     pub vault_pda: AccountInfo<'info>,
// }

// // 2. Explanation of the Code
// // Vault PDA:

// // A Program-Derived Address (PDA) is used to control the token vault, ensuring only the program can authorize actions like transfers.
// // Transfer Tokens:

// // When tokens are received in the vault, they are automatically transferred to the recipient's account using the SPL Token Program's transfer instruction.
// // Dynamic Token Support:

// // The program dynamically accepts the mint and associated token accounts, allowing it to support any SPL token.
// // Secure Authorization:

// // The PDA is used as the authority to ensure only the program has control over the vault.
