import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "./wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import bs58 from 'bs58';



// We're going to import our keypair from the wallet file
const secretKeyString = wallet.secretKey;  // Load secret key from wallet
const secretKeyArrayBuffer = Buffer.from(bs58.decode(secretKeyString));  // Decode secret key into array buffer
const keypair = Keypair.fromSecretKey(secretKeyArrayBuffer);


//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("CQhE1ys2vPmcJRGey74RMzycj8Myf4LvswWN1mJELsGZ");

// Recipient address
const to = new PublicKey("76SxBSN8nCfvdR4W1TFD8pBLhxyEczggqLA4SFCT9MEc");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromATA = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
            );

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toATA = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
            );

        // Transfer the new token to the "toTokenAccount" we just created
        await transfer(
            connection,
            keypair,
            fromATA.address,
            toATA.address,
            keypair.publicKey,
            1_000_000
            );
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
