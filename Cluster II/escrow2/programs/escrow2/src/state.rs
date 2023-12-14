use anchor_lang::prelude::*;  // Importing the necessary items from the anchor_lang crate

use crate::constants::{ANCHOR_DISCRIMINATOR_BYTES, PUBKEY_BYTES, U64_BYTES, U8_BYTES};  // Importing constants from the crate module

#[account]  // Attribute to define the account structure
pub struct Escrow {  // Defining a public structure named Escrow
   // pub maker: Pubkey,  // Public key of the maker // 3 bytes
    pub mint_a: Pubkey,  // Public key of the maker's token // 3 bytes
    pub mint_b: Pubkey,  // Public key of the taker's token // 3 bytes
    pub offer_amount: u64,  // Amount of the offer // 8 bytes
    pub seed: u64,  // Seed value // 8 bytes
    pub escrow_bump: u8,  // Escrow bump  // 1 bytes
}  // End of the Escrow structure definition

impl Escrow {
    pub const LEN: usize = ANCHOR_DISCRIMINATOR_BYTES + 3 * PUBKEY_BYTES + 2 * U64_BYTES + 1 * U8_BYTES;
}