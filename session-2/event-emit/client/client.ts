// ============================================================
// Event Emit - Solana Playground Client
// No imports needed: pg, web3, anchor, BN are globally available
// ============================================================

const PROGRAM_ID = pg.program.programId;

// Profile PDA
const [profilePda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("profile"), pg.wallet.publicKey.toBuffer()],
  PROGRAM_ID
);

console.log("Cüzdan     :", pg.wallet.publicKey.toBase58());
console.log("Program ID :", PROGRAM_ID.toBase58());
console.log("Profile PDA:", profilePda.toBase58());
console.log();

// -------------------------------------------------------
// Event listener'ları kur
// -------------------------------------------------------
const listeners: number[] = [];

listeners.push(
  pg.program.addEventListener("userRegistered", (event: any) => {
    console.log("\n[EVENT] UserRegistered:");
    console.log("  Kullanıcı:", event.user.toBase58());
    console.log("  Username :", event.username);
    console.log("  Zaman    :", new Date(event.timestamp.toNumber() * 1000).toISOString());
  })
);

listeners.push(
  pg.program.addEventListener("messageSent", (event: any) => {
    console.log("\n[EVENT] MessageSent:");
    console.log("  Gönderen:", event.sender.toBase58());
    console.log("  Alıcı   :", event.recipient.toBase58());
    console.log("  Mesaj   :", event.message);
  })
);

listeners.push(
  pg.program.addEventListener("pointsAwarded", (event: any) => {
    console.log("\n[EVENT] PointsAwarded:");
    console.log("  Kullanıcı:", event.user.toBase58());
    console.log("  Puan     :", event.points.toString());
    console.log("  Sebep    :", event.reason);
    console.log("  Toplam   :", event.totalPoints.toString());
  })
);

// -------------------------------------------------------
// Profil oluştur (idempotent)
// -------------------------------------------------------
try {
  const profile = await pg.program.account.userProfile.fetch(profilePda);
  console.log("Mevcut profil:", profile.username, "| Puan:", profile.points.toString());
} catch {
  console.log("Profil oluşturuluyor...");
  const tx = await pg.program.methods
    .registerUser("workshop_user")
    .accounts({ profile: profilePda, authority: pg.wallet.publicKey })
    .rpc({ commitment: "confirmed" });
  console.log("Register tx:", tx);
}

// -------------------------------------------------------
// Mesaj gönder
// -------------------------------------------------------
console.log("\n--- sendMessage ---");
const recipientKey = web3.Keypair.generate().publicKey;
const msgTx = await pg.program.methods
  .sendMessage(recipientKey, "Merhaba Solana Workshop!")
  .accounts({ profile: profilePda, authority: pg.wallet.publicKey })
  .rpc({ commitment: "confirmed" });
console.log("Tx:", msgTx);

// -------------------------------------------------------
// Puan ver
// -------------------------------------------------------
console.log("\n--- awardPoints ---");
const awardTx = await pg.program.methods
  .awardPoints(new BN(100), "SolPG client üzerinden puan")
  .accounts({ profile: profilePda, authority: pg.wallet.publicKey })
  .rpc({ commitment: "confirmed" });
console.log("Tx:", awardTx);

// Event'lerin işlenmesi için bekle
await new Promise((resolve) => setTimeout(resolve, 3000));

// -------------------------------------------------------
// Güncel profil
// -------------------------------------------------------
const profile = await pg.program.account.userProfile.fetch(profilePda);
console.log("\n--- Güncel Profil ---");
console.log("Username    :", profile.username);
console.log("Puan        :", profile.points.toString());
console.log("Mesaj sayısı:", profile.messageCount.toString());

// Listener'ları temizle
for (const id of listeners) {
  await pg.program.removeEventListener(id);
}

console.log("\nTamamlandı.");
