import * as anchor from '@project-serum/anchor';
import { createTokenAccount, createMint, getTokenAccount } from '@project-serum/common'
import { web3, BN } from '@project-serum/anchor';
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createAssociatedTokenAccount,
} from '@project-serum/associated-token'

describe('solvent-contracts', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.getProvider();


  before("Test the token", async () => {
    const nftMint = await createMint(provider);
    const createNFTWalletIx = await createAssociatedTokenAccount(
      provider.wallet.publicKey,
      provider.wallet.publicKey,
      nftMint,
    );

    const tx = new Transaction();
    provider.send()
  });

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.SolventContracts;
    const tx = await program.rpc.initialize();
    console.log("Your transaction signature", tx);
  });
});
