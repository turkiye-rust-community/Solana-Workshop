/// # Generics (Jenerikler)
///
/// Anchor'da generics:
/// - `Account<'info, T>` — T herhangi bir account tipi
/// - `Context<T>` — T instruction accounts struct'ı
/// - `Vec<T>`, `Option<T>`, `Result<T, E>` — standart generic tipler
///
/// Edition 2024 notları:
/// - Generic syntax değişmedi
/// - `impl Trait` capture semantiği (09_traits'te ele alındı)
/// - `use<>` syntax ile precise lifetime capture

fn main() {
    // =========================================================
    // 1. Generic fonksiyon
    // =========================================================
    println!("--- Generic Fonksiyon ---");

    println!("max(3u64, 7u64) = {}", max_val(3u64, 7u64));
    println!("max(3i64, -5i64) = {}", max_val(3i64, -5i64));
    println!("max('a', 'z') = {}", max_val('a', 'z'));

    // Anchor'da kullanışlı: overflow-safe toplama
    let result = checked_add(u64::MAX, 1u64);
    println!("checked_add(MAX, 1) = {result:?}");

    let result2 = checked_add(100u64, 200u64);
    println!("checked_add(100, 200) = {result2:?}");

    // =========================================================
    // 2. Generic struct
    // =========================================================
    println!("\n--- Generic Struct ---");

    let vault: Account<VaultState> = Account::new(VaultState { total: 1_000_000, bump: 254 });
    println!("vault.data.total = {}", vault.data.total);
    println!("vault.data.bump = {}", vault.data.bump);
    println!("vault.is_writable = {}", vault.is_writable);

    let token: Account<TokenState> = Account::new(TokenState { amount: 500_000, decimals: 6 });
    println!("token.data.amount = {}", token.data.amount);
    println!("token.data.decimals = {}", token.data.decimals);

    // Generic metot çağrısı
    vault.log("vault hesabı");
    token.log("token hesabı");

    // =========================================================
    // 3. Generic enum
    // =========================================================
    println!("\n--- Generic Enum ---");

    let ok: TxResult<u64> = TxResult::Success(1_000_000);
    let fail: TxResult<u64> = TxResult::Failure("yetersiz bakiye".into());

    ok.print();
    fail.print();

    // =========================================================
    // 4. Birden fazla generic parametre
    // =========================================================
    println!("\n--- Çoklu Generic ---");

    let pair: Pair<u64, bool> = Pair::new(500_000, true);
    println!("amount={}, is_active={}", pair.first, pair.second);
    pair.print();

    let pair2: Pair<&str, u8> = Pair::new("USDC", 6);
    pair2.print();

    // =========================================================
    // 5. where clause — trait bound'ları daha okunabilir
    // =========================================================
    println!("\n--- Where Clause ---");

    let amounts = vec![100u64, 500, 200, 800, 300];
    println!("sum = {}", sum_all(&amounts));
    println!("max = {:?}", find_max(&amounts));

    let labels = vec!["escrow", "vault", "mint"];
    println!("find_max = {:?}", find_max(&labels));

    // =========================================================
    // 6. Monomorphization — zero-cost abstraction
    // =========================================================
    // Rust, generic kodunu derleme zamanında her tip için ayrı üretir.
    // Performans maliyeti yoktur (runtime dispatch YOK, static dispatch).
    println!("\n--- Zero-Cost Abstraction ---");
    let v1 = wrap_value(42u64);
    let v2 = wrap_value("hello");
    let v3 = wrap_value(true);
    println!("v1={v1:?}, v2={v2:?}, v3={v3:?}");
}

// =========================================================
// Generic fonksiyonlar
// =========================================================

fn max_val<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn checked_add<T>(a: T, b: T) -> Option<T>
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialOrd + Copy + Bounded,
{
    if a > T::max_val() - b {
        None
    } else {
        Some(a + b)
    }
}

// =========================================================
// Generic struct'lar
// =========================================================

// Anchor'daki Account<'info, T>'yi basitçe simüle eder
struct Account<T> {
    data: T,
    is_writable: bool,
}

impl<T: std::fmt::Debug> Account<T> {
    fn new(data: T) -> Self {
        Account { data, is_writable: true }
    }

    fn log(&self, label: &str) {
        println!("[{label}] data = {:?}, writable = {}", self.data, self.is_writable);
    }
}

#[derive(Debug)]
struct VaultState {
    total: u64,
    bump: u8,
}

#[derive(Debug)]
struct TokenState {
    amount: u64,
    decimals: u8,
}

// =========================================================
// Generic enum
// =========================================================

enum TxResult<T> {
    Success(T),
    Failure(String),
}

impl<T: std::fmt::Display> TxResult<T> {
    fn print(&self) {
        match self {
            TxResult::Success(v) => println!("[OK] {v}"),
            TxResult::Failure(e) => println!("[ERR] {e}"),
        }
    }
}

// =========================================================
// Çoklu generic
// =========================================================

struct Pair<A, B> {
    first: A,
    second: B,
}

impl<A, B> Pair<A, B> {
    fn new(first: A, second: B) -> Self {
        Pair { first, second }
    }
}

// where clause ile trait bound
impl<A, B> Pair<A, B>
where
    A: std::fmt::Display,
    B: std::fmt::Display,
{
    fn print(&self) {
        println!("({}, {})", self.first, self.second);
    }
}

// =========================================================
// where clause fonksiyonlar
// =========================================================

fn sum_all<T>(items: &[T]) -> T
where
    T: std::iter::Sum + Copy,
{
    items.iter().copied().sum()
}

fn find_max<T>(items: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    items.iter().reduce(|a, b| if b > a { b } else { a })
}

fn wrap_value<T: std::fmt::Debug>(v: T) -> T {
    v
}

// =========================================================
// Yardımcı trait — checked_add için sınır değerler
// =========================================================

trait Bounded {
    fn max_val() -> Self;
}

impl Bounded for u64 {
    fn max_val() -> Self {
        u64::MAX
    }
}

impl Bounded for u32 {
    fn max_val() -> Self {
        u32::MAX
    }
}
