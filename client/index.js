const { Connection, PublicKey, Program } = require('@solana/web3.js');

// Replace with your program ID
const programId = new PublicKey('CcXDtgNex3qFycGqSMDzY1dAMrdqLQN5h1RwNkv3PSvF');

// Replace with your RPC endpoint
const connection = new Connection('http://127.0.0.1:8899');

const main = async () => {
  // ... your Web3.js logic her
  const publicKey = new PublicKey('CcXDtgNex3qFycGqSMDzY1dAMrdqLQN5h1RwNkv3PSvF');
  const accountInfo = await connection.getAccountInfo(publicKey);

  if (accountInfo) {
    console.log(`Balance: ${accountInfo.lamports}`);
  } else {
    console.log('Account not found.');
  }
};

main().catch(console.error);
