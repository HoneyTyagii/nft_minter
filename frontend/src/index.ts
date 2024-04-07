import { Connection, PublicKey, Keypair, Transaction, TransactionInstruction, Account } from '@solana/web3.js';

const PROGRAM_ID = new PublicKey('YOUR_PROGRAM_ID');

async function mintNFT(): Promise<void> {
  // Connect to Solana
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  const walletKeyPair = Keypair.generate();

  const nftAccount = new Account();

  const mintNftInstruction = new TransactionInstruction({
    keys: [
      { pubkey: walletKeyPair.publicKey, isSigner: true, isWritable: false },
      { pubkey: nftAccount.publicKey, isSigner: false, isWritable: true },
    ],
    programId: PROGRAM_ID,
    data: Buffer.alloc(0), 
  });

  // Create a new transaction
  const transaction = new Transaction().add(mintNftInstruction);

  // Sign the transaction
  transaction.recentBlockhash = (await connection.getRecentBlockhash()).blockhash;
  transaction.sign(walletKeyPair);

  // Send the transaction
  const signature = await connection.sendTransaction(transaction, [walletKeyPair]);

  // Wait for confirmation
  await connection.confirmTransaction(signature);

  console.log('NFT Minted successfully!');
}

mintNFT().catch(console.error);