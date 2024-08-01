import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, clusterApiUrl, PublicKey } from "@solana/web3.js"
import wallet from "./dev-wallet.json"

const from = Keypair.fromSecretKey(new Uint8Array(wallet))
console.log("from: ", from.publicKey.toBase58())
const to = new PublicKey("J46MYawFotziZu2ztkPWrrks18GSaZ4xoMsCtgwLwpnF")
console.log("to: ", to.toBase58())

const cluster = clusterApiUrl("devnet")
const connection = new Connection(cluster);

(async() => {
    try {
        const transation = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: LAMPORTS_PER_SOL * 0.5
            })
        )
        transation.recentBlockhash = (await connection.getLatestBlockhash("confirmed")).blockhash
        transation.feePayer = from.publicKey

        const signature = await sendAndConfirmTransaction(
            connection,
            transation,
            [from]
        );
        console.log("Transaction successful txid: ", signature)
    }
    catch(e) {
        console.log(e)
    }
})()

// const bob = "J46MYawFotziZu2ztkPWrrks18GSaZ4xoMsCtgwLwpnF"