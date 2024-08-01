import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "../../../q3-rust/dev-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("HJKvigwCB1DBqFxNkJ7bBGrTdmABwF3t6hsXLnxi3Ac9");

(async () => {
    try {
// ########## Code to create multiple Token Accounts of a single owner ###################
        // for(let i = 0; i<10; i++) {
        //     const tokenAccount = await createAccount(
        //         connection,
        //         keypair,
        //         mint,
        //         keypair.publicKey,
        //         Keypair.generate()
        //     )
        //     await mintTo(
        //         connection,
        //         keypair,
        //         mint,
        //         tokenAccount,
        //         keypair,
        //         Math.floor(Math.random() * 1000000)
        //     )
        //     console.log(`Token account ${i} is: ${tokenAccount.toBase58()}`)
        //     console.log(`Minted some tokens to Token account ${i}`)
        // }

        
        const ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )
        console.log(`ATA is: ${ata.address.toString()}`);

        // Mint to ATA
        const mintTx = await mintTo(
            connection,
            keypair,
            mint,
            ata.address,
            keypair,
            3000000
        )
        console.log(`Your mint txid: ${mintTx}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
