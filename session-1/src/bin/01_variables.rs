/// # Değişkenler, Mutability, Shadowing ve Sabitler
///
/// Anchor programlarında değişkenler:
/// - Account alanları genellikle `u64`, `i64`, `u8`, `bool` tiplerindedir
/// - `mut` anahtar kelimesi account context'inde sıkça kullanılır
/// - Sabitler discriminator, seed uzunluğu gibi değerler için kullanılır

// Sabit: derleme zamanında bilinen, değişmeyen değer
// Anchor'da genellikle SEED, DISCRIMINATOR gibi değerler için kullanılır
const MAX_LAMPORTS: u64 = 1_000_000_000; // 1 SOL (lamport cinsinden)
const PROGRAM_SEED: &str = "escrow";

fn main() {
    // --- 1. Immutable (değiştirilemez) değişken ---
    // Rust'ta varsayılan olarak tüm değişkenler immutable'dır
    let balance: u64 = 500_000_000; // 0.5 SOL
    println!("Bakiye: {} lamport", balance);

    // Aşağıdaki satır derleme hatası verir:
    // balance = 600_000_000; // error: cannot assign twice to immutable variable

    // --- 2. Mutable (değiştirilebilir) değişken ---
    // `mut` ile işaretlenen değişkenler değiştirilebilir
    // Anchor'da account.amount += deposit gibi işlemlerde kullanılır
    let mut vault_amount: u64 = 0;
    println!("Vault başlangıç: {} lamport", vault_amount);

    vault_amount += 100_000_000;
    println!("Vault sonra: {} lamport", vault_amount);

    // --- 3. Shadowing (gölgeleme) ---
    // Aynı isimde yeni bir değişken tanımlamak, eskisini gölgeler
    // `mut`'tan farkı: tip değiştirilebilir, orijinal değişken değişmez
    let amount = "500000"; // &str
    println!("String amount: {}", amount);

    let amount = amount.parse::<u64>().unwrap(); // u64'e dönüştü
    println!("Parsed amount: {}", amount);

    let amount = amount * 2; // Yeni değer, aynı isim
    println!("Doubled amount: {}", amount);

    // --- 4. Sabitler ---
    println!("Max lamports: {}", MAX_LAMPORTS);
    println!("Program seed: {}", PROGRAM_SEED);

    // --- 5. Tip çıkarımı (type inference) ---
    // Rust, tipi otomatik çıkarabilir
    let is_initialized = false; // bool olarak çıkarıldı
    let slot = 42u64;           // u64 literal suffix ile
    let fee: u32 = 5000;        // explicit tip

    println!("initialized: {}, slot: {}, fee: {}", is_initialized, slot, fee);

    // --- 6. Edition 2024: `gen` artık ayrılmış keyword ---
    // Aşağıdaki kod artık derlenmez:
    // let gen = 5;  // error[E0658]: `gen` is a keyword in edition 2024
    // Bunun yerine farklı bir isim kullanın:
    let generator_id: u64 = 5;
    println!("generator_id: {}", generator_id);

    // --- 7. Anchor bağlantısı ---
    println!("\n--- Anchor'da Kullanılan Tipler ---");
    let lamports: u64 = 2_039_280;    // Rent-exempt minimum
    let bump: u8 = 254;               // PDA bump seed
    let authority_set: bool = true;   // Flag alanı
    println!("lamports={}, bump={}, authority_set={}", lamports, bump, authority_set);
}
