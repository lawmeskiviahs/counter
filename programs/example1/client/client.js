const assert = require("assert");
const anchor = require("@project-serum/anchor");
const solanaWeb3 = require('@solana/web3.js');

// import { SystemProgram } from '@solana/web3.js';
// const SystemProgram = require("@solana/web3.js")
const { SystemProgram } = anchor.web3;
const provider = anchor.Provider.env();
anchor.setProvider(provider);
// const path = anchor.Provider.env.generate;
const program = anchor.workspace.Example1;
const programId = new anchor.web3.PublicKey("4Y8PGKfY7q5hxDA17h6UHk5eACo6k9idy1chiHb7HKsp");
// async function main (){}
async function deposit() {
  const userPubKey = new anchor.web3.PublicKey("5wbnW6cAoT2Rdwk5ecdgpCzoCp3g9rV7Vr5jNFAJBbCY");
  const vaultPubKey = new anchor.web3.PublicKey("9jud739eoWfqPcJ3h2x7oLVeT4ULJCwHTt59cRg4taBb");
  const remainingAccounts=[];
  // console.log(".....>>>>>",provider);
    const idl = await anchor.Program.fetchIdl(programId, provider);
  const program = new anchor.Program(idl, programId, provider);
  let amount = 2;
    
    await program.rpc.deposit(amount,{
      accounts: {
          pool: userPubKey,
          vault: vaultPubKey,
         // referrer: userPubKey,
          //systemProgram: SystemProgram.programId,
          // rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          // clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          // recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
          // instructionSysvarAccount: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      },
      remainingAccounts:
        remainingAccounts.length > 0 ? remainingAccounts : undefined,
    });
}
deposit();

console.log('deposit function called.')