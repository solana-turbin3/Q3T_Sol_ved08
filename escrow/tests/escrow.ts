import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { Account, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token"
import {Keypair, PublicKey, LAMPORTS_PER_SOL} from "@solana/web3.js"
import { assert } from "chai";
import {randomBytes} from "crypto"
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const connection = provider.connection

  const program = anchor.workspace.Escrow as Program<Escrow>;

  const maker = Keypair.generate();
  const taker = Keypair.generate();

  let mintA: PublicKey;
  let mintB: PublicKey;
  let makerAtaA;
  let makerAtaB;
  let takerAtaA
  let takerAtaB
  let accounts: any = {}
  const seed = new anchor.BN(100)
  const [escrow, _bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), maker.publicKey.toBuffer(), seed.toArrayLike(Buffer, "le", 8)],
    program.programId
  )

  it("Set up accounts and mints", async () => {
    // Add your test here.
    let tx1 = await connection.requestAirdrop(maker.publicKey, LAMPORTS_PER_SOL*10)
    await connection.confirmTransaction(tx1)
    let tx2 = 
    await connection.requestAirdrop(taker.publicKey, LAMPORTS_PER_SOL*10)
    await connection.confirmTransaction(tx2)

    mintA = await createMint(
      connection,
      maker,
      maker.publicKey,
      null,
      0
    )
    mintB = await createMint(
      connection,
      taker,
      taker.publicKey,
      null,
      0
    )
    makerAtaA = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mintA,
      maker.publicKey,
    )
    makerAtaB = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mintB,
      maker.publicKey
    )
    takerAtaA = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintA,
      taker.publicKey
    )
    takerAtaB = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintB,
      taker.publicKey
    )
    
    await mintTo(
      connection,
      maker,
      mintA,
      makerAtaA.address,
      maker,
      10000
    )
    await mintTo(
      connection,
      taker,
      mintB,
      takerAtaB.address,
      taker,
      10000
    )
    assert((await connection.getTokenAccountBalance(makerAtaA.address)).value.uiAmount == 10000)
    assert((await connection.getTokenAccountBalance(takerAtaB.address)).value.uiAmount == 10000)
  });
  it("can create and deposit to escrow", async() => {
    let ataA = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mintA,
      maker.publicKey,
    )
    let vault = getAssociatedTokenAddressSync(mintA, escrow,true,TOKEN_PROGRAM_ID) 
    const _tx = await program.methods.make(seed, new anchor.BN(100), new anchor.BN(100))
    .accounts({
      maker: maker.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      mintA: mintA,
      mintB: mintB,
    })
    .signers([maker])
    .rpc()
    assert((await connection.getTokenAccountBalance(vault)).value.uiAmount == 100)
  })
  it("can finalize an escrow", async() => {

    // let tx2 = await program.methods.take()
    // .accounts({
    //   taker: taker.publicKey,
    //   maker: maker.publicKey,
    //   tokenProgram: TOKEN_PROGRAM_ID,
    //   mintA: mintA,
    //   mintB,
    // })
    // .signers([taker])
    // .rpc()
    // console.log(tx2)
  })
  it("can cancel an escrow", async() => {

    const tx = await program.methods.refund()
    .accounts({
      maker: maker.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      mintA: mintA
    })
    .signers([maker])
    .rpc()
    console.log(tx)
  })
});
