import wallet from "../../../q3-rust/dev-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://arweave.net/b1OsLQmxNdOYbfGVGrFlIu2WGLyMFDFr7FEB8cXrnMY"
        const metadata = {
            name: "Ved's Rug",
            symbol: "veb",
            description: "A cool rug by ved",
            image,
            attributes: [
                {trait_type: 'coolness', value: '1000'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: null
        };
        const myUri = await umi.uploader.uploadJson(metadata)
        .catch(err => {throw new Error(err)})
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
