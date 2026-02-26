/// # Closure'lar ve Iterator'lar
///
/// Anchor programlarında kullanım alanları:
/// - `seeds.iter()` — PDA seed'lerini dolaşmak
/// - `.map()`, `.filter()`, `.any()`, `.all()` — hesaplama ve doğrulama
/// - Closure'lar `invoke_signed` gibi helper fonksiyonlara geçilebilir
///
/// Edition 2024 notları:
/// - Closure capture semantiği: varsayılan olarak mümkün olan en az kısıtlayıcı şekilde capture
/// - `move` closure davranışı aynı
/// - `gen {}` blokları ve generator sözdizimi eklendi (nightly'den stable'a geçti)
///   Bu dosyada temel closure/iterator gösteriyoruz, gen bloğu ayrıca ele alınıyor

fn main() {
    // =========================================================
    // 1. Temel closure syntax
    // =========================================================
    println!("--- Temel Closure ---");

    // Parametre yok
    let greet = || println!("Merhaba Solana!");
    greet();

    // Parametreli closure
    let double = |x: u64| x * 2;
    println!("double(500_000) = {}", double(500_000));

    // Çok satırlı closure
    let apply_fee = |amount: u64, fee_bps: u64| -> u64 {
        let fee = amount * fee_bps / 10_000;
        amount - fee
    };
    println!("after_fee = {}", apply_fee(1_000_000, 300));

    // =========================================================
    // 2. Closure capture
    // =========================================================
    println!("\n--- Capture ---");

    let fee_bps: u64 = 300;
    // fee_bps borrow edilir (immutable capture)
    let calculate = |amount: u64| amount * fee_bps / 10_000;
    println!("fee = {}", calculate(1_000_000));
    println!("fee_bps hâlâ erişilebilir: {fee_bps}");

    // move closure — thread veya tokio task'e geçerken gerekebilir
    let multiplier: u64 = 10;
    let scale = move |x: u64| x * multiplier; // multiplier move edildi
    println!("scale(100) = {}", scale(100));

    // =========================================================
    // 3. Fn, FnMut, FnOnce trait'leri
    // =========================================================
    println!("\n--- Fn Trait'leri ---");

    // Fn — immutable capture, birden fazla çağrılabilir
    let threshold: u64 = 100_000;
    let is_above = |x: u64| x > threshold;
    println!("500_000 > threshold: {}", is_above(500_000));
    println!("50_000 > threshold: {}", is_above(50_000));

    // FnMut — mutable capture
    let mut count = 0u32;
    let mut increment = || { count += 1; count };
    println!("count: {}", increment());
    println!("count: {}", increment());
    println!("count: {}", increment());

    // FnOnce — değeri consume eder, bir kez çağrılabilir
    let name = String::from("vault");
    let consume = move || {
        println!("tüketen closure: {name}");
        // name burada drop edilir
    };
    consume();
    // consume(); // HATA: FnOnce ikinci kez çağrılamaz

    // =========================================================
    // 4. Iterator — temel kullanım
    // =========================================================
    println!("\n--- Iterator ---");

    let amounts: Vec<u64> = vec![100_000, 500_000, 200_000, 800_000, 300_000];

    // iter() — immutable borrow
    for amount in amounts.iter() {
        print!("{amount} ");
    }
    println!();

    // into_iter() — ownership alır
    let labels = vec!["vault", "escrow", "mint"];
    for label in labels.into_iter() {
        print!("{label} ");
    }
    println!();

    // =========================================================
    // 5. Iterator adaptörler (lazy — consume edilene kadar çalışmaz)
    // =========================================================
    println!("\n--- Iterator Adaptörler ---");

    let amounts2: Vec<u64> = vec![100_000, 500_000, 200_000, 800_000, 300_000];

    // map — her elemanı dönüştür
    let doubled: Vec<u64> = amounts2.iter().map(|&a| a * 2).collect();
    println!("doubled: {doubled:?}");

    // filter — koşulu sağlayanları tut
    let large: Vec<&u64> = amounts2.iter().filter(|&&a| a > 250_000).collect();
    println!("large (>250k): {large:?}");

    // map + filter zinciri
    let net_large: Vec<u64> = amounts2
        .iter()
        .filter(|&&a| a > 250_000)
        .map(|&a| a * 97 / 100) // 3% fee
        .collect();
    println!("net_large (3% fee): {net_large:?}");

    // =========================================================
    // 6. Iterator tüketiciler (consuming)
    // =========================================================
    println!("\n--- Iterator Tüketiciler ---");

    let total: u64 = amounts2.iter().sum();
    println!("sum = {total}");

    let max = amounts2.iter().max();
    println!("max = {max:?}");

    let count = amounts2.iter().filter(|&&a| a > 300_000).count();
    println!("300k üzeri sayı = {count}");

    let any_large = amounts2.iter().any(|&a| a > 700_000);
    println!("700k üzeri var mı: {any_large}");

    let all_positive = amounts2.iter().all(|&a| a > 0);
    println!("hepsi pozitif mi: {all_positive}");

    // fold — accumulator ile
    let sum_fold: u64 = amounts2.iter().fold(0, |acc, &a| acc + a);
    println!("fold sum = {sum_fold}");

    // =========================================================
    // 7. enumerate, zip
    // =========================================================
    println!("\n--- enumerate ve zip ---");

    let seeds = vec!["escrow", "vault", "mint"];
    for (i, seed) in seeds.iter().enumerate() {
        println!("  seeds[{i}] = {seed}");
    }

    let keys = vec!["8xJ3..", "EPjF..", "So11.."];
    let balances = vec![1_000_000u64, 500_000, 200_000];
    for (key, bal) in keys.iter().zip(balances.iter()) {
        println!("  {key} => {bal}");
    }

    // =========================================================
    // 8. flat_map ve chain
    // =========================================================
    println!("\n--- flat_map ve chain ---");

    let nested = vec![vec![1u64, 2, 3], vec![4, 5], vec![6, 7, 8]];
    let flat: Vec<u64> = nested.iter().flat_map(|v| v.iter().copied()).collect();
    println!("flat: {flat:?}");

    let first_part = vec![100u64, 200];
    let second_part = vec![300u64, 400];
    let chained: Vec<u64> = first_part.iter().chain(second_part.iter()).copied().collect();
    println!("chained: {chained:?}");

    // =========================================================
    // 9. Anchor bağlantısı
    // =========================================================
    println!("\n--- Anchor Bağlantısı ---");

    // PDA seed'lerini doğrulama örneği:
    let seeds: Vec<Vec<u8>> = vec![
        b"escrow".to_vec(),
        vec![254u8], // bump
    ];
    let total_seed_len: usize = seeds.iter().map(|s| s.len()).sum();
    println!("total seed bytes = {total_seed_len}");

    // Hesap miktarlarının geçerliliğini kontrol et
    let amounts3 = vec![1_000_000u64, 500_000, 0, 200_000];
    let all_nonzero = amounts3.iter().all(|&a| a > 0);
    println!("all nonzero: {all_nonzero}");

    let valid_amounts: Vec<u64> = amounts3.into_iter().filter(|&a| a > 0).collect();
    println!("valid amounts: {valid_amounts:?}");
}
