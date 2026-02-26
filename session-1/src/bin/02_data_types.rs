/// # Veri Tipleri
///
/// Edition 2024 notları:
/// - `unsafe` bloklar artık daha kısıtlı (unsafe_op_in_unsafe_fn lint varsayılan error)
/// - `gen` keyword'ü rezerve edildi
/// - Float kullanımı: Solana on-chain kodda yine de kaçınılmalı
///
/// Anchor/Solana programlarında sıkça kullanılan tipler:
/// - `u8`  : bump seed, discriminator byte, token decimals
/// - `u64` : lamport miktarı, token miktarı, slot numarası
/// - `i64` : Unix timestamp (Clock::get().unix_timestamp)
/// - `u32` : fee, sequence numarası
/// - `bool`: flag alanları (is_initialized, is_locked vb.)
/// - `[u8; 32]` : Pubkey bayt dizisi
/// - `String` / `Vec<u8>` : değişken uzunluklu alanlar

fn main() {
    // =========================================================
    // SCALAR (Skaler) Tipler
    // =========================================================

    // --- Integer ---
    let decimals: u8 = 6;               // Token decimals (USDC = 6)
    let bump: u8 = 254;                 // PDA bump
    let amount: u64 = 1_000_000;        // 1 USDC (6 decimals)
    let slot: u64 = 300_000_000;        // Güncel slot
    let timestamp: i64 = 1_700_000_000; // Unix timestamp
    let fee_bps: u16 = 300;             // 3% (basis points)

    println!("--- Integer Tipleri ---");
    println!("decimals (u8) : {decimals}");
    println!("bump (u8)     : {bump}");
    println!("amount (u64)  : {amount}");
    println!("slot (u64)    : {slot}");
    println!("timestamp(i64): {timestamp}");
    println!("fee_bps (u16) : {fee_bps}");

    // Taşma (overflow) — Anchor'da checked_add, saturating_add kullanılır
    let max_u64: u64 = u64::MAX;
    println!("\nu64::MAX = {max_u64}");
    let safe_add = max_u64.checked_add(1); // None döner, panic yok
    println!("checked_add(1) = {safe_add:?}"); // None

    // Float: Solana on-chain kodda KULLANMAYIN
    // Fixed-point arithmetic tercih edin: amount * 10^decimals
    // Sadece off-chain / test kodunda kabul edilebilir.

    // --- Boolean ---
    let is_locked: bool = false;
    let has_authority: bool = true;
    println!("\n--- Boolean ---");
    println!("is_locked={is_locked}, has_authority={has_authority}");

    // --- Char (Solana'da nadir) ---
    let grade: char = 'A';
    println!("grade: {grade}");

    // =========================================================
    // COMPOUND (Bileşik) Tipler
    // =========================================================

    // --- Tuple ---
    let mint_info: (&str, u8, u64) = ("EPjFWdd5...", 6, 1_000_000);
    println!("\n--- Tuple ---");
    println!("mint={}, decimals={}, supply={}", mint_info.0, mint_info.1, mint_info.2);

    let (mint, dec, supply) = mint_info; // destructuring
    println!("Destructured: mint={mint}, dec={dec}, supply={supply}");

    // --- Array (sabit uzunluk) ---
    // Solana'da `[u8; 32]` = Pubkey, `[u8; 64]` = Signature
    let pubkey_bytes: [u8; 32] = [0u8; 32];
    println!("\n--- Array ---");
    println!("pubkey_bytes[0] = {}", pubkey_bytes[0]);
    println!("uzunluk = {}", pubkey_bytes.len());

    // Seeds dizisi (Anchor PDA için):
    let seeds: [&[u8]; 2] = [b"escrow", &[254u8]];
    println!("seeds uzunluk = {}", seeds.len());

    let scores: [u32; 5] = [10, 20, 30, 40, 50];
    let sum: u32 = scores.iter().sum();
    println!("scores toplamı = {sum}");

    // --- Slice ---
    let first_two = &scores[0..2];
    println!("İlk iki skor: {first_two:?}");

    // =========================================================
    // HEAP Tipleri
    // =========================================================

    // String — Anchor'da alan space hesabı: 4 (prefix) + max_len
    let name: String = String::from("Escrow");
    println!("\n--- String ---");
    println!("name = {name}, len = {}", name.len());

    // Vec<u8> — Anchor'da alan space: 4 (prefix) + max_len
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    println!("data = {data:?}, len = {}", data.len());

    // =========================================================
    // TİP DÖNÜŞÜMÜ (Casting)
    // =========================================================
    println!("\n--- Tip Dönüşümü ---");
    let big: u64 = 1_000_000_000;
    let small: u32 = big as u32;        // truncation riski var
    let exact: u64 = u64::from(small);  // güvenli genişleme
    println!("u64 -> u32: {small}, u32 -> u64: {exact}");

    // Edition 2024: `gen` artık keyword, değişken adı olarak kullanılamaz
    // let gen = 5; // HATA — edition 2024'te `gen` rezerve keyword
    let sequence_id: u32 = 5;
    println!("sequence_id: {sequence_id}");
}
