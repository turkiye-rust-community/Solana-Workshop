// ============================================================
// Sol Transfer - Anchor Tests (Solana Playground)
// No imports needed: pg, web3, anchor, BN, assert are globally available
// ============================================================

describe("sol-transfer", () => {
  const PROGRAM_ID = pg.program.programId;
  const LAMPORTS_PER_SOL = web3.LAMPORTS_PER_SOL;

  const [vaultPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), pg.wallet.publicKey.toBuffer()],
    PROGRAM_ID
  );

  const recipient = web3.Keypair.generate();

  before(async () => {
    console.log("Vault PDA :", vaultPda.toBase58());
    console.log("Recipient :", recipient.publicKey.toBase58());
  });

  it("direct SOL transfer via CPI", async () => {
    const transferAmount = new BN(0.01 * LAMPORTS_PER_SOL);
    const before = await pg.connection.getBalance(recipient.publicKey);

    const tx = await pg.program.methods
      .transferSol(transferAmount)
      .accounts({ sender: pg.wallet.publicKey, recipient: recipient.publicKey })
      .rpc();
    console.log("Tx:", tx);

    const after = await pg.connection.getBalance(recipient.publicKey);
    assert.equal(after - before, transferAmount.toNumber());
    console.log("Alıcı bakiyesi:", after / LAMPORTS_PER_SOL, "SOL");
  });

  it("zero amount should fail (ZeroAmount)", async () => {
    try {
      await pg.program.methods
        .transferSol(new BN(0))
        .accounts({ sender: pg.wallet.publicKey, recipient: recipient.publicKey })
        .rpc();
      assert.fail("ZeroAmount hatası beklendi");
    } catch (err: any) {
      if (err instanceof anchor.AnchorError) {
        assert.equal(err.error.errorCode.code, "ZeroAmount");
        console.log("Beklenen hata:", err.error.errorCode.code);
      } else {
        throw err;
      }
    }
  });

  it("initialize vault", async () => {
    // Vault varsa atla
    try {
      await pg.program.account.vault.fetch(vaultPda);
      console.log("Vault zaten mevcut, atlanıyor.");
      return;
    } catch {}

    const tx = await pg.program.methods
      .initializeVault()
      .accounts({ vault: vaultPda, authority: pg.wallet.publicKey })
      .rpc();
    console.log("Tx:", tx);

    const vault = await pg.program.account.vault.fetch(vaultPda);
    assert.ok(vault.authority.equals(pg.wallet.publicKey));
    assert.equal(vault.totalDeposited.toNumber(), 0);
    console.log("Vault başlatıldı.");
  });

  it("deposit to vault", async () => {
    const depositAmount = new BN(0.05 * LAMPORTS_PER_SOL);
    const before = (await pg.program.account.vault.fetch(vaultPda)).totalDeposited.toNumber();

    const tx = await pg.program.methods
      .depositToVault(depositAmount)
      .accounts({ vault: vaultPda, depositor: pg.wallet.publicKey })
      .rpc();
    console.log("Tx:", tx);

    const after = (await pg.program.account.vault.fetch(vaultPda)).totalDeposited.toNumber();
    assert.equal(after - before, depositAmount.toNumber());
    console.log("total_deposited:", after, "lamport");
  });

  it("withdraw from vault", async () => {
    const withdrawAmount = new BN(0.02 * LAMPORTS_PER_SOL);
    const vaultBefore = await pg.connection.getBalance(vaultPda);
    assert.isAtLeast(vaultBefore, withdrawAmount.toNumber(), "Vault'ta yeterli SOL yok");

    const tx = await pg.program.methods
      .withdrawFromVault(withdrawAmount)
      .accounts({ vault: vaultPda, authority: pg.wallet.publicKey })
      .rpc();
    console.log("Tx:", tx);

    const vaultAfter = await pg.connection.getBalance(vaultPda);
    assert.equal(vaultBefore - vaultAfter, withdrawAmount.toNumber());
    console.log("Vault bakiyesi azaldı:", (vaultBefore - vaultAfter) / LAMPORTS_PER_SOL, "SOL");
  });
});
