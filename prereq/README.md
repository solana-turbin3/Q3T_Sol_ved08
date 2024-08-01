# Tx link
[https://explorer.solana.com/tx/5cgX1qe9cB3u2RQv5VNx17MRcbA1ZXmpjUVLFBX3BoWyvgXS9F9izQynA8Xvii8C648agyAZmBYVkctxu8r9iq9o?cluster=devnet](https://explorer.solana.com/tx/5cgX1qe9cB3u2RQv5VNx17MRcbA1ZXmpjUVLFBX3BoWyvgXS9F9izQynA8Xvii8C648agyAZmBYVkctxu8r9iq9o?cluster=devnet)
# Description
## keygen.ts
This file generates a new keypair and logs the public key and private key in the console. <br>
Use `yarn keygen` to run this file
## airdrop.ts
This file airdrops a few SOL(0.5 to be exact) to the devnet wallet address provided.<br>
Use `yarn airdrop` to run this file
## transfer.ts
This file creates an instruction to transfer SOL from sender's wallet to recipient's wallet.<br>
Use `yarn transfer` to run this file
## programs/wba_prereq.ts
This file stores the IDL file of the WBA program 
## enroll.ts
This file contains the final project code. A Program instance of the WBA program is created using the IDL file in `wba_prereq.ts`. Then complete() function is called from the program instance which creates a PDA to store my github username.<br>
Use `yarn enroll` to run this file