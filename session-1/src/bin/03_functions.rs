/// # Fonksiyonlar
///
/// Edition 2024 notları:
/// - `unsafe fn` içindeki unsafe işlemler artık explicit `unsafe {}` bloğu gerektirir
/// - Diğer fonksiyon davranışları edition 2021 ile aynıdır
///
/// Anchor programlarında fonksiyonlar:
/// - Her instruction handler bir fonksiyondur: `pub fn initialize(ctx: Context<Initialize>) -> Result<()>`
/// - Yardımcı fonksiyonlar hesaplama ve doğrulama için kullanılır
/// - `impl` blokları içindeki metotlar account struct'larına davranış ekler

fn main() {
    // --- 1. Temel fonksiyon çağrısı ---
    greet("Solana");

    // --- 2. Değer döndüren fonksiyon ---
    let lamports = sol_to_lamports(1.5);
    println!("1.5 SOL = {lamports} lamport");

    // --- 3. Birden fazla parametre ---
    let fee = calculate_fee(1_000_000, 300);
    println!("Fee: {fee} lamport");

    // --- 4. Tuple döndürme ---
    let (amount_out, fee_paid) = swap(1_000_000, 300);
    println!("Swap: amount_out={amount_out}, fee_paid={fee_paid}");

    // --- 5. Expression vs Statement ---
    // Rust'ta bloklar değer döndürebilir (expression)
    let status = {
        let balance: u64 = 500_000_000;
        let threshold: u64 = 100_000_000;
        balance >= threshold // noktalı virgül YOK → bu değer döner
    };
    println!("Has enough balance: {status}");

    // --- 6. Early return ---
    match transfer(500_000_000, 1_000_000_000) {
        Ok(new_balance) => println!("Transfer sonrası: {new_balance}"),
        Err(e) => println!("Hata: {e}"),
    }
    match transfer(1_000_000_000, 500_000_000) {
        Ok(new_balance) => println!("Transfer sonrası: {new_balance}"),
        Err(e) => println!("Hata: {e}"),
    }

    // --- 7. Özyinelemeli (recursive) fonksiyon ---
    println!("2^10 = {}", power(2u64, 10));

    // --- 8. Edition 2024: unsafe fn içinde explicit unsafe blok ---
    // Edition 2021'de unsafe fn içindeki her şey otomatik unsafe sayılırdı.
    // Edition 2024'te unsafe fn içindeki unsafe işlemler explicit olmalı.
    // Örnek (raw pointer):
    let value: u64 = 42;
    let result = safe_wrapper(&value);
    println!("safe_wrapper sonucu: {result}");

    // --- 9. Anchor instruction handler pattern ---
    let mut vault_total: u64 = 0;
    simulate_deposit(&mut vault_total, 500_000);
    simulate_deposit(&mut vault_total, 300_000);
    println!("Vault total: {vault_total}");
}

// Basit fonksiyon — birim tip `()` döner
fn greet(name: &str) {
    println!("Merhaba, {name}!");
}

// Değer döndüren fonksiyon
// Son expression döner (return keyword olmadan)
fn sol_to_lamports(sol: f64) -> u64 {
    (sol * 1_000_000_000.0) as u64
}

// amount: lamport, fee_bps: basis points (100 = 1%)
fn calculate_fee(amount: u64, fee_bps: u64) -> u64 {
    amount * fee_bps / 10_000
}

// Tuple döndürme
fn swap(amount_in: u64, fee_bps: u64) -> (u64, u64) {
    let fee = calculate_fee(amount_in, fee_bps);
    let amount_out = amount_in - fee;
    (amount_out, fee)
}

// Early return — Anchor'daki Result<()> pattern'ini simüle eder
fn transfer(balance: u64, amount: u64) -> Result<u64, String> {
    if amount > balance {
        return Err(format!("Yetersiz bakiye: {balance} < {amount}"));
    }
    Ok(balance - amount)
}

// Özyinelemeli fonksiyon
fn power(base: u64, exp: u32) -> u64 {
    if exp == 0 {
        return 1;
    }
    base * power(base, exp - 1)
}

// Edition 2024: unsafe fn içinde unsafe blok explicit olmalı
// Bu fonksiyon unsafe blok içeriyor ama safe arayüz sunuyor
fn safe_wrapper(val: &u64) -> u64 {
    // raw pointer üzerinden okuma — unsafe gerektirir
    let ptr = val as *const u64;
    // SAFETY: ptr, geçerli bir &u64'ten türetildiği için her zaman geçerli
    unsafe { *ptr }
}

// Mutable referans alan fonksiyon
fn simulate_deposit(vault_total: &mut u64, amount: u64) {
    *vault_total += amount;
}
