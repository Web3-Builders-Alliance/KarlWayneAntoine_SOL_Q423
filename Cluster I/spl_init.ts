import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "./wba-wallet.json"

import bs58 from 'bs58';

const secretKeyString = wallet.secretKey; 
const secretKeyArrayBuffer = Buffer.from(bs58.decode(secretKeyString));

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(secretKeyArrayBuffer);

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
    try {
        
        // Start here
        // Create a new mint
        const mint = await createMint(
            connection,
            keypair,
            keypair.publicKey,
            null,
            6
        );
        
        console.log(`Mint created with public key: ${mint.toBase58()}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
