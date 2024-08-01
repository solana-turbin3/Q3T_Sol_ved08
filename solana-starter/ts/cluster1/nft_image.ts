import wallet from "../../../q3-rust/dev-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        const image = await readFile("/Users/ved08/workspace/solana-starter/ts/cluster1/generug2.png")
        //2. Convert image to generic file.
        const genericFile = await createGenericFile(image, "rug.png", {
            tags: [{
                name: 'Content-Type', value: 'image/png'
            }]
        })
        //3. Upload image
        const imageUri = await umi.uploader.upload([genericFile])
        .catch(err => {throw new Error(err)})


        const [myUri] = imageUri
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
