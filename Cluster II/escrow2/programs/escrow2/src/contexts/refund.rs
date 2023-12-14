use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, transfer, CloseAccount, Mint, Token, TokenAccount, Transfer},
};

use crate::{error::EscrowError, state::Escrow};

#[derive(Accounts)] // This is an Anchor macro that prepares the struct to hold account information for a Solana program instruction.
pub struct Refund<'info> { //Defines a Rust struct named Refund, which is generic over a lifetime 'info. This lifetime is used to tie the accounts to the duration of the instruction call.
    #[account(mut)] //Specifies that the maker account is mutable (can be changed during the instruction execution).
    pub maker: Signer<'info>, //is the account of the user who initiated the escrow and is a signer of the transaction.
   
    pub mint_a: Account<'info, Mint>, //Declares an account mint_a, representing a token mint (type of token) involved in the escrow.
   
    #[account(
        init_if_needed, //Initializes the account if it doesn't exist.
        payer = maker, //The maker pays for the account creation or rent.
        associated_token::mint = mint_a,//Links to the mint_a token mint.
        associated_token::authority = maker,//The maker is the authority of this account.
    )]
    pub maker_ata_a: Account<'info, TokenAccount>, //this account is for the maker to receive tokens of type mint_a as part of the refund process.
   
    #[account(
        mut,//The account is mutable.
        close = maker,//Allows the maker to close this account.
        has_one = mint_a,//Ensures the escrow account is associated with the mint_a token mint.
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],//Used for deriving the account's address using the PDA mechanism.
        bump = escrow.escrow_bump //Used for deriving the account's address using the PDA mechanism.
    )]
    pub escrow: Account<'info, Escrow>, //Holds the state of the escrow transaction.
    
    #[account(
        mut,//Indicates mutability.
        associated_token::mint = mint_a,//Links the vault to the mint_a token mint.
        associated_token::authority = escrow,//Sets the escrow account as the authority of the vault.
    )]
    pub vault: Account<'info, TokenAccount>, //The vault account is where the maker's tokens are held during the escrow.
    //These are references to the Solana System Program, SPL Token Program, and Associated Token Program, used for various operations like creating accounts and handling tokens.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Refund<'info> {
    pub fn refund(&mut self) -> Result<()> { //Declares a public function named refund that can modify the state of the Refund struct and returns a Result type for error handling.
        let transfer_accounts = Transfer {//Initializes a Transfer struct to specify the accounts involved in the token transfer.
            from: self.vault.to_account_info(),//Specifies the vault account as the source of the transfer.
            to: self.maker_ata_a.to_account_info(),//Specifies the maker's ATA for mint_a as the destination.
            authority: self.escrow.to_account_info(),//Sets the escrow account as the authority for the transfer.
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[ // Sets up the seeds for the PDA (escrow account) that will authorize the transfer.
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.escrow_bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(//Creates a CPI context with the necessary accounts and signer seeds for the token transfer.
            self.token_program.to_account_info(),
            transfer_accounts,
            &signer_seeds,
        );

        transfer(cpi_ctx, self.vault.amount) //Calls the SPL transfer function to transfer the specified amount from the vault to the maker's ATA.
            .map_err(|_| return error!(EscrowError::RefundFailed))?;
        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.escrow_bump],
        ]]; 

        let cpi_ctx_close = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            &signer_seeds,
        );

        close_account(cpi_ctx_close).map_err(|_| return EscrowError::CloseVaultFailed)?;
        Ok(())
    }
}