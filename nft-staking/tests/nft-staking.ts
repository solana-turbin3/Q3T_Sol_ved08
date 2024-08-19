import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { assert } from "chai";

describe("nft-staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.NftStaking as Program<NftStaking>;

  let pointsPerStake = 2;
  let freezePeriod = 2000;


  const signer = web3.Keypair.generate()
  
  const airdrop = async (wallet: web3.PublicKey) => {
    const tx = await provider.connection.requestAirdrop(wallet, web3.LAMPORTS_PER_SOL * 5);
    await provider.connection.confirmTransaction(tx)
  }


  it("can create nft collection and mint to user", async() => {
    
  })
  it("can initalize stake pool", async () => {
      await airdrop(signer.publicKey)
    // Add your test here.
    const stakeConfig = web3.PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId)[0]
    const tx = await program.methods.initStake(pointsPerStake, 1, freezePeriod)
    .accounts({
      tokenProgram: TOKEN_PROGRAM_ID,
      signer: signer.publicKey
    })
    .signers([signer])
    .rpc()

    const config = await program.account.stakeConfig.fetch(stakeConfig)
    assert(config.pointsPerStake == pointsPerStake, "Point per stake not matching")
    assert(config.freezePeriod == freezePeriod, "Freeze period not matching")
  });

  it("can initalize user account", async() => {
    const userAccount = await web3.PublicKey.findProgramAddressSync([Buffer.from("user"), signer.publicKey.toBuffer()], program.programId)[0]
    const tx = await program.methods.initUser()
    .accounts({
      signer: signer.publicKey
    })
    .signers([signer])
    .rpc()

    const user = await program.account.userAccount.fetch(userAccount)
    assert(user.amountStaked == 0, "Amount staked is not 0")
  })
});

