/// # Anchor-lang Rust Kalıpları
///
/// Bu dosya Anchor programı yazarken kullandığınız Rust pattern'lerini
/// **standart Rust ile simüle ederek** gösterir.
/// Gerçek Anchor kodu `anchor build` ile derlenir; bu dosya sadece
/// `cargo run --bin 14_anchor_patterns` ile çalışır.
///
/// Kapsanan konular:
/// 1. Account struct pattern (Space hesabı)
/// 2. Instruction context pattern
/// 3. PDA (Program Derived Address) seed hesabı
/// 4. Hata yönetimi (#[error_code] simülasyonu)
/// 5. CPI (Cross-Program Invocation) pattern
/// 6. Events
/// 7. Constraints (require! simülasyonu)
///
/// Edition 2024 notları:
/// - `use<'_>` lifetime capture syntax kullanıldı
/// - `gen` keyword rezerve — seed/generator değişken adı olarak kullanılmaz

use std::collections::HashMap;
use std::fmt;

fn main() {
    // =========================================================
    // 1. Account struct ve Space hesabı
    // =========================================================
    println!("=== 1. Account Struct & Space ===");

    // Anchor'da:
    // #[account]
    // pub struct EscrowState {
    //     pub initializer: Pubkey,    // 32
    //     pub taker: Pubkey,          // 32
    //     pub maker_amount: u64,      // 8
    //     pub taker_amount: u64,      // 8
    //     pub bump: u8,               // 1
    // }
    // impl Space for EscrowState { const INIT_SPACE: usize = 32+32+8+8+1; }

    let space = EscrowState::SPACE;
    println!("EscrowState space = {} bytes (+ 8 discriminator = {})", space, space + 8);

    let escrow = EscrowState::new([1u8; 32], [2u8; 32], 1_000_000, 500_000, 254);
    escrow.print();

    // =========================================================
    // 2. Instruction context pattern
    // =========================================================
    println!("\n=== 2. Instruction Context ===");

    // Anchor'da:
    // pub fn make(ctx: Context<Make>, maker_amount: u64, taker_amount: u64) -> Result<()>
    //
    // Context<'_, '_, 'info, Make> şu alanları içerir:
    //   - ctx.accounts: &mut Make<'info>
    //   - ctx.program_id: &Pubkey
    //   - ctx.bumps: BTreeMap<String, u8>

    let mut state = EscrowState::new([0u8; 32], [0u8; 32], 0, 0, 0);
    let result = instruction_make(&mut state, [3u8; 32], [4u8; 32], 2_000_000, 1_000_000, 253);
    match result {
        Ok(()) => println!("make başarılı"),
        Err(e) => println!("make hatası: {e}"),
    }
    state.print();

    // =========================================================
    // 3. PDA seed hesabı
    // =========================================================
    println!("\n=== 3. PDA Seeds ===");

    // Anchor'da:
    // #[account(
    //     seeds = [b"escrow", initializer.key().as_ref()],
    //     bump,
    // )]
    // pub escrow_state: Account<'info, EscrowState>,
    //
    // seeds = [discriminator_seed, pubkey_bytes, bump]

    let initializer_key = [3u8; 32];
    let (pda_bytes, bump) = simulate_find_pda(&[b"escrow", &initializer_key]);
    println!("PDA (simulated) = {:?}...", &pda_bytes[..4]);
    println!("bump = {bump}");

    // Seed validation simülasyonu
    let valid = validate_pda_seeds(
        &pda_bytes,
        &[b"escrow", &initializer_key],
        bump,
    );
    println!("PDA geçerli mi: {valid}");

    // =========================================================
    // 4. Hata yönetimi
    // =========================================================
    println!("\n=== 4. Hata Yönetimi ===");

    // Anchor'da:
    // #[error_code]
    // pub enum EscrowError {
    //     #[msg("Miktar sıfır olamaz")]
    //     ZeroAmount,
    //     #[msg("Hesap kilitli")]
    //     AccountLocked,
    // }

    for amount in [0u64, 100, 1_000_000] {
        match require_nonzero(amount) {
            Ok(()) => println!("amount={amount} geçerli"),
            Err(e) => println!("amount={amount} HATA: {e}"),
        }
    }

    // =========================================================
    // 5. CPI (Cross-Program Invocation) pattern
    // =========================================================
    println!("\n=== 5. CPI Pattern ===");

    // Anchor'da token transfer CPI:
    // let cpi_ctx = CpiContext::new_with_signer(
    //     ctx.accounts.token_program.to_account_info(),
    //     Transfer {
    //         from: ctx.accounts.maker_ata.to_account_info(),
    //         to: ctx.accounts.vault.to_account_info(),
    //         authority: ctx.accounts.initializer.to_account_info(),
    //     },
    //     signer_seeds,
    // );
    // transfer(cpi_ctx, amount)?;

    let mut ledger = TokenLedger::new();
    ledger.mint("maker_ata", 2_000_000);
    ledger.mint("vault", 0);
    ledger.mint("taker_ata", 0);

    println!("CPI öncesi:");
    ledger.print();

    // Maker -> Vault transfer (CPI simülasyonu)
    match ledger.transfer("maker_ata", "vault", 2_000_000) {
        Ok(()) => println!("maker->vault transfer OK"),
        Err(e) => println!("CPI hata: {e}"),
    }

    println!("CPI sonrası:");
    ledger.print();

    // =========================================================
    // 6. Events
    // =========================================================
    println!("\n=== 6. Events ===");

    // Anchor'da:
    // #[event]
    // pub struct EscrowMade {
    //     pub initializer: Pubkey,
    //     pub maker_amount: u64,
    //     pub taker_amount: u64,
    // }
    // emit!(EscrowMade { ... });

    let event = EscrowMadeEvent {
        initializer: [3u8; 32],
        maker_amount: 2_000_000,
        taker_amount: 1_000_000,
    };
    event.emit();

    // =========================================================
    // 7. Constraints pattern (require! macro simülasyonu)
    // =========================================================
    println!("\n=== 7. Constraints ===");

    // Anchor constraint'leri:
    // require!(amount > 0, EscrowError::ZeroAmount)
    // require_eq!(vault.authority, ctx.accounts.user.key(), EscrowError::Unauthorized)
    // require_keys_eq!(mint.key(), expected_mint, EscrowError::InvalidMint)

    let test_cases = vec![
        (1_000_000u64, false, "normal"),
        (0u64, false, "sıfır miktar"),
        (1_000_000u64, true, "kilitli hesap"),
    ];

    for (amount, is_locked, desc) in test_cases {
        match validate_withdraw(amount, is_locked, 5_000_000) {
            Ok(()) => println!("[{desc}] geçti"),
            Err(e) => println!("[{desc}] HATA: {e}"),
        }
    }

    // require_keys_eq! simülasyonu — Unauthorized
    // Anchor'da: require_keys_eq!(ctx.accounts.user.key(), escrow.initializer, EscrowError::Unauthorized)
    let expected_authority = [3u8; 32];
    let caller = [9u8; 32]; // farklı key — yetkisiz
    match validate_authority(&caller, &expected_authority) {
        Ok(()) => println!("[authority] geçti"),
        Err(e) => println!("[authority] HATA: {e}"),
    }
}

// =========================================================
// Account Struct
// =========================================================

struct EscrowState {
    initializer: [u8; 32],
    taker: [u8; 32],
    maker_amount: u64,
    taker_amount: u64,
    bump: u8,
}

impl EscrowState {
    // Space hesabı: Anchor'da InitSpace derive ile otomatik yapılır
    const SPACE: usize = 32 + 32 + 8 + 8 + 1; // = 81

    fn new(
        initializer: [u8; 32],
        taker: [u8; 32],
        maker_amount: u64,
        taker_amount: u64,
        bump: u8,
    ) -> Self {
        Self { initializer, taker, maker_amount, taker_amount, bump }
    }

    fn print(&self) {
        println!(
            "EscrowState {{ maker_amount={}, taker_amount={}, bump={} }}",
            self.maker_amount, self.taker_amount, self.bump
        );
    }
}

// =========================================================
// Instruction handler simülasyonu
// =========================================================

fn instruction_make(
    state: &mut EscrowState,
    initializer: [u8; 32],
    taker: [u8; 32],
    maker_amount: u64,
    taker_amount: u64,
    bump: u8,
) -> Result<(), EscrowError> {
    // require!(maker_amount > 0, EscrowError::ZeroAmount)
    if maker_amount == 0 {
        return Err(EscrowError::ZeroAmount);
    }
    if taker_amount == 0 {
        return Err(EscrowError::ZeroAmount);
    }

    state.initializer = initializer;
    state.taker = taker;
    state.maker_amount = maker_amount;
    state.taker_amount = taker_amount;
    state.bump = bump;

    Ok(())
}

// =========================================================
// PDA simülasyonu
// =========================================================

fn simulate_find_pda(seeds: &[&[u8]]) -> ([u8; 32], u8) {
    // Gerçek Pubkey::find_program_address yerine deterministik simülasyon
    let mut hasher_input = Vec::new();
    for seed in seeds {
        hasher_input.extend_from_slice(seed);
    }
    // Basit hash simülasyonu (gerçek değil)
    let mut result = [0u8; 32];
    for (i, &byte) in hasher_input.iter().enumerate() {
        result[i % 32] ^= byte.wrapping_add(i as u8);
    }
    let bump = 254u8;
    (result, bump)
}

fn validate_pda_seeds(pda: &[u8; 32], seeds: &[&[u8]], bump: u8) -> bool {
    let (expected, expected_bump) = simulate_find_pda(seeds);
    pda == &expected && bump == expected_bump
}

// =========================================================
// Hata tipi
// =========================================================

#[derive(Debug)]
enum EscrowError {
    ZeroAmount,
    AccountLocked,
    InsufficientFunds { balance: u64, requested: u64 },
    Unauthorized,
}

impl fmt::Display for EscrowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EscrowError::ZeroAmount => write!(f, "Miktar sıfır olamaz"),
            EscrowError::AccountLocked => write!(f, "Hesap kilitli"),
            EscrowError::InsufficientFunds { balance, requested } => {
                write!(f, "Yetersiz bakiye: sahip={balance}, istek={requested}")
            }
            EscrowError::Unauthorized => write!(f, "Yetkisiz erişim"),
        }
    }
}

fn require_nonzero(amount: u64) -> Result<(), EscrowError> {
    if amount == 0 {
        return Err(EscrowError::ZeroAmount);
    }
    Ok(())
}

fn validate_authority(caller: &[u8; 32], expected: &[u8; 32]) -> Result<(), EscrowError> {
    if caller != expected {
        return Err(EscrowError::Unauthorized);
    }
    Ok(())
}

fn validate_withdraw(amount: u64, is_locked: bool, balance: u64) -> Result<(), EscrowError> {
    // require!(amount > 0, EscrowError::ZeroAmount)
    if amount == 0 {
        return Err(EscrowError::ZeroAmount);
    }
    // require!(!is_locked, EscrowError::AccountLocked)
    if is_locked {
        return Err(EscrowError::AccountLocked);
    }
    // require!(balance >= amount, EscrowError::InsufficientFunds { ... })
    if balance < amount {
        return Err(EscrowError::InsufficientFunds { balance, requested: amount });
    }
    Ok(())
}

// =========================================================
// CPI token ledger simülasyonu
// =========================================================

struct TokenLedger {
    balances: HashMap<&'static str, u64>,
}

impl TokenLedger {
    fn new() -> Self {
        Self { balances: HashMap::new() }
    }

    fn mint(&mut self, account: &'static str, amount: u64) {
        self.balances.insert(account, amount);
    }

    fn transfer(&mut self, from: &str, to: &'static str, amount: u64) -> Result<(), EscrowError> {
        let from_balance = *self.balances.get(from).unwrap_or(&0);
        if from_balance < amount {
            return Err(EscrowError::InsufficientFunds {
                balance: from_balance,
                requested: amount,
            });
        }
        *self.balances.get_mut(from).unwrap() -= amount;
        *self.balances.entry(to).or_insert(0) += amount;
        Ok(())
    }

    fn print(&self) {
        let mut entries: Vec<_> = self.balances.iter().collect();
        entries.sort_by_key(|&(k, _)| k);
        for (account, balance) in entries {
            println!("  {account}: {balance}");
        }
    }
}

// =========================================================
// Event simülasyonu
// =========================================================

struct EscrowMadeEvent {
    initializer: [u8; 32],
    maker_amount: u64,
    taker_amount: u64,
}

impl EscrowMadeEvent {
    fn emit(&self) {
        println!(
            "EVENT EscrowMade {{ initializer={:?}.., maker_amount={}, taker_amount={} }}",
            &self.initializer[..2],
            self.maker_amount,
            self.taker_amount,
        );
    }
}
