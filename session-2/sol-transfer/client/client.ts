// ============================================================
// Sol Transfer - Solana Playground Client
// No imports needed: pg, web3, anchor, BN are globally available
// ============================================================

const PROGRAM_ID = pg.program.programId;
const LAMPORTS_PER_SOL = web3.LAMPORTS_PER_SOL;

// Vault PDA
const [vaultPda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("vault"), pg.wallet.publicKey.toBuffer()],
  PROGRAM_ID
);

async function getBalance(pk: web3.PublicKey): Promise<string> {
  const lamports = await pg.connection.getBalance(pk);
  return (lamports / LAMPORTS_PER_SOL).toFixed(6) + " SOL";
}

console.log("Cüzdan    :", pg.wallet.publicKey.toBase58());
console.log("Program ID:", PROGRAM_ID.toBase58());
console.log("Vault PDA :", vaultPda.toBase58());
console.log("Bakiye    :", await getBalance(pg.wallet.publicKey));
console.log();

// -------------------------------------------------------
// 1) Direkt SOL Transfer (CPI)
// -------------------------------------------------------
console.log("=== 1) Direkt CPI Transfer ===");
const recipient = web3.Keypair.generate().publicKey;
const transferAmount = new BN(0.001 * LAMPORTS_PER_SOL);

const transferTx = await pg.program.methods
  .transferSol(transferAmount)
  .accounts({
    sender: pg.wallet.publicKey,
    recipient,
  })
  .rpc();
console.log("Tx     :", transferTx);
console.log("Alıcı  :", recipient.toBase58());
console.log("Bakiye :", await getBalance(recipient));
console.log();

// -------------------------------------------------------
// 2) Vault Başlat (idempotent)
// -------------------------------------------------------
console.log("=== 2) Vault ===");
try {
  const v = await pg.program.account.vault.fetch(vaultPda);
  console.log("Vault mevcut. total_deposited:", v.totalDeposited.toString(), "lamport");
} catch {
  console.log("Vault başlatılıyor...");
  const tx = await pg.program.methods
    .initializeVault()
    .accounts({ vault: vaultPda, authority: pg.wallet.publicKey })
    .rpc();
  console.log("Tx:", tx);
}

// -------------------------------------------------------
// 3) Vault'a Yatır
// -------------------------------------------------------
console.log("\n--- Deposit ---");
const depositAmount = new BN(0.01 * LAMPORTS_PER_SOL);
const depositTx = await pg.program.methods
  .depositToVault(depositAmount)
  .accounts({ vault: vaultPda, depositor: pg.wallet.publicKey })
  .rpc();
console.log("Tx:", depositTx);
let vault = await pg.program.account.vault.fetch(vaultPda);
console.log("total_deposited:", vault.totalDeposited.toString(), "lamport");

// -------------------------------------------------------
// 4) Vault'tan Çek
// -------------------------------------------------------
console.log("\n--- Withdraw ---");
const withdrawAmount = new BN(0.005 * LAMPORTS_PER_SOL);
const withdrawTx = await pg.program.methods
  .withdrawFromVault(withdrawAmount)
  .accounts({ vault: vaultPda, authority: pg.wallet.publicKey })
  .rpc();
console.log("Tx:", withdrawTx);
vault = await pg.program.account.vault.fetch(vaultPda);
console.log("total_deposited:", vault.totalDeposited.toString(), "lamport");
console.log("Son bakiye:", await getBalance(pg.wallet.publicKey));

console.log("\nTamamlandı.");
