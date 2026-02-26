// ============================================================
// Counter - Anchor Tests (Solana Playground)
// No imports needed: pg, web3, anchor, BN, assert are globally available
// ============================================================

describe("counter", () => {
  const PROGRAM_ID = pg.program.programId;

  const [counterPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), pg.wallet.publicKey.toBuffer()],
    PROGRAM_ID
  );

  before(async () => {
    console.log("Counter PDA:", counterPda.toBase58());
    console.log("Program ID :", PROGRAM_ID.toBase58());
  });

  it("initialize counter", async () => {
    // Hesap zaten varsa atla (devnet'te idempotent)
    try {
      await pg.program.account.counter.fetch(counterPda);
      console.log("Counter zaten mevcut, initialize atlanıyor.");
      return;
    } catch {}

    const tx = await pg.program.methods
      .initialize()
      .accounts({ counter: counterPda, authority: pg.wallet.publicKey })
      .rpc();
    console.log("Tx:", tx);

    const acc = await pg.program.account.counter.fetch(counterPda);
    assert.ok(acc.authority.equals(pg.wallet.publicKey));
    assert.equal(acc.count.toNumber(), 0);
    console.log("Initial count:", acc.count.toNumber());
  });

  it("increment counter", async () => {
    const before = (await pg.program.account.counter.fetch(counterPda)).count.toNumber();

    await pg.program.methods
      .increment()
      .accounts({ counter: counterPda, authority: pg.wallet.publicKey })
      .rpc();

    const after = (await pg.program.account.counter.fetch(counterPda)).count.toNumber();
    assert.equal(after, before + 1);
    console.log("Count:", after);
  });

  it("increment counter again", async () => {
    const before = (await pg.program.account.counter.fetch(counterPda)).count.toNumber();

    await pg.program.methods
      .increment()
      .accounts({ counter: counterPda, authority: pg.wallet.publicKey })
      .rpc();

    const after = (await pg.program.account.counter.fetch(counterPda)).count.toNumber();
    assert.equal(after, before + 1);
    console.log("Count:", after);
  });

  it("decrement counter", async () => {
    const before = (await pg.program.account.counter.fetch(counterPda)).count.toNumber();

    await pg.program.methods
      .decrement()
      .accounts({ counter: counterPda, authority: pg.wallet.publicKey })
      .rpc();

    const after = (await pg.program.account.counter.fetch(counterPda)).count.toNumber();
    assert.equal(after, before - 1);
    console.log("Count:", after);
  });

  it("reset counter", async () => {
    await pg.program.methods
      .reset()
      .accounts({ counter: counterPda, authority: pg.wallet.publicKey })
      .rpc();

    const acc = await pg.program.account.counter.fetch(counterPda);
    assert.equal(acc.count.toNumber(), 0);
    console.log("Count sıfırlandı:", acc.count.toNumber());
  });

  it("decrement below zero should fail (AlreadyZero)", async () => {
    // Sayaç 0'da olmalı (önceki test reset yaptı)
    try {
      await pg.program.methods
        .decrement()
        .accounts({ counter: counterPda, authority: pg.wallet.publicKey })
        .rpc();
      assert.fail("AlreadyZero hatası beklendi");
    } catch (err: any) {
      if (err instanceof anchor.AnchorError) {
        assert.equal(err.error.errorCode.code, "AlreadyZero");
        console.log("Beklenen hata:", err.error.errorCode.code);
      } else {
        throw err;
      }
    }
  });
});
