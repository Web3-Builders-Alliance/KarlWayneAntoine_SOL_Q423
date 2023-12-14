use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, transfer, CloseAccount, Mint, Token, TokenAccount, Transfer},
};

use crate::{error::EscrowError, state::Escrow};

#[derive(Accounts)] //his is an Anchor macro that prepares the struct to hold account information for a Solana program instruction.
#[instruction(seed: u64)] //Indicates that the Take instruction takes an additional parameter seed of type u64. This seed is used in the account initialization process.
pub struct Take<'info> { //Defines a Rust struct named Take, which is generic over a lifetime 'info. This lifetime is used to tie the accounts to the duration of the instruction call.
    #[account(mut)] //Specifies that these accounts are mutable (can be changed during the instruction execution)
    pub taker: Signer<'info>, //taker is the account of the user taking the escrow
    
    #[account(mut)] //Specifies that these accounts are mutable (can be changed during the instruction execution)
    pub maker: SystemAccount<'info>, //maker is the account of the user who created the escrow
    
    pub mint_a : Account<'info, Mint>, //Declares an account mint_a, representing a token mint (type of token) involved in the escrow.
    pub mint_b: Account<'info, Mint>, //Declares an account mint_b, representing a token mint (type of token) involved in the escrow.
   
    #[account( // These annotations set up constraints and requirements for the token accounts of the taker. They specify the associated token mint and authority.
        // mut,
        init_if_needed, //This indicates that the account should be initialized if it does not already exist.
        payer = taker, ////Specifies that the taker account will pay for any account creation or rent fees.
        associated_token::mint = mint_a , //Links this account to the mint_a token mint.
        associated_token::authority = taker, //Sets the taker as the authority of this token account.
    )]
    pub taker_ata_a: Account<'info, TokenAccount>, //his account is for the taker to receive tokens of type mint_a. It's where the taker will receive tokens from the escrow (the maker's offering).
   
    #[account( //These annotations set up constraints and requirements for the token accounts of the taker. They specify the associated token mint and authority.
        mut, //Indicates that this account is mutable and may be modified.
        associated_token::mint = mint_b, //Links this account to the mint_b token mint.
        associated_token::authority = taker, //Sets the taker as the authority of this token account.
    )]
    pub taker_ata_b: Account<'info, TokenAccount>, //This account is for the taker to deposit their own tokens of type mint_b into the escrow. It's part of the taker's contribution to the escrow transaction.
    
    #[account( //Similar to taker_ata_b, but for the maker's token account for mint_b.
        init_if_needed, //This indicates that the account should be initialized if it does not already exist.
        payer = taker, //Specifies that the taker account will pay for any account creation or rent fees.
        associated_token::mint = mint_b, //Links this account to the mint_b token mint.
        associated_token::authority = maker, //Sets the maker  as the authority of this token account.
    )]
    pub maker_ata_b: Account<'info, TokenAccount>, //This account is for the maker to receive tokens of type mint_b. It's where the maker will receive tokens from the escrow (the taker's offering).  
    
    #[account( //This annotation initializes the escrow account with specific parameters like seeds for generating its address and constraints like has_one to ensure it's associated with the correct mints.
        mut, //The account is mutable.
        close = maker, //Allows the maker to close this account.

        //maker.key().as_ref()
        //maker.key() This gets the public key of the maker account.
        //.as_ref()  Converts the public key into a reference to a byte slice
        //Converts the public key into a reference to a byte slice (&[u8]), which is the required format for a seed. This dynamic seed ensures that the PDA is unique to the maker

        //escrow.seed.to_le_bytes().as_ref()
        //escrow .seed  Refers to a seed value stored in the escrow account's state.
        //to_le_bytes()  Converts the seed (likely a numeric type) into an array of bytes in little-endian format.
        //Converts the byte array into a byte slice. This adds another layer of uniqueness to the PDA, tying it to a specific escrow transaction.
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()], //Used for deriving the account's address using the PDA (Program Derived Address) mechanism.
        
        //bump: This is a value used in conjunction with the seeds to derive the PDA. It's a single byte that Solana runtime adjusts to ensure the generated address is off-curve (i.e., has no corresponding private key).
        //escrow.escrow_bump, This refers to the bump value stored in the escrow account's state. When the PDA was initially created, the program would have found a bump value that, combined with the specified seeds, results in a valid PDA. This value is then stored in the escrow account for future reference.
        bump = escrow.escrow_bump,
        has_one = mint_a, //Ensures that the escrow account is associated with the specified token mints.
        has_one = mint_b, //Ensures that the escrow account is associated with the specified token mints.
    )]
    pub escrow: Account<'info, Escrow>,
    
    #[account(
        mut, //Indicates mutability.
        associated_token::mint = mint_a, //Links the vault to the mint_a token mint.
        associated_token::authority = escrow, //Sets the escrow account as the authority of the vault.
    )] // Like a PDA The vault account is an ATA with the escrow program account set as its authority, allowing the program to control it. This setup is often used in scenarios where a program needs to manage tokens on behalf of users, as is the case in escrow transactions.
    pub vault: Account<'info, TokenAccount>, //Declares the vault account, which holds the tokens deposited by the maker during the escrow.
    
    pub system_program: Program<'info, System>, //Reference to the Solana System Program, used for system-level operations.
    pub token_program: Program<'info, Token>, //Reference to the SPL Token Program, used for token-related operations.
    pub associated_token_program: Program<'info, AssociatedToken>, //Reference to the Associated Token Program, used for operations related to associated token accounts.
}

impl<'info> Take<'info> {
    pub fn deposit(&mut self) -> Result<()> {//Defines a public function deposit that mutates the state of Take and returns a Result type for error handling.
        let transfer_accounts = Transfer {//Creates a Transfer struct to specify the accounts involved in the token transfer.
            from: self.taker_ata_b.to_account_info(),//The source account for the transfer is the taker's token account for mint_b.
            to: self.maker_ata_b.to_account_info(), //The destination account is the maker's token account for mint_b.
            authority: self.taker.to_account_info(), //The authority to execute this transfer is the taker's account.
        };
        // Creates a Cross-Program Invocation (CPI) context for the token transfer.
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer(cpi_ctx, self.escrow.offer_amount) //Calls the SPL transfer function to transfer tokens from the taker to the maker.
            .map_err(|_| return error!(EscrowError::DepositFailed))?; //Handles any errors that might occur during the transfer.
        Ok(())
    }
    //Similar to deposit, but this time transferring from the vault to the taker_ata_a.
    pub fn withdraw(&mut self) -> Result<()> {
        let transfer_accounts = Transfer {
            from: self.vault.to_account_info(), //Sets the from field to the vault account's information. This is the source account from which tokens will be withdrawn. to_account_info() converts the vault account into a format that can be used in a Cross-Program Invocation (CPI).
            to: self.taker_ata_a.to_account_info(), //Sets the to field to the taker_ata_a account's information. This is the destination account where the tokens will be deposited. Again, to_account_info() is used for CPI compatibility.
            authority: self.escrow.to_account_info(), //Sets the authority field to the escrow account's information. This account is the authority over the vault account and is required to authorize the transfer. The escrow account is likely a Program Derived Account (PDA), and its authority is used to validate the transaction.
        };
        //Purpose: signer_seeds are used to generate a Program Derived Address (PDA) for signing transactions that require access to an account controlled by the program (like the vault).
        let signer_seeds: [&[&[u8]]; 1] = [&[  //Sets up the seeds for signing the transaction, necessary for operations involving a PDA (like the vault).
            b"escrow", // A static seed, usually a string literal that identifies the purpose of the PDA.
            self.maker.to_account_info().key.as_ref(), //The public key of the maker account, ensuring the PDA is unique to this escrow transaction.
            &self.escrow.seed.to_le_bytes()[..], //A unique seed stored in the escrow account, converted to a byte array.
            &[self.escrow.escrow_bump], // The bump seed used in conjunction with the other seeds to generate the PDA.
        ]];
        //Creates a CPI context with the signer seeds, used for executing the transfer from a PDA.
        //Purpose: Creates a Cross-Program Invocation (CPI) context for calling another program (like the Token program) with the necessary accounts and signer seeds.
        let cpi_ctx = CpiContext::new_with_signer(// is used to create a context for a CPI when the involved account is a PDA. This is necessary because PDAs don't have private keys to sign transactions.
            self.token_program.to_account_info(), //The account info of the Token program, which will process the token transfer.
            transfer_accounts, //A struct specifying the accounts involved in the transfer.
            &signer_seeds, //The seeds array used to sign the transaction on behalf of the PDA.
        );
        //Transfers the entire amount from the vault to the taker's account.
        transfer(cpi_ctx, self.vault.amount)
            .map_err(|_| return error!(EscrowError::WithdrawFailed))?;
        Ok(())
    }
//Prepares to close the vault account.
    pub fn close_vault(&mut self) -> Result<()> { //Closes the vault account, transferring any remaining lamports (Solana's smallest unit of currency) to the taker.
        let close_accounts = CloseAccount { //Initializes a variable close_accounts with a CloseAccount struct. This struct is used to specify the accounts involved in closing a token account.
            account: self.vault.to_account_info(), //Sets the account field to the vault account's information. This is the account that will be closed. to_account_info() is a method that converts the vault account into a format that can be used in a Cross-Program Invocation (CPI).
            destination: self.taker.to_account_info(), //Sets the destination field to the taker account's information. When the vault account is closed, any remaining lamports (the smallest unit of currency in Solana) will be transferred to this destination account.
            authority: self.escrow.to_account_info(), //Sets the authority field to the escrow account's information. The escrow account, likely a Program Derived Account (PDA), is the authority over the vault account and is required to authorize its closure.
        };
//Similar to withdraw, sets up the seeds and CPI context for closing the account.
           //Purpose: signer_seeds are used to generate a Program Derived Address (PDA) for signing transactions that require access to an account controlled by the program (like the vault).
           let signer_seeds: [&[&[u8]]; 1] = [&[  //Sets up the seeds for signing the transaction, necessary for operations involving a PDA (like the vault).
           b"escrow", // A static seed, usually a string literal that identifies the purpose of the PDA.
           self.maker.to_account_info().key.as_ref(), //The public key of the maker account, ensuring the PDA is unique to this escrow transaction.
           &self.escrow.seed.to_le_bytes()[..], //A unique seed stored in the escrow account, converted to a byte array.
           &[self.escrow.escrow_bump], // The bump seed used in conjunction with the other seeds to generate the PDA.
       ]];
//Closing the Vault:
        let cpi_ctx_close = CpiContext::new_with_signer( //Similar to cpi_ctx in withdraw, but specifically set up for closing the vault account.
            self.token_program.to_account_info(),
            close_accounts,
            &signer_seeds,
        );

        close_account(cpi_ctx_close).map_err(|_| return error!(EscrowError::CloseVaultFailed))?;
        Ok(())
    }
}

//signer_seeds are crucial for operations involving PDAs, allowing the program to sign transactions on behalf of these accounts.
//cpi_ctx and cpi_ctx_close create contexts for Cross-Program Invocations, enabling the program to interact with other programs (like the Token program) for transfers and account closures.
//withdraw and close_vault are key functions for managing the flow of tokens and closing accounts as part of the escrow process, with robust error handling.