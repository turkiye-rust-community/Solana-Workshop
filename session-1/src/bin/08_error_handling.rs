/// # Hata Yönetimi
///
/// Anchor programlarında hata yönetimi kritiktir:
/// - Tüm instruction handler'lar `Result<()>` döner
/// - `#[error_code]` ile özel hata enum'ları tanımlanır
/// - `?` operatörü ile hata yayılımı (propagation) sağlanır
/// - `require!`, `require_eq!`, `require_keys_eq!` macro'ları Anchor'a özgüdür
///
/// Edition 2024 notları:
/// - `?` operatörü davranışı aynı
/// - `std::error::Error` trait'i `Display` gerektirir (değişmedi)
/// - `From` trait impl'leri ile hata dönüşümü aynı

use std::fmt;
use std::num::ParseIntError;

fn main() {
    // =========================================================
    // 1. Result<T, E> temelleri
    // =========================================================
    println!("--- Result Temelleri ---");

    let ok: Result<u64, String> = Ok(1_000_000);
    let err: Result<u64, String> = Err("yetersiz bakiye".into());

    println!("ok = {ok:?}");
    println!("err = {err:?}");

    // is_ok / is_err
    println!("ok.is_ok() = {}", ok.is_ok());
    println!("err.is_err() = {}", err.is_err());

    // unwrap_or
    println!("err.unwrap_or(0) = {}", err.unwrap_or(0));

    // =========================================================
    // 2. ? operatörü — hata yayılımı
    // =========================================================
    println!("\n--- ? Operatörü ---");

    match parse_and_double("500000") {
        Ok(v) => println!("sonuç = {v}"),
        Err(e) => println!("hata = {e}"),
    }

    match parse_and_double("abc") {
        Ok(v) => println!("sonuç = {v}"),
        Err(e) => println!("hata = {e}"),
    }

    // =========================================================
    // 3. Özel hata tipi — Anchor'daki #[error_code] gibi
    // =========================================================
    println!("\n--- Özel Hata Tipi ---");

    match transfer(1_000_000, 2_000_000) {
        Ok(bal) => println!("yeni bakiye = {bal}"),
        Err(e) => println!("hata [{:?}]: {e}", e),
    }

    match transfer(1_000_000, 500_000) {
        Ok(bal) => println!("yeni bakiye = {bal}"),
        Err(e) => println!("hata [{:?}]: {e}", e),
    }

    match transfer(0, 500_000) {
        Ok(bal) => println!("yeni bakiye = {bal}"),
        Err(e) => println!("hata [{:?}]: {e}", e),
    }

    // =========================================================
    // 4. Hata dönüşümü — From trait
    // =========================================================
    println!("\n--- Hata Dönüşümü (From) ---");

    match process_deposit("1000000") {
        Ok(()) => println!("deposit başarılı"),
        Err(e) => println!("deposit hatası: {e}"),
    }

    match process_deposit("xyz") {
        Ok(()) => println!("deposit başarılı"),
        Err(e) => println!("deposit hatası: {e}"),
    }

    // =========================================================
    // 5. map_err — hata tipini dönüştür
    // =========================================================
    println!("\n--- map_err ---");

    let result: Result<u64, AppError> = "999"
        .parse::<u64>()
        .map_err(|e| AppError::ParseError(e));
    println!("map_err sonuç: {result:?}");

    // =========================================================
    // 6. and_then — zincirleme işlemler
    // =========================================================
    println!("\n--- and_then zinciri ---");

    let final_result = "500000"
        .parse::<u64>()
        .map_err(|e| AppError::ParseError(e))
        .and_then(|amount| validate_amount(amount))
        .and_then(|amount| apply_fee(amount, 300));

    match final_result {
        Ok(net) => println!("net amount = {net}"),
        Err(e) => println!("hata = {e}"),
    }

    // =========================================================
    // 7. Anchor bağlantısı
    // =========================================================
    println!("\n--- Anchor Hata Pattern'i ---");
    // Anchor'da tipik instruction handler:
    //
    // pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    //     let vault = &mut ctx.accounts.vault;
    //
    //     require!(amount > 0, EscrowError::ZeroAmount);
    //     require!(vault.total >= amount, EscrowError::InsufficientFunds);
    //     require!(!vault.is_locked, EscrowError::AccountLocked);
    //
    //     vault.total -= amount;
    //     Ok(())
    // }
    //
    // Bunu std ile simüle edelim:

    match anchor_like_withdraw(1_000_000, 500_000, false) {
        Ok(new_total) => println!("withdraw sonrası total = {new_total}"),
        Err(e) => println!("anchor hata: {e}"),
    }

    match anchor_like_withdraw(1_000_000, 0, false) {
        Ok(new_total) => println!("withdraw sonrası total = {new_total}"),
        Err(e) => println!("anchor hata: {e}"),
    }

    match anchor_like_withdraw(1_000_000, 500_000, true) {
        Ok(new_total) => println!("withdraw sonrası total = {new_total}"),
        Err(e) => println!("anchor hata: {e}"),
    }
}

// =========================================================
// Tipler
// =========================================================

#[derive(Debug)]
enum AppError {
    InsufficientFunds { balance: u64, requested: u64 },
    ZeroAmount,
    ParseError(ParseIntError),
    AccountLocked,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InsufficientFunds { balance, requested } => {
                write!(f, "Yetersiz bakiye: sahip={balance}, istek={requested}")
            }
            AppError::ZeroAmount => write!(f, "Sıfır miktar gönderilemez"),
            AppError::ParseError(e) => write!(f, "Parse hatası: {e}"),
            AppError::AccountLocked => write!(f, "Hesap kilitli"),
        }
    }
}

// ParseIntError -> AppError otomatik dönüşümü için
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

// =========================================================
// Fonksiyonlar
// =========================================================

// ? operatörü: ParseIntError döner
fn parse_and_double(s: &str) -> Result<u64, ParseIntError> {
    let n: u64 = s.parse()?; // parse başarısız olursa ? ile çıkar
    Ok(n * 2)
}

fn transfer(balance: u64, amount: u64) -> Result<u64, AppError> {
    if amount == 0 {
        return Err(AppError::ZeroAmount);
    }
    if balance < amount {
        return Err(AppError::InsufficientFunds { balance, requested: amount });
    }
    Ok(balance - amount)
}

// From impl sayesinde ? ile ParseIntError -> AppError dönüşümü otomatik
fn process_deposit(s: &str) -> Result<(), AppError> {
    let amount: u64 = s.parse()?; // ParseIntError otomatik AppError'a dönüşür
    println!("deposit amount = {amount}");
    Ok(())
}

fn validate_amount(amount: u64) -> Result<u64, AppError> {
    if amount == 0 {
        return Err(AppError::ZeroAmount);
    }
    Ok(amount)
}

fn apply_fee(amount: u64, fee_bps: u64) -> Result<u64, AppError> {
    let fee = amount * fee_bps / 10_000;
    Ok(amount - fee)
}

fn anchor_like_withdraw(total: u64, amount: u64, is_locked: bool) -> Result<u64, AppError> {
    // require!(amount > 0, ...)
    if amount == 0 {
        return Err(AppError::ZeroAmount);
    }
    // require!(!is_locked, ...)
    if is_locked {
        return Err(AppError::AccountLocked);
    }
    // require!(total >= amount, ...)
    if total < amount {
        return Err(AppError::InsufficientFunds { balance: total, requested: amount });
    }
    Ok(total - amount)
}
