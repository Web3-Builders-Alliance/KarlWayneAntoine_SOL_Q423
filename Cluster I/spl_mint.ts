import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "./wba-wallet.json"

import bs58 from 'bs58';

const secretKeyString = wallet.secretKey; 
const secretKeyArrayBuffer = Buffer.from(bs58.decode(secretKeyString));

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(secretKeyArrayBuffer);

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("A9Jwz92Vfbe9ejNJsEL4REhMkSFV6J3pSHLy8YnhYZjP");

(async () => {
    try {
        // Get or create associated token account
        const ata = await getOrCreateAssociatedTokenAccount(
            connection, //
            keypair, //The account that will pay for the transaction fees if a new token account needs to be created.
            mint, //The public key of the token's mint (the specific token type).
            keypair.publicKey //The public key of the account for which the associated token account is intended.
        );
        console.log(`Your ata is: ${ata.address.toBase58()}`);

        // Mint tokens to ATA
        const mintTx = await mintTo(
            connection,
            keypair, // The account paying for the transaction
            mint, // The public key of the mint
            ata.address, // The account receiving the newly minted tokens
            keypair.publicKey, // The mint authority
            token_decimals // The amount to mint, calculated with decimals
        );
        console.log(`Your mint txid: ${mintTx}`);

    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
