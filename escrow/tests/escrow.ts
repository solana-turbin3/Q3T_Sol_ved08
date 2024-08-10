import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token"
import {Keypair, PublicKey, LAMPORTS_PER_SOL} from "@solana/web3.js"
import { assert } from "chai";
import {randomBytes} from "crypto"

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
  let takerAtaB
  const seed1 = new anchor.BN(randomBytes(8))
  const seed2 = new anchor.BN(randomBytes(8))
  const escrow1 = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), maker.publicKey.toBuffer(), seed1.toArrayLike(Buffer, "le", 8)],
    program.programId
  )[0]
  const escrow2 = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), maker.publicKey.toBuffer(), seed2.toArrayLike(Buffer, "le", 8)],
    program.programId
  )[0]

  it("Set up accounts and mints", async () => {
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
  });
  it("can create and deposit to 2 escrows", async() => {
    let vault = getAssociatedTokenAddressSync(mintA, escrow1,true,TOKEN_PROGRAM_ID) 
    const tx1 = await program.methods.make(seed1, new anchor.BN(100), new anchor.BN(100))
    .accounts({
      maker: maker.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      mintA: mintA,
      mintB: mintB,
    })
    .signers([maker])
    .rpc()
    const tx2 = await program.methods.make(seed2, new anchor.BN(69), new anchor.BN(420))
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
  it("can finalize an escrow1", async() => {
    let tx = await program.methods.take()
    .accountsPartial({
      taker: taker.publicKey,
      maker: maker.publicKey,
      mintA,
      mintB,
      escrow: escrow1,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([taker])
    .rpc()
  })
  it("can refund escrow 2", async() => {
    const tx2 = await program.methods.refund()
    .accountsPartial({
      maker: maker.publicKey,
      mintA: mintA,
      escrow: escrow2,
      tokenProgram: TOKEN_PROGRAM_ID
    })
    .signers([maker])
    .rpc()
  })
});
