// ============================================================
// Counter - Solana Playground Client
// No imports needed: pg, web3, anchor, BN are globally available
// ============================================================

const PROGRAM_ID = pg.program.programId;

// Counter PDA
const [counterPda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("counter"), pg.wallet.publicKey.toBuffer()],
  PROGRAM_ID
);

console.log("Cüzdan    :", pg.wallet.publicKey.toBase58());
console.log("Program ID:", PROGRAM_ID.toBase58());
console.log("Counter   :", counterPda.toBase58());
console.log();

// -------------------------------------------------------
// Mevcut state kontrol et
// -------------------------------------------------------
let counterAccount: any;
try {
  counterAccount = await pg.program.account.counter.fetch(counterPda);
  console.log("Mevcut count:", counterAccount.count.toString());
} catch {
  console.log("Counter bulunamadı, initialize ediliyor...");
  const initTx = await pg.program.methods
    .initialize()
    .accounts({
      counter: counterPda,
      authority: pg.wallet.publicKey,
    })
    .rpc();
  console.log("Initialize tx:", initTx);
  counterAccount = await pg.program.account.counter.fetch(counterPda);
  console.log("Başlangıç count:", counterAccount.count.toString());
}

// -------------------------------------------------------
// Increment (2x)
// -------------------------------------------------------
console.log("\n--- Increment ---");
for (let i = 0; i < 2; i++) {
  const tx = await pg.program.methods
    .increment()
    .accounts({ counter: counterPda, authority: pg.wallet.publicKey })
    .rpc();
  counterAccount = await pg.program.account.counter.fetch(counterPda);
  console.log(`Tx: ${tx}  →  count: ${counterAccount.count.toString()}`);
}

// -------------------------------------------------------
// Decrement (1x)
// -------------------------------------------------------
console.log("\n--- Decrement ---");
const decTx = await pg.program.methods
  .decrement()
  .accounts({ counter: counterPda, authority: pg.wallet.publicKey })
  .rpc();
counterAccount = await pg.program.account.counter.fetch(counterPda);
console.log(`Tx: ${decTx}  →  count: ${counterAccount.count.toString()}`);

// -------------------------------------------------------
// Reset
// -------------------------------------------------------
console.log("\n--- Reset ---");
const resetTx = await pg.program.methods
  .reset()
  .accounts({ counter: counterPda, authority: pg.wallet.publicKey })
  .rpc();
counterAccount = await pg.program.account.counter.fetch(counterPda);
console.log(`Tx: ${resetTx}  →  count: ${counterAccount.count.toString()}`);

console.log("\nTamamlandı.");
