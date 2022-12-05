import * as anchor from "@project-serum/anchor";
import { web3 } from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
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
  
  // Try initializing IANA
  it('Inits IANA account', async () => {
    await program.methods.initIana().accounts({ owner, iana }).rpc()
  })

  // // Try initializing IANA and then AS
  // it('Inits IANA account', async () => {
  //   await program.methods.initIana().accounts({ owner, iana }).rpc()
  // })


  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
  
  // it("init", async () => {
  //   // Send transaction
  //   let ianaAcc = new web3.Keypair()
  //   const txHash = await program.methods
  //     .initIana()
  //     .accounts({
  //       iana: ianaAcc.publicKey,
  //       owner: ianaAcc.publicKey,
  //       systemProgram: web3.SystemProgram.programId,
  //     })
  //     .rpc();
  //   console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  //   // Confirm transaction
  //   await OriginValidation.connection.confirmTransaction(txHash);

  //   // Fetch the created account
  //   const ianaAccount = await program.account.iana.fetch(ianaAccountPk);

  //   // console.log("Fizz:", ianaAccount.fizz);
  //   // console.log("Buzz:", ianaAccount.buzz);
  //   // console.log("N:", ianaAccount.n.toString());
  // });

});
