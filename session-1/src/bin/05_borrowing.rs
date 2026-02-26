/// # Borrowing (Ödünç Alma) ve Referanslar
///
/// Ownership'i aktarmadan veriye erişmek için referanslar kullanılır.
/// Anchor'da `&ctx.accounts.vault` ve `&mut ctx.accounts.vault` bu mekanizmayı kullanır.
///
/// Edition 2024 notları:
/// - NLL (Non-Lexical Lifetimes) iyileştirmeleri: borrow checker daha akıllı
/// - `impl Trait` dönüş tiplerinde lifetime capture kuralları değişti
/// - Temel borrow kuralları aynı kaldı

fn main() {
    // =========================================================
    // 1. Immutable Reference (&T)
    // =========================================================
    println!("--- Immutable Borrow ---");

    let vault_total: u64 = 5_000_000;
    let r1 = &vault_total;
    let r2 = &vault_total; // birden fazla immutable borrow — TAMAM
    println!("r1={r1}, r2={r2}");
    println!("orijinal hâlâ geçerli: {vault_total}");

    // Fonksiyona referans geçirme — sahiplik aktarılmaz
    let name = String::from("EscrowVault");
    let len = string_length(&name);
    println!("name={name}, len={len}"); // name hâlâ geçerli

    // =========================================================
    // 2. Mutable Reference (&mut T)
    // =========================================================
    println!("\n--- Mutable Borrow ---");

    let mut balance: u64 = 0;
    add_lamports(&mut balance, 1_000_000);
    add_lamports(&mut balance, 500_000);
    println!("balance = {balance}");

    // Kural: Aynı anda YALNIZCA BİR mutable borrow olabilir
    let mut amount: u64 = 100;
    let m1 = &mut amount;
    *m1 += 50;
    // let m2 = &mut amount; // HATA: m1 hâlâ aktifken m2 oluşturulamaz
    println!("amount = {amount}"); // m1 bu noktada artık kullanılmıyor (NLL)

    // Edition 2024 NLL: borrow, son kullanım noktasından sonra biter
    let mut data: u64 = 10;
    {
        let borrow = &mut data;
        *borrow += 5;
    } // borrow burada bitti
    let borrow2 = &mut data; // artık mümkün
    *borrow2 *= 2;
    println!("data = {data}");

    // =========================================================
    // 3. Slice Referansları
    // =========================================================
    println!("\n--- Slice ---");

    let scores: [u32; 5] = [10, 20, 30, 40, 50];
    let middle = &scores[1..4]; // [20, 30, 40]
    println!("middle = {middle:?}");

    let text = String::from("hello world");
    let word = first_word(&text);
    println!("ilk kelime: {word}");

    // &str bir string slice'tır — Anchor'da &str yerine String kullanılır
    let literal: &str = "program_seed";
    println!("literal = {literal}");

    // =========================================================
    // 4. Dangling Reference — Rust bunu engeller
    // =========================================================
    // Aşağıdaki kod DERLENMEZ:
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s // s drop edilecek, referans geçersiz kalır
    // }

    // =========================================================
    // 5. Anchor bağlantısı
    // =========================================================
    println!("\n--- Anchor Bağlantısı ---");

    // Anchor instruction handler pattern:
    // pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    //     let vault = &mut ctx.accounts.vault;  // mutable borrow
    //     let user  = &ctx.accounts.user;       // immutable borrow
    //     vault.total += amount;
    //     msg!("Deposited by {}", user.key());
    //     Ok(())
    // }

    let mut vault = VaultState { total: 0, bump: 254 };
    let depositor = "8xJ3...";

    // Anchor'daki & ve &mut kullanımını simüle et
    deposit_simulation(&mut vault, depositor, 1_000_000);
    println!("vault.total = {}", vault.total);

    // Hem immutable hem mutable aynı anda olmaz:
    // let r = &vault;
    // let m = &mut vault; // HATA: immutable borrow varken mutable borrow olamaz
    // println!("{}", r.total);

    // Çözüm: önce immutable işlemini bitir
    let _info = vault.total; // immutable borrow bitti
    let vm = &mut vault;
    vm.total += 1;
    println!("vault.total (sonra +1) = {}", vault.total);
}

fn string_length(s: &String) -> usize {
    s.len()
}

fn add_lamports(balance: &mut u64, amount: u64) {
    *balance += amount;
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    s
}

struct VaultState {
    total: u64,
    bump: u8,
}

fn deposit_simulation(vault: &mut VaultState, depositor: &str, amount: u64) {
    vault.total += amount;
    println!("[deposit] {} tarafından {} lamport yatırıldı (bump={})", depositor, amount, vault.bump);
}
