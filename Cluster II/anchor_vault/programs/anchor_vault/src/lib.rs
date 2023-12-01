// Importing the anchor_lang prelude module which includes all the necessary dependencies for the anchor framework
use anchor_lang::prelude::*;

// Declaring the program ID. This is a unique identifier for the program
declare_id!("B6yKmDcGT47RM9i6hYHZ6gL6jsysQKv1qxLc3ZsCA3BW");

// Defining the program module
#[program]
pub mod anchor_vault {
    // Importing the Transfer struct and transfer function from the system_program module of anchor_lang
    use anchor_lang::system_program::{Transfer,transfer};

    // Importing the parent module
    use super::*;

    // Defining the deposit function which takes a Context of Vault and lamports as arguments and returns a Result
    pub fn deposit(ctx: Context<Vault>, lamports : u64) -> Result<()> {
        // Creating a Transfer struct with from and to accounts
        let accounts = Transfer {
            from : ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            
        };
        
        // Creating a CpiContext with the system_program account and the Transfer accounts
        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), accounts);
        // Calling the transfer function with the CpiContext and lamports
        transfer(cpi_ctx,lamports)
       
    }

    // Defining the withdraw function which takes a Context of Vault and lamports as arguments and returns a Result
    pub fn withdraw(ctx: Context<Vault>, lamports : u64) -> Result<()> {
        // Creating a Transfer struct with from and to accounts
        let accounts =Transfer {
            from : ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.signer.to_account_info(),

    };

    // Creating a binding with the bump of the vault account
    let binding = [ctx.bumps.vault];
    // Creating the signer seeds with the vault string, the signer key and the binding
    let signer_seeds = [&[
        b"vault",
        ctx.accounts.signer.clone().key.as_ref(),
        &binding,
    ][..]];

    // Creating a CpiContext with the system_program account, the Transfer accounts and the signer seeds
    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info()
        , accounts, &signer_seeds);
        // Calling the transfer function with the CpiContext and lamports
        transfer(cpi_ctx, lamports)
    }

}

// Defining the Vault struct with the Accounts derive attribute
#[derive(Accounts)]
pub struct Vault<'info> {
    // Defining the signer account with the mut attribute
    #[account(mut)]
    signer:Signer<'info>,

    // Defining the vault account with the mut attribute and seeds for deterministic address generation
    #[account(
        mut,
        seeds = [b"vault",signer.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    // Defining the system_program account
    system_program:Program<'info, System>,
}


// Command to start the Solana test validator
//solana-test-validator
// Command to deploy the anchor program
//anchor deploy
// Command to run the anchor tests skipping the local validator
//anchor test --skip-local-validator
// Command to add the Solana web3.js library for testing
//yarn add @solana/web3.js for test
