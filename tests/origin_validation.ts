import * as anchor from "@project-serum/anchor";
import { web3 } from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { getMultipleAccounts } from "@project-serum/anchor/dist/cjs/utils/rpc";
import { OriginValidation } from "../target/types/origin_validation";

describe("origin_validation", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.OriginValidation as Program<OriginValidation>;
  //  the address of the Anchor provider's wallet, which signs every transaction that we send to our Solana localnet
  const owner = provider.wallet.publicKey

  // PDA, iana account owned by `owner`
  const iana = web3.PublicKey.findProgramAddressSync(
    [Buffer.from('iana-account'), owner.toBuffer()],
    program.programId
  )[0]

  const as = web3.PublicKey.findProgramAddressSync(
    [Buffer.from('as-account'), owner.toBuffer()],
    program.programId
  )[0]

  const prefix = web3.PublicKey.findProgramAddressSync(
    [Buffer.from('prefix-account'), owner.toBuffer()],
    program.programId
  )[0]
  
  // Try initializing IANA
  it('Inits IANA account', async () => {
    await program.methods.initIana().accounts({ owner, iana }).rpc()
  })

  // Try initializing an AS
  it('Inits AS account', async () => {
    await program.methods.initAs().accounts({ owner, iana, as }).rpc()
  })

  // Try initializing a prefix
  it('Inits a prefix', async () => {
    // add 10.0.0.1/16 for AS 0
    let ip_prefix = 167772161
    let ip_mask = 16
    await program.methods.initPrefix(ip_prefix, ip_mask).accounts({ owner, iana, as, prefix }).rpc()
  })

  // it('Get ASN from prefix', async () => {
  //   let ip_prefix = 167772161
  //   let ip_mask = 16
    
  //   // program.views.initAs
  // })

});
