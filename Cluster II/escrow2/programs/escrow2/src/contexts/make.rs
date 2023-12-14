use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};

use crate::state::Escrow;



#[derive(Accounts)] // Macro that will prepare the struct to hold account information for the solana instruction
#[instruction(seed: u64)] //This is a custom instruction parameter. It's used as part of the seed for generating a unique address for the escrow account.
//This line indicates that the Make instruction takes an additional parameter seed of type u64. This seed is used in the account initialization process.
pub struct Make<'info> { //Defines a Rust struct named Make, which is generic over a lifetime 'info. This lifetime is used to tie the accounts to the duration of the instruction call.
    #[account(mut)] //This annotation specifies that the maker account is mutable, meaning it can be modified during the instruction execution.
   pub maker: Signer<'info>, //Declares an account maker, which must sign the transaction. The Signer type ensures that the account has signed the transaction.
   
   pub mint_a : Account<'info, Mint>, //This line declares an account mint_a, which represents a token mint. The Mint type is part of the SPL Token program.
   pub mint_b : Account<'info, Mint>, //This line declares an account mint_b, which represents a token mint. The Mint type is part of the SPL Token program.
   
    #[account( //This complex annotation sets up constraints and requirements for the maker_ata_a account, which is an associated token account for mint_a and maker. The constraints ensure that this account is associated with the correct mint and authority.
        mut,
        associated_token::mint = mint_a, 
        associated_token::authority = maker
    )]
   pub maker_ata_a : Account<'info, TokenAccount>, //Declares the token account of the maker for the token maker_mint_token_a
     
     #[account( //This annotation initializes the escrow account with specific parameters like space allocation and seeds for generating its address.
        init,
        payer = maker,
        space = Escrow::LEN,
        seeds = [b"escrow",maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
     )]
   pub escrow : Account<'info, Escrow>, //Declares the escrow account, which holds the state of the escrow transaction.
    
    #[account( //Similar to escrow, this initializes the vault account, which will hold the tokens deposited by the maker.
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow, // gives the autority to the escrow account to the vault account ***********
    )]
    pub vault : Account<'info, TokenAccount>, //Declares the vault account for holding tokens during the escrow.
    
    pub associated_token_program : Program<'info, AssociatedToken>, //This is a reference to the Solana System Program, used for creating accounts and other system-level operations.
    pub token_program : Program<'info, Token>, //A reference to the SPL Token Program, used for token-related operations.
    pub system_program: Program<'info, System> //A reference to the Associated Token Program, used for operations related to associated token accounts.
}

impl<'info> Make<'info> { //This line begins the implementation block for methods on the Make struct. The 'info lifetime parameter ensures that the data referenced by the struct lives as long as the struct itself.
    //Sets up the escrow account with necessary details like seed, token mints, and amounts.
    //bumps : &MakeBumps

    //Indicates that the function will modify the Make struct.
        //The function takes three parameters - seed, offer_amount, and bump, all of which are used to set up the escrow account.
        // The function returns a Result type, which is a common Rust pattern for error handling. () signifies that it returns no value upon success.
    pub fn save_escrow(&mut self,seed: u64,offer_amount: u64, bump : u8 ) -> Result<()> { 
        //self.escrow: Refers to the escrow account within the Make struct.
        //set_inner: A method provided by Anchor to set the data of an account. It's used here to initialize the Escrow struct with specific values.
        //Escrow{ ... }: Creates a new instance of the Escrow struct with the provided values.
        self.escrow.set_inner(Escrow{
            mint_a: self.mint_a.key(), //Sets the maker_mint_token_a field to the public key of maker_mint_token_a, identifying the token type the maker is offering.
            mint_b: self.mint_b.key(), //Sets the taker_mint_token_b field to the public key of taker_mint_token_b, identifying the token type the taker will provide.
            offer_amount: offer_amount, //Sets the offer_amount field to the amount of maker_mint_token_a being offered in the escrow.
            seed, //Sets the seed field, used in generating the address of the escrow account.
            //escrow_bump: bumps.escrow
            escrow_bump : bump //Sets the escrow_bump field, which is part of the seed used for the escrow account's address generation.

        });
        Ok(())

    }
    //Handles the transfer of tokens from the maker's account to the vault account.
    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.maker_ata_a.to_account_info(), //The source account from which tokens will be debited.
            to:self.vault.to_account_info(), //The destination account to which tokens will be credited.
            authority: self.maker.to_account_info() //The account that has the authority to approve the transfer. In this case, it's the maker's account.
        };

        //This function creates a new Cross-Program Invocation (CPI) context. CPI is used when a program (like our escrow program) wants to call another program (like the Token program).
        //The CPI context specifies the program being called (the Token program in this case) and the accounts involved in the call (encapsulated in the Transfer struct).
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts );

        transfer(cpi_ctx,deposit)
    
    }
}
