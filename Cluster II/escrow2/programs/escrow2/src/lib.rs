use anchor_lang::prelude::*;

pub mod contexts;
use contexts::*;

pub mod state;
pub mod error;
pub mod constants;



declare_id!("76bwsGyECyqNWDaQ85Jn9TLqiSEW7D8JpgzzKJft7t3N");



#[program]
pub mod escrow2 {
    use super::*;

    //This function initializes the escrow transaction.
    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?; // Ensures the maker deposits the specified amount (deposit) into the vault. If the deposit fails, the transaction will not proceed.
        ctx.accounts.save_escrow(seed, receive, ctx.bumps.escrow)?; //Saves the escrow details, including the seed for PDA generation, the amount to be received by the taker, and the bump seed for the escrow account.
        Ok(())
    }

    //Facilitates the completion of the escrow transaction.
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?; //Checks if the taker has deposited their required tokens (of type mint_b). If this fails, the transaction does not proceed.
        ctx.accounts.withdraw()?; //Withdraws the tokens from the vault to the taker's account (taker_ata_a). This step is contingent on the successful deposit by the taker.
        ctx.accounts.close_vault()?;//Closes the vault account, transferring any remaining lamports to the taker. This is the final step in the escrow process.
        Ok(())
    }

    //andles the refund process in case the escrow conditions are not met.
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?; //Initiates the refund of tokens from the vault back to the maker. This occurs if the taker does not fulfill their part of the transaction.
        ctx.accounts.close_vault()?;//Closes the vault account after the refund, similar to the take function.
        Ok(())
    }

}