import wallet from "./wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr"

import bs58 from 'bs58';

const secretKeyString = wallet.secretKey; 
const secretKeyArrayBuffer = Buffer.from(bs58.decode(secretKeyString));

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(secretKeyArrayBuffer);
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

         const image = "https://arweave.net/QyJNrQakNqHO964sZCJrahW-MDkhk1AFPAjsg70pnog"
         const metadata = {
             name: "RUUUUGGGG",
             symbol: "RUUUUGGGG",
             description: "THE BIG RUG",
             image,
             attributes: [
                 {
                trait_type: "Background",
                value: 'pink'
                },
                {
                trait_type: "Rarity",
                value: 'GOAT'
                    },
                {
                trait_type: "Color",
                value: 'Supa Pink'
                            }
             ],
             properties: {
                 files: [
                     {
                         type: "image/png",
                         uri: image
                     },
                 ]
             },
             creators: []
         };
         const myUri = await bundlrUploader.uploadJson(metadata)
         console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();