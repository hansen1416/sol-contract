// use anchor_lang::prelude::*;
// use anchor_lang::solana_program::entrypoint::ProgramResult;
// use anchor_spl::token::{ Token, TokenAccount, Transfer };

// declare_id!("CcXDtgNex3qFycGqSMDzY1dAMrdqLQN5h1RwNkv3PSvF");
use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount, Transfer };
use anchor_spl::token::instruction::AuthorityType;

declare_id!("CcXDtgNex3qFycGqSMDzY1dAMrdqLQN5h1RwNkv3PSvF"); // Replace with your program's ID

#[program]
pub mod token_transfer_relay {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.relay_state.bump = *ctx.bumps.get("relay_state").unwrap();
        Ok(())
    }

    pub fn relay_transfer<'info>(
        ctx: Context<'_, '_, 'info, 'info, RelayTransfer<'info>>,
        amount: u64
    ) -> Result<()> {
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        // Check if the authority is the expected PDA
        let (expected_authority, bump) = Pubkey::find_program_address(
            &[ctx.accounts.relay_state.to_account_info().key.as_ref()],
            ctx.program_id
        );

        if expected_authority != *ctx.accounts.authority.key {
            return Err(ErrorCode::InvalidAuthority.into());
        }

        anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        Ok(())
    }

    pub fn set_authority<'info>(
        ctx: Context<'_, '_, 'info, 'info, SetAuthority<'info>>
    ) -> Result<()> {
        let cpi_program = ctx.accounts.token_program.to_account_info();

        // Set the program as the delegate authority
        anchor_spl::token::instruction::set_authority(
            cpi_program.key,
            ctx.accounts.token_account.to_account_info().key,
            Some(ctx.accounts.relay_state.key), // New authority (our PDA)
            AuthorityType::AccountOwner,
            ctx.accounts.current_authority.key,
            &[&ctx.accounts.current_authority.key]
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1, seeds = [b"relay_state"], bump)]
    pub relay_state: Account<'info, RelayState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RelayTransfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    /// CHECK: Authority checked in the instruction
    pub authority: AccountInfo<'info>, // This will be our PDA
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"relay_state"], bump = relay_state.bump)]
    pub relay_state: Account<'info, RelayState>,
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub current_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"relay_state"], bump = relay_state.bump)]
    pub relay_state: Account<'info, RelayState>,
}

#[account]
pub struct RelayState {
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid authority provided")]
    InvalidAuthority,
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
