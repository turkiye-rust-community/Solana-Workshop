/// # Trait'ler
///
/// Anchor programlarında trait'ler:
/// - `AccountSerialize`, `AccountDeserialize` — account serialization
/// - `Owner` — program ownership kontrolü
/// - `Space` — account boyutu hesabı
/// - `Key` — pubkey erişimi
/// - `#[derive(AnchorSerialize, AnchorDeserialize, Clone)]` — sık kullanılan derive'lar
///
/// Edition 2024 notları:
/// - `impl Trait` dönüş tiplerinde lifetime capture kuralı değişti:
///   `-> impl Trait` artık tüm generic lifetime'ları capture eder (önceden etmiyordu)
///   Eskiden kaçınmak için `+ '_` yazılırdı, artık varsayılan davranış bu
/// - `dyn Trait` davranışı aynı

use std::fmt;

fn main() {
    // =========================================================
    // 1. Trait tanımlama ve implementasyon
    // =========================================================
    println!("--- Temel Trait ---");

    let vault = VaultAccount { total: 5_000_000, bump: 254 };
    let escrow = EscrowAccount { maker: "Alice", taker: "Bob", amount: 1_000_000 };

    println!("vault size = {}", vault.space());
    println!("escrow size = {}", escrow.space());

    vault.print_info();
    escrow.print_info();

    // =========================================================
    // 2. Default implementasyon
    // =========================================================
    println!("\n--- Default Impl ---");

    let token = TokenAccount { mint: "USDC", amount: 500_000, decimals: 6 };
    token.print_info(); // default implementasyon kullanılır
    println!("is_valid: {}", token.is_valid());

    // =========================================================
    // 3. Trait bound — generic fonksiyonlar
    // =========================================================
    println!("\n--- Trait Bound ---");

    print_space(&vault);
    print_space(&escrow);
    print_space(&token);

    // Birden fazla trait bound
    let items: Vec<&dyn AccountInfo> = vec![&vault, &escrow, &token];
    for item in &items {
        item.print_info();
    }

    // =========================================================
    // 4. impl Trait syntax (edition 2024 değişikliği)
    // =========================================================
    println!("\n--- impl Trait ---");

    // Edition 2024: `-> impl Trait` tüm input lifetime'larını capture eder
    // Bu sayede borrowing daha sezgisel çalışır
    let name = String::from("vault_seed");
    let desc = make_description(&name); // &name lifetime'ı capture edilir
    println!("desc = {desc}");

    // =========================================================
    // 5. derive macro'lar — Anchor'da çok kullanılır
    // =========================================================
    println!("\n--- Derive Macro'lar ---");

    let state = EscrowState {
        maker_amount: 1_000_000,
        taker_amount: 500_000,
        bump: 253,
    };

    // Debug
    println!("{state:?}");

    // Clone
    let state2 = state.clone();
    println!("cloned: maker_amount={}", state2.maker_amount);

    // PartialEq
    let state3 = EscrowState { maker_amount: 1_000_000, taker_amount: 500_000, bump: 253 };
    println!("state == state3: {}", state == state3);

    // =========================================================
    // 6. Display trait
    // =========================================================
    println!("\n--- Display ---");

    let lamport_amount = LamportAmount(1_500_000_000);
    println!("{lamport_amount}");

    // =========================================================
    // 7. Operator overloading — Add
    // =========================================================
    println!("\n--- Operator Overloading ---");

    let a = LamportAmount(500_000_000);
    let b = LamportAmount(500_000_000);
    let sum = a + b;
    println!("sum = {sum}");

    // =========================================================
    // 8. dyn Trait — trait objects
    // =========================================================
    println!("\n--- dyn Trait ---");

    let accounts: Vec<Box<dyn AccountInfo>> = vec![
        Box::new(VaultAccount { total: 0, bump: 252 }),
        Box::new(EscrowAccount { maker: "X", taker: "Y", amount: 100 }),
    ];

    for acc in &accounts {
        acc.print_info();
        println!("  space = {}", acc.space());
    }
}

// =========================================================
// Trait tanımları
// =========================================================

// Anchor'daki Space trait'ini simüle eder
trait AccountInfo {
    fn space(&self) -> usize;
    fn print_info(&self);

    // Default implementasyon
    fn is_valid(&self) -> bool {
        self.space() > 8 // discriminator'dan büyük olmalı
    }
}

struct VaultAccount {
    total: u64, // 8 bytes
    bump: u8,   // 1 byte
}

impl AccountInfo for VaultAccount {
    fn space(&self) -> usize {
        8 + 8 + 1 // discriminator + total + bump
    }

    fn print_info(&self) {
        println!("[VaultAccount] total={}, bump={}", self.total, self.bump);
    }
}

struct EscrowAccount {
    maker: &'static str,
    taker: &'static str,
    amount: u64, // 8 bytes
}

impl AccountInfo for EscrowAccount {
    fn space(&self) -> usize {
        8 + 32 + 32 + 8 // discriminator + maker pubkey + taker pubkey + amount
    }

    fn print_info(&self) {
        println!("[EscrowAccount] maker={}, taker={}, amount={}", self.maker, self.taker, self.amount);
    }
}

struct TokenAccount {
    mint: &'static str,
    amount: u64,
    decimals: u8,
}

impl AccountInfo for TokenAccount {
    fn space(&self) -> usize {
        8 + 32 + 8 + 1 // discriminator + mint + amount + decimals
    }

    fn print_info(&self) {
        println!("[TokenAccount] mint={}, amount={}, decimals={}", self.mint, self.amount, self.decimals);
    }
    // is_valid() — default impl kullanılır
}

// Trait bound fonksiyon
fn print_space<T: AccountInfo>(account: &T) {
    println!("space({}) = {}", std::any::type_name::<T>(), account.space());
}

// Edition 2024: impl Trait lifetime capture
fn make_description<'a>(name: &'a str) -> impl fmt::Display + use<'a> {
    // use<'a> ile lifetime'ı açıkça capture et (edition 2024 syntax)
    format!("Account: {name}")
}

// =========================================================
// Derive macro'lar
// =========================================================

#[derive(Debug, Clone, PartialEq)]
struct EscrowState {
    maker_amount: u64,
    taker_amount: u64,
    bump: u8,
}

// Display trait manuel implementasyon
struct LamportAmount(u64);

impl fmt::Display for LamportAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sol = self.0 as f64 / 1_000_000_000.0;
        write!(f, "{} lamports ({:.3} SOL)", self.0, sol)
    }
}

// Add operator overloading
impl std::ops::Add for LamportAmount {
    type Output = LamportAmount;
    fn add(self, other: LamportAmount) -> LamportAmount {
        LamportAmount(self.0 + other.0)
    }
}
