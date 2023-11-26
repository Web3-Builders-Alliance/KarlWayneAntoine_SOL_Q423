// Import necessary libraries
import { PublicKey, Keypair} from "@solana/web3.js"
import wallet from "./wba-wallet.json"
import bs58 from 'bs58';

// Import Metaplex libraries
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createMetadataAccountV3 } from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, publicKey, signerIdentity } from "@metaplex-foundation/umi";
import { base58 } from "@metaplex-foundation/umi/serializers";

// Get secret key from wallet
const secretKeyString = wallet.secretKey; 
const secretKeyArrayBuffer = Buffer.from(bs58.decode(secretKeyString));

// Create UMI instance
const umi = createUmi('https://api.devnet.solana.com'); //Initializes UMI (Universal Metaplex Interface) with the Solana Devnet RPC URL.
let keypair = umi.eddsa.createKeypairFromSecretKey(secretKeyArrayBuffer); //Restore/Create a keypair from the decoded secret key.
const myKeypairSigner = createSignerFromKeypair(umi,keypair); //Creates a signer from the keypair.

// Use signer identity
umi.use(signerIdentity(myKeypairSigner)); //Sets the signer identity in UMI to the created signer.

// Define Mint address
const mint = new PublicKey("CQhE1ys2vPmcJRGey74RMzycj8Myf4LvswWN1mJELsGZ")

// Add the Token Metadata Program
const token_metadata_program_id = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')

// Create PDA for token metadata
const metadata_seeds = [
    Buffer.from('metadata'),
    token_metadata_program_id.toBuffer(),
    mint.toBuffer(),
];

// Find program address
const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(
    metadata_seeds,
    token_metadata_program_id,
);

// Create metadata account
(async () => {  // Start an asynchronous function
    try {  // Start a try-catch block to handle potential errors
        let myTransaction = createMetadataAccountV3(  // Create a metadata account using the Metaplex function
            umi,  // Pass the UMI instance
            {
                //accounts
                metadata: publicKey(metadata_pda.toString()),  // The metadata account's public key
                mint: publicKey(mint.toString()),  // The mint's public key
                mintAuthority: myKeypairSigner,  // The signer who has the authority to mint tokens
                payer: myKeypairSigner,  // The signer who will pay for the transaction
                updateAuthority: keypair.publicKey,  // The public key of the account that has the authority to update the metadata
                data: {  // The metadata to be stored
                  name: "myname",  // The name of the token
                  symbol: "exp",  // The symbol of the token
                  uri: "example_uri.com",  // The URI where the metadata is stored
                  sellerFeeBasisPoints: 0,  // The fee to be paid to the seller, in basis points
                  creators: null,  // The creators of the token
                  collection: null,  // The collection to which the token belongs
                  uses: null  // The uses of the token
                },
                isMutable: true,  // Whether the metadata can be updated
                collectionDetails: null,  // The details of the collection to which the token belongs
            }
        )

        // Send and confirm transaction
        let result = await myTransaction.sendAndConfirm(umi);
        console.log(result.signature);

        // Deserialize signature
        const signature = base58.deserialize(result.signature);
        console.log(signature[0]);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
