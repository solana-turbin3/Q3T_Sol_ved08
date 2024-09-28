import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { NftAuction } from "../target/types/nft_auction";
import { createSignerFromKeypair, generateSigner, keypairIdentity, percentAmount, signerIdentity, sol } from "@metaplex-foundation/umi"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { create, mplCore } from "@metaplex-foundation/mpl-core"
import base58  from "bs58";
import { toWeb3JsInstruction, toWeb3JsKeypair } from "@metaplex-foundation/umi-web3js-adapters";
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { assert } from "chai";

describe("nft-auction", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const connection = provider.connection;
  const seller = anchor.web3.Keypair.generate();
  const bidder1 = anchor.web3.Keypair.generate();
  const bidder2 = anchor.web3.Keypair.generate();
  const bidder3 = anchor.web3.Keypair.generate();
  const bidder4 = anchor.web3.Keypair.generate();
  const bidder5 = anchor.web3.Keypair.generate();
  let tx: string;
  const {web3} = anchor
  const program = anchor.workspace.NftAuction as Program<NftAuction>;
  
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
  
  const umi = createUmi("http://localhost:8899")
  const umiKeypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(seller.secretKey))
  const umiSeller = createSignerFromKeypair(umi, umiKeypair)
  umi.use(signerIdentity(umiSeller))
  .use(mplTokenMetadata())

  const mint = generateSigner(umi)
  it("Airdropped all the required wallets", async() => {
      tx = await connection.requestAirdrop(seller.publicKey, 10 * web3.LAMPORTS_PER_SOL);
      await connection.confirmTransaction(tx);
      
      tx = await connection.requestAirdrop(bidder1.publicKey, 5 * web3.LAMPORTS_PER_SOL);
      await connection.confirmTransaction(tx);
      tx = await connection.requestAirdrop(bidder2.publicKey, 5 * web3.LAMPORTS_PER_SOL);
      await connection.confirmTransaction(tx);
      tx = await connection.requestAirdrop(bidder3.publicKey, 5 * web3.LAMPORTS_PER_SOL);
      await connection.confirmTransaction(tx);
      tx = await connection.requestAirdrop(bidder4.publicKey, 5 * web3.LAMPORTS_PER_SOL);
      await connection.confirmTransaction(tx);
      tx = await connection.requestAirdrop(bidder5.publicKey, 5 * web3.LAMPORTS_PER_SOL);
      await connection.confirmTransaction(tx);

      // console.log("Bidder 4 is: ", bidder4.publicKey.toString())
  })
  it("Mints a new NFT to Seller", async () => {
    await umi.rpc.airdrop(umiKeypair.publicKey, sol(10))
    let tx = createNft(umi, {
      mint,
      name: "Test NFT",
      symbol: "TST",
      sellerFeeBasisPoints: percentAmount(1),
      uri: 'https://arweave.net/9ypjRmkaxsa5kZdmz0vDH3WcASN_VYW7LYNEeIEEhS4'
    })
    let result = await tx.sendAndConfirm(umi)
    const signature = base58.encode(result.signature)
    // console.log("NFT Minted: ", mint.publicKey.toString())
  })

  it("Seller starts an auction with floor price 1000 Lamports", async() => {
    const floorPrice = new anchor.BN(1_000);
    const endTime = new anchor.BN(Date.now() / 1000 + 5)
    const auctionAddress = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("auction"), toWeb3JsKeypair(mint).publicKey.toBuffer()], program.programId
    )[0]
    // const rewardPubkeys = web3.PublicKey.findProgramAddressSync(
    //   [Buffer.from("rewards"), auctionAddress.toBuffer()], program.programId
    // )[0]
    // console.log(`Reward Pubkeys 1: ${rewardPubkeys.toString()}`)
    await program.methods.initializeAuction(floorPrice, endTime)
    .accountsPartial({
      seller: seller.publicKey,
      sellerNftMint: mint.publicKey,
      auction: auctionAddress,
      tokenProgram: TOKEN_PROGRAM_ID
    })
    .signers([seller])
    .rpc()
    const auction = await program.account.auction.fetch(auctionAddress)

    // console.log("Auction initialized: ", auctionAddress.toString())
    assert(auction.ended === false)
  })
  it("Person 1 bids 1500 Lamports", async() =>  {
    const auctionAddress = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("auction"), toWeb3JsKeypair(mint).publicKey.toBuffer()], program.programId
    )[0]
    let auction = await program.account.auction.fetch(auctionAddress)
    let rewardPubkeys = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("rewards"), auctionAddress.toBuffer()], program.programId
    )[0]
    let bidderVault = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bidderVault"), auctionAddress.toBuffer()],
      program.programId
    )[0]

    await program.methods.bid(new anchor.BN(1500))
    .accountsPartial({
      bidder: bidder1.publicKey,
      auction: auctionAddress,
      bidderVault,
      systemProgram: web3.SystemProgram.programId,
      previousBidder: null,
    })
    .signers([bidder1])
    .rpc()
    
    auction = await program.account.auction.fetch(auctionAddress)
    // let rewardPubkeysData = await program.account.rewardPubkeys.fetch(rewardPubkeys)
    // console.log("Bid placed for: ", 1010)
    // console.log("Reward List: ", rewardPubkeysData.rewardList)

    assert(auction.currentBidder.equals(bidder1.publicKey))
  })

  it("Person 2 bids 2500 Lamports", async() =>  {
    const auctionAddress = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("auction"), toWeb3JsKeypair(mint).publicKey.toBuffer()], program.programId
    )[0]
    let auction = await program.account.auction.fetch(auctionAddress)
    let rewardPubkeys = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("rewards"), auctionAddress.toBuffer()], program.programId
    )[0]
    let bidderVault = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bidderVault"), auctionAddress.toBuffer()],
      program.programId
    )[0]

    await program.methods.bid(new anchor.BN(2500))
    .accountsPartial({
      bidder: bidder2.publicKey,
      auction: auctionAddress,
      bidderVault,
      systemProgram: web3.SystemProgram.programId,
      previousBidder: null,
      
    })
    .signers([bidder2])
    .rpc()
    
    auction = await program.account.auction.fetch(auctionAddress)
    // let rewardPubkeysData = await program.account.rewardPubkeys.fetch(rewardPubkeys)
    // console.log("Bid placed for: ", 2010)
    // console.log("Reward List: ", rewardPubkeysData.rewardList)

    assert(auction.currentBidder.equals(bidder2.publicKey))
  })
  it("Person 3 tries to lowball and fails", async() =>  {
    const auctionAddress = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("auction"), toWeb3JsKeypair(mint).publicKey.toBuffer()], program.programId
    )[0]
    let auction = await program.account.auction.fetch(auctionAddress)
    let rewardPubkeys = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("rewards"), auctionAddress.toBuffer()], program.programId
    )[0]
    let bidderVault = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bidderVault"), auctionAddress.toBuffer()],
      program.programId
    )[0]
    try {
      await program.methods.bid(new anchor.BN(1))
      .accountsPartial({
        bidder: bidder3.publicKey,
        auction: auctionAddress,
        bidderVault,
        systemProgram: web3.SystemProgram.programId,
        previousBidder: null,
        
      })
      .signers([bidder3])
      .rpc()
      assert.fail("Lowball bid should have failed")
    } catch(_e) {
      assert.ok(true)
    }
  })
  it("Person 4 bids 0.5 SOL", async() =>  {
    const auctionAddress = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("auction"), toWeb3JsKeypair(mint).publicKey.toBuffer()], program.programId
    )[0]
    let auction = await program.account.auction.fetch(auctionAddress)
    let rewardPubkeys = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("rewards"), auctionAddress.toBuffer()], program.programId
    )[0]
    let bidderVault = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bidderVault"), auctionAddress.toBuffer()],
      program.programId
    )[0]

    await program.methods.bid(new anchor.BN(web3.LAMPORTS_PER_SOL / 2))
    .accountsPartial({
      bidder: bidder4.publicKey,
      auction: auctionAddress,
      bidderVault,
      systemProgram: web3.SystemProgram.programId,
      previousBidder: null,
      
    })
    .signers([bidder4])
    .rpc()
    
    auction = await program.account.auction.fetch(auctionAddress)
    // let rewardPubkeysData = await program.account.rewardPubkeys.fetch(rewardPubkeys)
    // console.log("Reward List: ", rewardPubkeysData.rewardList)

    assert(auction.currentBidder.equals(bidder4.publicKey))
  })
  it("Person 5 bids after the auction ends and fails", async() =>  {
    // Move 5.001 seconds in future
    await sleep(5001)

    const auctionAddress = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("auction"), toWeb3JsKeypair(mint).publicKey.toBuffer()], program.programId
    )[0]
    let auction = await program.account.auction.fetch(auctionAddress)
    // let rewardPubkeys = web3.PublicKey.findProgramAddressSync(
    //   [Buffer.from("rewards"), auctionAddress.toBuffer()], program.programId
    // )[0]
    let bidderVault = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bidderVault"), auctionAddress.toBuffer()],
      program.programId
    )[0]
    try {
        await program.methods.bid(new anchor.BN(web3.LAMPORTS_PER_SOL))
        .accountsPartial({
          bidder: bidder5.publicKey,
          auction: auctionAddress,
          bidderVault,
          systemProgram: web3.SystemProgram.programId,
          previousBidder: null,
          
        })
        .signers([bidder5])
        .rpc()
        
        auction = await program.account.auction.fetch(auctionAddress)
      
        assert.fail("Auction ended how did this pass?")
    } catch(e) {
      assert.ok(true)
    }
  })
  it("Auction is resolved. Person 4 recieves the NFT and Seller gets 0.5 Sol", async() => {
    const auctionAddress = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("auction"), toWeb3JsKeypair(mint).publicKey.toBuffer()], program.programId
    )[0]
    const bidderVault = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bidderVault"), auctionAddress.toBuffer()], program.programId
    )[0]
    const nftVault = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("nft_vault"), toWeb3JsKeypair(mint).publicKey.toBuffer()], program.programId
    )[0]
    await program.methods.resolveAuction()
    .accountsPartial({
      nftMint: mint.publicKey,
      seller: seller.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      auction: auctionAddress,
      nftVault,
      bidderVault,
      reciever: bidder4.publicKey
    })
    .signers([seller])
    .rpc()
    const recieverAta = await connection.getTokenAccountsByOwner(bidder4.publicKey, {
      mint: new web3.PublicKey(mint.publicKey.toString()),
    })
    const recieverNftBalance = (await connection.getTokenAccountBalance(recieverAta.value[0].pubkey)).value.amount
    // console.log(recieverAta.value[0].pubkey.toString())
    assert(recieverNftBalance === "1")
  })
});
