import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "./wba-wallet.json"
import bs58 from 'bs58';

const secretKeyString = wallet.secretKey; 
const secretKeyArrayBuffer = Buffer.from(bs58.decode(secretKeyString));

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(secretKeyArrayBuffer);
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
     let tx = createNft(umi,
        {
            mint,
            uri: "https://arweave.net/qKi4_F7yCstEczAHJgV_bnuGgbjRMnhMmBFrawiHgCM",
            //uri: "https://arweave.net/5vH9rhYDBayY2AioInQJGdRYfHM5cchQbJ3UIV0V3Hk",
            name : "RUGGG",
            symbol: "RUGGG",
            sellerFeeBasisPoints: percentAmount(1)

        }
        )
     let result = await tx.sendAndConfirm(umi);
     const signature = bs58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();