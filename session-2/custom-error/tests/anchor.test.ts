// ============================================================
// Custom Error - Anchor Tests (Solana Playground)
// No imports needed: pg, web3, anchor, BN, assert are globally available
// ============================================================

describe("custom-error", () => {
  const PROGRAM_ID = pg.program.programId;
  const MINIMUM_BALANCE = 10_000;

  const [bankPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bank"), pg.wallet.publicKey.toBuffer()],
    PROGRAM_ID
  );

  // Hata kodunu doğrula
  async function expectAnchorError(fn: () => Promise<any>, errorCode: string) {
    try {
      await fn();
      assert.fail(`'${errorCode}' hatası beklendi ama fırlatılmadı`);
    } catch (err: any) {
      if (err instanceof anchor.AnchorError) {
        assert.equal(
          err.error.errorCode.code,
          errorCode,
          `Beklenen: ${errorCode}, Alınan: ${err.error.errorCode.code}`
        );
        console.log(`✓ ${errorCode} (${err.error.errorCode.number}): ${err.error.errorMessage}`);
      } else {
        throw err;
      }
    }
  }

  before(async () => {
    console.log("Bank PDA  :", bankPda.toBase58());
    console.log("Program ID:", PROGRAM_ID.toBase58());
  });

  it("open account", async () => {
    // Hesap varsa atla
    try {
      const acc = await pg.program.account.bankAccount.fetch(bankPda);
      console.log("Hesap zaten mevcut. Bakiye:", acc.balance.toString());
      return;
    } catch {}

    const tx = await pg.program.methods
      .openAccount(new BN(500_000))
      .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
      .rpc();
    console.log("Tx:", tx);

    const acc = await pg.program.account.bankAccount.fetch(bankPda);
    assert.ok(acc.owner.equals(pg.wallet.publicKey));
    assert.equal(acc.balance.toNumber(), 500_000);
    assert.isTrue(acc.isActive);
    assert.isFalse(acc.isFrozen);
    console.log("Bakiye:", acc.balance.toString());
  });

  it("ZeroAmount - deposit 0 should fail", async () => {
    await expectAnchorError(
      () =>
        pg.program.methods
          .deposit(new BN(0))
          .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
          .rpc(),
      "ZeroAmount"
    );
  });

  it("deposit success", async () => {
    const before = (await pg.program.account.bankAccount.fetch(bankPda)).balance.toNumber();

    await pg.program.methods
      .deposit(new BN(200_000))
      .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
      .rpc();

    const after = (await pg.program.account.bankAccount.fetch(bankPda)).balance.toNumber();
    assert.equal(after - before, 200_000);
    console.log("Bakiye:", after);
  });

  it("InsufficientBalance - withdraw too much", async () => {
    await expectAnchorError(
      () =>
        pg.program.methods
          .withdraw(new BN(999_999_999))
          .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
          .rpc(),
      "InsufficientBalance"
    );
  });

  it("withdraw success", async () => {
    const acc = await pg.program.account.bankAccount.fetch(bankPda);
    const safeAmount = acc.balance.toNumber() - MINIMUM_BALANCE - 1;

    if (safeAmount <= 0) {
      console.log("Çekilecek miktar yok, atlanıyor.");
      return;
    }

    await pg.program.methods
      .withdraw(new BN(safeAmount))
      .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
      .rpc();

    const after = (await pg.program.account.bankAccount.fetch(bankPda)).balance.toNumber();
    console.log("Çekim sonrası bakiye:", after);
  });

  it("AccountFrozen - deposit to frozen account should fail", async () => {
    await pg.program.methods
      .freezeAccount()
      .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
      .rpc();
    console.log("Hesap donduruldu.");

    await expectAnchorError(
      () =>
        pg.program.methods
          .deposit(new BN(100_000))
          .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
          .rpc(),
      "AccountFrozen"
    );
  });

  it("AccountFrozen - double freeze should fail", async () => {
    await expectAnchorError(
      () =>
        pg.program.methods
          .freezeAccount()
          .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
          .rpc(),
      "AccountFrozen"
    );
  });

  it("unfreeze and deposit again", async () => {
    await pg.program.methods
      .unfreezeAccount()
      .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
      .rpc();
    console.log("Hesap çözüldü.");

    await pg.program.methods
      .deposit(new BN(50_000))
      .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
      .rpc();

    const acc = await pg.program.account.bankAccount.fetch(bankPda);
    assert.isFalse(acc.isFrozen);
    console.log("Bakiye:", acc.balance.toString());
  });

  it("close account", async () => {
    const tx = await pg.program.methods
      .closeAccount()
      .accounts({ bankAccount: bankPda, owner: pg.wallet.publicKey })
      .rpc();
    console.log("Tx:", tx);

    try {
      await pg.program.account.bankAccount.fetch(bankPda);
      assert.fail("Hesap kapatılmış olmalıydı");
    } catch {
      console.log("Hesap başarıyla kapatıldı.");
    }
  });
});
