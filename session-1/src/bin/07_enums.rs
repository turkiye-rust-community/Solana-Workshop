/// # Enum'lar ve Pattern Matching
///
/// Anchor programlarında enum'lar:
/// - Instruction tipi ayrımı
/// - Hata kodları (`#[error_code]` ile)
/// - State machine durumları
/// - Option<T> ve Result<T,E> — Rust'ın yerleşik enum'ları
///
/// Edition 2024 notları:
/// - `match` ergonomics (matchability) iyileştirildi
/// - Pattern matching semantiği aynı, ancak bazı edge-case'ler daha tutarlı

fn main() {
    // =========================================================
    // 1. Temel Enum
    // =========================================================
    println!("--- Temel Enum ---");

    let state = VaultStatus::Active;
    println!("vault status: {state:?}");

    let locked = VaultStatus::Locked { reason: "dispute", until_slot: 300_000_000 };
    println!("locked: {locked:?}");

    describe_status(&state);
    describe_status(&locked);
    describe_status(&VaultStatus::Closed);

    // =========================================================
    // 2. match — kapsamlı eşleştirme (exhaustive)
    // =========================================================
    println!("\n--- match ---");

    let instruction = Instruction::Deposit { amount: 1_000_000, bump: 254 };
    process_instruction(instruction);

    let instruction2 = Instruction::Withdraw { amount: 500_000 };
    process_instruction(instruction2);

    process_instruction(Instruction::Close);

    // =========================================================
    // 3. if let — tek varyant için kısa syntax
    // =========================================================
    println!("\n--- if let ---");

    let maybe_amount = Some(500_000u64);
    if let Some(amount) = maybe_amount {
        println!("amount = {amount}");
    }

    let nothing: Option<u64> = None;
    if let Some(v) = nothing {
        println!("bu çalışmaz: {v}");
    } else {
        println!("değer yok");
    }

    // =========================================================
    // 4. while let
    // =========================================================
    println!("\n--- while let ---");

    let mut stack = vec![1u64, 2, 3, 4, 5];
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();

    // =========================================================
    // 5. Option<T>
    // =========================================================
    println!("\n--- Option<T> ---");

    let balance: Option<u64> = find_balance("vault_1");
    // unwrap_or — None ise varsayılan değer
    println!("balance = {}", balance.unwrap_or(0));

    let balance2: Option<u64> = find_balance("unknown");
    println!("balance2 = {}", balance2.unwrap_or(0));

    // map — değeri dönüştür, None ise None döner
    let doubled = balance.map(|b| b * 2);
    println!("doubled = {doubled:?}");

    // and_then — zincirleme (flatmap)
    let half = balance.and_then(|b| if b > 0 { Some(b / 2) } else { None });
    println!("half = {half:?}");

    // =========================================================
    // 6. Result<T, E>
    // =========================================================
    println!("\n--- Result<T, E> ---");

    match parse_amount("1000000") {
        Ok(v) => println!("parsed: {v}"),
        Err(e) => println!("hata: {e}"),
    }

    match parse_amount("abc") {
        Ok(v) => println!("parsed: {v}"),
        Err(e) => println!("hata: {e}"),
    }

    // ? operatörü — bir sonraki konuda detaylı gösterilecek
    match run_transaction() {
        Ok(()) => println!("işlem başarılı"),
        Err(e) => println!("işlem hatası: {e}"),
    }

    // =========================================================
    // 7. Anchor hata enum'u örneği
    // =========================================================
    println!("\n--- Anchor Error Enum ---");
    // Anchor'da:
    // #[error_code]
    // pub enum EscrowError {
    //     #[msg("Yetersiz bakiye")]
    //     InsufficientFunds,
    //     #[msg("Hesap kilitli")]
    //     AccountLocked,
    // }
    //
    // Simülasyon:
    let err = EscrowError::InsufficientFunds;
    println!("hata kodu: {} - {}", err.code(), err.message());

    let locked_err = EscrowError::AccountLocked;
    println!("hata kodu: {} - {}", locked_err.code(), locked_err.message());
}

// =========================================================
// Tipler
// =========================================================

#[derive(Debug)]
enum VaultStatus {
    Active,
    Locked { reason: &'static str, until_slot: u64 },
    Closed,
}

#[derive(Debug)]
enum Instruction {
    Deposit { amount: u64, bump: u8 },
    Withdraw { amount: u64 },
    Close,
}

#[derive(Debug)]
enum EscrowError {
    InsufficientFunds,
    AccountLocked,
}

impl EscrowError {
    fn code(&self) -> u32 {
        match self {
            EscrowError::InsufficientFunds => 6000,
            EscrowError::AccountLocked => 6001,
        }
    }
    fn message(&self) -> &str {
        match self {
            EscrowError::InsufficientFunds => "Yetersiz bakiye",
            EscrowError::AccountLocked => "Hesap kilitli",
        }
    }
}

// =========================================================
// Fonksiyonlar
// =========================================================

fn describe_status(status: &VaultStatus) {
    match status {
        VaultStatus::Active => println!("Vault aktif"),
        VaultStatus::Locked { reason, until_slot } => {
            println!("Vault kilitli — sebep: {reason}, slot: {until_slot}")
        }
        VaultStatus::Closed => println!("Vault kapalı"),
    }
}

fn process_instruction(ix: Instruction) {
    match ix {
        Instruction::Deposit { amount, bump } => {
            println!("[deposit] amount={amount}, bump={bump}");
        }
        Instruction::Withdraw { amount } => {
            println!("[withdraw] amount={amount}");
        }
        Instruction::Close => {
            println!("[close] vault kapatıldı");
        }
    }
}

fn find_balance(vault_id: &str) -> Option<u64> {
    match vault_id {
        "vault_1" => Some(5_000_000),
        "vault_2" => Some(3_000_000),
        _ => None,
    }
}

fn parse_amount(s: &str) -> Result<u64, String> {
    s.parse::<u64>().map_err(|e| format!("parse hatası: {e}"))
}

fn run_transaction() -> Result<(), String> {
    let amount = parse_amount("2000000")?; // ? ile erken çıkış
    println!("[transaction] amount={amount}");
    Ok(())
}
