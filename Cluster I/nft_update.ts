import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount, publicKey } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata, updateMetadataAccountV2 } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "./wba-wallet.json"
import bs58 from 'bs58';

import { PublicKey, Keypair} from "@solana/web3.js"

const secretKeyString = wallet.secretKey;  // Load secret key from wallet
const secretKeyArrayBuffer = Buffer.from(bs58.decode(secretKeyString));  // Decode secret key into array buffer

const RPC_ENDPOINT = "https://api.devnet.solana.com";  // Define RPC endpoint
const umi = createUmi(RPC_ENDPOINT);  // Create UMI instance with RPC endpoint

let keypair = umi.eddsa.createKeypairFromSecretKey(secretKeyArrayBuffer);  // Create keypair from secret key
const myKeypairSigner = createSignerFromKeypair(umi, keypair);  // Create signer from keypair
umi.use(signerIdentity(myKeypairSigner));  // Use signer identity
umi.use(mplTokenMetadata())  // Use MPL token metadata



// Define Mint address
const mint = new PublicKey("2gzu5EFZ7KG3AMWGH7rvWBgxJyHQHxY5N8gsq2gioRSx")  // Define mint address

// Add the Token Metadata Program
const token_metadata_program_id = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')  // Define token metadata program ID

// Create PDA for token metadata
const metadata_seeds = [  // Define metadata seeds
    Buffer.from('metadata'),  // Buffer for 'metadata'
    token_metadata_program_id.toBuffer(),  // Buffer for token metadata program ID
    mint.toBuffer(),  // Buffer for mint address
];

// Find PDA  address
const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(  // Find program address synchronously
    metadata_seeds,  // Metadata seeds
    token_metadata_program_id,  // Token metadata program ID
);



(async () => {  // Start an asynchronous function
     let updateMetada = updateMetadataAccountV2(umi,  // Create update metadata account V2
        {
            metadata: publicKey(metadata_pda.toString()),  // Define metadata public key
            updateAuthority : myKeypairSigner,  // Set update authority as myKeypairSigner
        data: {  // Define metadata data
            uri: "https://arweave.net/qKi4_F7yCstEczAHJgV_bnuGgbjRMnhMmBFrawiHgCM",  // Set URI
            name : "RUGGG v2",  // Set name
            symbol: "RUGGG v2",  // Set symbol
            sellerFeeBasisPoints: 100,  // Set seller fee basis points
            creators : [  // Define creators
                {
                    address: keypair.publicKey,  // Set creator address
                    share: 100,  // Set share
                    verified: true  // Set verified
                }
            ],
            collection : null,  // Set collection
            uses : null  // Set uses
        }
            

        }
        )
     let result = await updateMetada.sendAndConfirm(umi);  // Send and confirm update metadata
     const signature = bs58.encode(result.signature);  // Encode signature to base58
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)  // Log success message with transaction link

    console.log("Mint Address: ", mint);  // Log mint address
})();