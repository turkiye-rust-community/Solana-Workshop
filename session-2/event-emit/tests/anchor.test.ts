// ============================================================
// Event Emit - Anchor Tests (Solana Playground)
// No imports needed: pg, web3, anchor, BN, assert are globally available
// ============================================================

describe("event-emit", () => {
  const PROGRAM_ID = pg.program.programId;

  const [profilePda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), pg.wallet.publicKey.toBuffer()],
    PROGRAM_ID
  );

  // Yakalanan event'leri tut
  const events: { name: string; data: any }[] = [];
  const listenerIds: number[] = [];

  before(async () => {
    console.log("Profile PDA:", profilePda.toBase58());

    listenerIds.push(
      pg.program.addEventListener("userRegistered", (e: any) => {
        events.push({ name: "userRegistered", data: e });
        console.log("[EVENT] UserRegistered:", e.username);
      })
    );
    listenerIds.push(
      pg.program.addEventListener("messageSent", (e: any) => {
        events.push({ name: "messageSent", data: e });
        console.log("[EVENT] MessageSent:", e.message);
      })
    );
    listenerIds.push(
      pg.program.addEventListener("pointsAwarded", (e: any) => {
        events.push({ name: "pointsAwarded", data: e });
        console.log("[EVENT] PointsAwarded:", e.points.toString(), "puan");
      })
    );
  });

  after(async () => {
    for (const id of listenerIds) {
      await pg.program.removeEventListener(id);
    }
  });

  it("register user → UserRegistered event", async () => {
    // Profil varsa atla
    try {
      await pg.program.account.userProfile.fetch(profilePda);
      console.log("Profil zaten mevcut, atlanıyor.");
      return;
    } catch {}

    const tx = await pg.program.methods
      .registerUser("alice")
      .accounts({ profile: profilePda, authority: pg.wallet.publicKey })
      .rpc({ commitment: "confirmed" });
    console.log("Tx:", tx);

    await new Promise((r) => setTimeout(r, 2000));

    const profile = await pg.program.account.userProfile.fetch(profilePda);
    assert.equal(profile.username, "alice");
    assert.equal(profile.points.toNumber(), 0);
    assert.equal(profile.messageCount.toNumber(), 0);
  });

  it("send message → MessageSent event", async () => {
    const recipient = web3.Keypair.generate().publicKey;

    const tx = await pg.program.methods
      .sendMessage(recipient, "Merhaba Solana!")
      .accounts({ profile: profilePda, authority: pg.wallet.publicKey })
      .rpc({ commitment: "confirmed" });
    console.log("Tx:", tx);

    await new Promise((r) => setTimeout(r, 2000));

    const profile = await pg.program.account.userProfile.fetch(profilePda);
    assert.isAtLeast(profile.messageCount.toNumber(), 1);
    console.log("Mesaj sayısı:", profile.messageCount.toNumber());
  });

  it("award points → PointsAwarded event", async () => {
    const before = (await pg.program.account.userProfile.fetch(profilePda)).points.toNumber();

    const tx = await pg.program.methods
      .awardPoints(new BN(100), "Test puanı")
      .accounts({ profile: profilePda, authority: pg.wallet.publicKey })
      .rpc({ commitment: "confirmed" });
    console.log("Tx:", tx);

    await new Promise((r) => setTimeout(r, 2000));

    const after = (await pg.program.account.userProfile.fetch(profilePda)).points.toNumber();
    assert.equal(after - before, 100);
    console.log("Toplam puan:", after);
  });

  it("empty username should fail (EmptyUsername)", async () => {
    const newUser = web3.Keypair.generate();

    // Airdrop
    const sig = await pg.connection.requestAirdrop(
      newUser.publicKey,
      0.1 * web3.LAMPORTS_PER_SOL
    );
    await pg.connection.confirmTransaction(sig, "confirmed");

    const [newProfilePda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), newUser.publicKey.toBuffer()],
      PROGRAM_ID
    );

    try {
      await pg.program.methods
        .registerUser("")
        .accounts({ profile: newProfilePda, authority: newUser.publicKey })
        .signers([newUser])
        .rpc();
      assert.fail("EmptyUsername hatası beklendi");
    } catch (err: any) {
      if (err instanceof anchor.AnchorError) {
        assert.equal(err.error.errorCode.code, "EmptyUsername");
        console.log("Beklenen hata:", err.error.errorCode.code);
      } else {
        throw err;
      }
    }
  });
});
