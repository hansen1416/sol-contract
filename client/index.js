const { Connection, PublicKey, Program } = require('@solana/web3.js');

// Replace with your program ID
const programId = new PublicKey('Fg6PaFkwSMi2XwCnkh3t7w3NStqR6USo4kU5WAzp5uPB3');

// Replace with your RPC endpoint
const connection = new Connection('https://api.devnet.solana.com');

const main = async () => {
  // ... your Web3.js logic her
  const publicKey = new PublicKey('YourPublicKeyHere');
  const accountInfo = await connection.getAccountInfo(publicKey);

  if (accountInfo) {
    console.log(`Balance: ${accountInfo.lamports}`);
  } else {
    console.log('Account not found.');
  }
};

main().catch(console.error);
