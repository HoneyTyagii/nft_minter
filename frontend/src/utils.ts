import { Connection, PublicKey, Keypair, Transaction, TransactionInstruction, Account } from '@solana/web3.js';

export function createConnection(): Connection {
  return new Connection('https://api.devnet.solana.com', 'confirmed'); 
}

export function createKeypair(): Keypair {
  return Keypair.generate();
}

export function createMintNftInstruction(programId: PublicKey, walletKeyPair: Keypair, nftAccount: Account): TransactionInstruction {
  return new TransactionInstruction({
    keys: [
      { pubkey: walletKeyPair.publicKey, isSigner: true, isWritable: false }, 
      { pubkey: nftAccount.publicKey, isSigner: false, isWritable: true },
    ], 
    programId: programId,
    data: Buffer.alloc(0), 
  });
}