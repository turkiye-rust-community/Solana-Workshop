// ============================================================
// Custom Error - Solana Playground Client
// No imports needed: pg, web3, anchor, BN are globally available
// ============================================================

const PROGRAM_ID = pg.program.programId;
const MINIMUM_BALANCE = 10_000;

// Bank PDA
const [bankPda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("bank"), pg.wallet.publicKey.toBuffer()],
  PROGRAM_ID
);

// Hata bilgisini güzel yazdır
function printError(label: string, err: unknown) {
  if (err instanceof anchor.AnchorError) {
    console.log(`[${label}] Hata:`);
    console.log(`  Kod  : ${err.error.errorCode.number}`);
    console.log(`  Ad   : ${err.error.errorCode.code}`);
    console.log(`  Mesaj: ${err.error.errorMessage}`);
  } else if (err instanceof Error) {
    console.log(`[${label}] ${err.message}`);
  }
}

console.log("Cüzdan    :", pg.wallet.publicKey.toBase58());
console.log("Program ID:", PROGRAM_ID.toBase58());
console.log("Bank PDA  :", bankPda.toBase58());
console.log();

// -------------------------------------------------------
// Hesap aç (idempotent)
// -------------------------------------------------------
let acc: any;
try {
  acc = await pg.program.account.bankAccount.fetch(bankPda);
  console.log("Mevcut bakiye:", acc.balance.toString(), "lamport");
} catch {
  console.log("Hesap açılıyor...");
  const tx = await pg.program.methods
    .openAccount(new BN(500_000))
    .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
    .rpc();
  console.log("Tx:", tx);
  acc = await pg.program.account.bankAccount.fetch(bankPda);
  console.log("Açılış bakiyesi:", acc.balance.toString(), "lamport");
}
console.log();

// ============================================================
// SENARYO 1: ZeroAmount
// ============================================================
console.log("=== Senaryo 1: ZeroAmount ===");
try {
  await pg.program.methods
    .deposit(new BN(0))
    .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
    .rpc();
  console.log("BEKLENMEDIK: hata fırlatılmadı!");
} catch (err) {
  printError("ZeroAmount", err);
}
console.log();

// ============================================================
// SENARYO 2: Başarılı Yatırım
// ============================================================
console.log("=== Senaryo 2: Başarılı Yatırım ===");
const depositTx = await pg.program.methods
  .deposit(new BN(300_000))
  .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
  .rpc();
console.log("Tx:", depositTx);
acc = await pg.program.account.bankAccount.fetch(bankPda);
console.log("Yeni bakiye:", acc.balance.toString(), "lamport");
console.log();

// ============================================================
// SENARYO 3: InsufficientBalance
// ============================================================
console.log("=== Senaryo 3: InsufficientBalance ===");
try {
  await pg.program.methods
    .withdraw(new BN(999_999_999))
    .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
    .rpc();
  console.log("BEKLENMEDIK: hata fırlatılmadı!");
} catch (err) {
  printError("InsufficientBalance", err);
}
console.log();

// ============================================================
// SENARYO 4: AccountFrozen
// ============================================================
console.log("=== Senaryo 4: AccountFrozen ===");

// Dondur
await pg.program.methods
  .freezeAccount()
  .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
  .rpc();
console.log("Hesap donduruldu.");

// Dondurulmuş hesaba yatır → hata
try {
  await pg.program.methods
    .deposit(new BN(100_000))
    .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
    .rpc();
  console.log("BEKLENMEDIK: hata fırlatılmadı!");
} catch (err) {
  printError("AccountFrozen (deposit)", err);
}

// Tekrar dondur → hata
try {
  await pg.program.methods
    .freezeAccount()
    .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
    .rpc();
  console.log("BEKLENMEDIK: hata fırlatılmadı!");
} catch (err) {
  printError("AccountFrozen (double freeze)", err);
}
console.log();

// ============================================================
// SENARYO 5: Unfreeze & Withdraw
// ============================================================
console.log("=== Senaryo 5: Unfreeze & Withdraw ===");
await pg.program.methods
  .unfreezeAccount()
  .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
  .rpc();
console.log("Hesap çözüldü.");

acc = await pg.program.account.bankAccount.fetch(bankPda);
const safeAmount = acc.balance.toNumber() - MINIMUM_BALANCE - 1;
if (safeAmount > 0) {
  const wTx = await pg.program.methods
    .withdraw(new BN(safeAmount))
    .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
    .rpc();
  console.log("Withdraw Tx:", wTx);
  acc = await pg.program.account.bankAccount.fetch(bankPda);
  console.log("Son bakiye:", acc.balance.toString(), "lamport");
} else {
  console.log("Çekilecek miktar yok.");
}

console.log("\nTamamlandı.");
