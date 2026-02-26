/// # Struct'lar
///
/// Anchor program account'ları Rust struct'larıdır.
/// `#[account]` attribute'u ile işaretlenen struct'lar on-chain veri depolar.
///
/// Edition 2024 notları:
/// - Struct tanımı ve impl blokları değişmedi
/// - Field init shorthand, update syntax — aynı
/// - `unsafe` içeren metotlarda explicit unsafe blok gerekebilir

fn main() {
    // =========================================================
    // 1. Temel Struct
    // =========================================================
    println!("--- Temel Struct ---");

    let vault = VaultState {
        total: 1_000_000,
        bump: 254,
        is_locked: false,
        authority: [0u8; 32],
    };
    println!("total={}, bump={}, locked={}", vault.total, vault.bump, vault.is_locked);

    // Alan erişimi
    println!("authority[0]={}", vault.authority[0]);

    // =========================================================
    // 2. Mutable struct
    // =========================================================
    println!("\n--- Mutable Struct ---");

    let mut escrow = EscrowState {
        initializer: [1u8; 32],
        taker: [2u8; 32],
        amount: 500_000,
        bump: 253,
    };
    println!("initializer[0]={}, taker[0]={}, bump={}", escrow.initializer[0], escrow.taker[0], escrow.bump);
    println!("amount öncesi: {}", escrow.amount);
    escrow.amount = 750_000;
    println!("amount sonrası: {}", escrow.amount);

    // =========================================================
    // 3. Struct update syntax
    // =========================================================
    println!("\n--- Update Syntax ---");

    let updated_vault = VaultState {
        total: 2_000_000,
        ..vault // geri kalan alanları vault'tan al
    };
    println!("updated total={}, bump={}", updated_vault.total, updated_vault.bump);

    // =========================================================
    // 4. Tuple struct
    // =========================================================
    println!("\n--- Tuple Struct ---");

    let lamports = Lamports(1_000_000_000);
    let basis_points = BasisPoints(300);
    println!("lamports={}, fee_bps={}", lamports.0, basis_points.0);

    let fee = lamports.fee(basis_points);
    println!("fee = {fee} lamports");

    // =========================================================
    // 5. impl bloğu — metotlar ve associated functions
    // =========================================================
    println!("\n--- impl Metotlar ---");

    // Associated function (constructor)
    let new_vault = VaultState::new([3u8; 32], 252);
    println!("new vault: total={}, bump={}", new_vault.total, new_vault.bump);

    // Metot
    let mut v = VaultState::new([4u8; 32], 251);
    v.deposit(300_000);
    v.deposit(200_000);
    println!("vault deposited: total={}", v.total);

    let withdrawn = v.withdraw(100_000);
    match withdrawn {
        Ok(bal) => println!("withdraw sonrası: {bal}"),
        Err(e) => println!("Hata: {e}"),
    }

    println!("is_rent_exempt: {}", v.is_rent_exempt());

    // =========================================================
    // 6. Debug ve Display
    // =========================================================
    println!("\n--- Debug ---");
    let stats = Stats { deposits: 5, withdrawals: 2, volume: 3_000_000 };
    println!("deposits={}, withdrawals={}, volume={}", stats.deposits, stats.withdrawals, stats.volume);
    println!("{stats:?}");
    println!("{stats:#?}");

    // =========================================================
    // 7. Anchor bağlantısı
    // =========================================================
    println!("\n--- Anchor Account Struct Örneği ---");
    // Anchor'da:
    // #[account]
    // pub struct EscrowState {
    //     pub initializer: Pubkey,   // 32 bytes
    //     pub taker: Pubkey,         // 32 bytes
    //     pub amount: u64,           // 8 bytes
    //     pub bump: u8,              // 1 byte
    // }
    // Space = 8 (discriminator) + 32 + 32 + 8 + 1 = 81 bytes

    let space = 8 + 32 + 32 + 8 + 1;
    println!("EscrowState space = {space} bytes");
    println!("escrow.amount = {}", escrow.amount);
}

// Anchor VaultState simülasyonu
struct VaultState {
    total: u64,
    bump: u8,
    is_locked: bool,
    authority: [u8; 32],
}

impl VaultState {
    // Associated function (static method gibi)
    fn new(authority: [u8; 32], bump: u8) -> Self {
        Self {
            total: 0,
            bump,
            is_locked: false,
            authority,
        }
    }

    // &mut self — mutable metot
    fn deposit(&mut self, amount: u64) {
        self.total += amount;
    }

    fn withdraw(&mut self, amount: u64) -> Result<u64, String> {
        if amount > self.total {
            return Err(format!("Yetersiz: {} < {}", self.total, amount));
        }
        self.total -= amount;
        Ok(self.total)
    }

    // &self — immutable metot
    fn is_rent_exempt(&self) -> bool {
        self.total >= 2_039_280 // min rent-exempt lamports
    }
}

struct EscrowState {
    initializer: [u8; 32],
    taker: [u8; 32],
    amount: u64,
    bump: u8,
}

// Tuple struct'lar — newtype pattern
struct Lamports(u64);
struct BasisPoints(u16);

impl Lamports {
    fn fee(&self, bps: BasisPoints) -> u64 {
        self.0 * u64::from(bps.0) / 10_000
    }
}

// Debug derive — println! ile {:?} kullanımı için
#[derive(Debug)]
struct Stats {
    deposits: u32,
    withdrawals: u32,
    volume: u64,
}
