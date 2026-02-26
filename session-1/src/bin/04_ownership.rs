/// # Ownership (Sahiplik)
///
/// Rust'ın en temel özelliği. Anchor programlarını yazarken ownership
/// kurallarını anlamak kritiktir çünkü account verilerine erişim bu kurallara
/// dayanır.
///
/// Edition 2024 notları:
/// - Ownership kuralları değişmedi, ancak `impl Trait` capture semantiği güçlendi
/// - `Box<T>` ve heap allocation davranışı aynı

fn main() {
    // =========================================================
    // KURAL 1: Her değerin tek bir sahibi (owner) vardır
    // =========================================================
    println!("--- Kural 1: Tek Sahip ---");

    let name = String::from("Escrow"); // name bu String'in sahibi
    println!("name = {name}");

    // =========================================================
    // KURAL 2: Move semantics — sahiplik aktarılabilir
    // =========================================================
    println!("\n--- Kural 2: Move ---");

    let s1 = String::from("vault_data");
    let s2 = s1; // s1'in sahipliği s2'ye taşındı (move)
    // println!("{s1}"); // HATA: s1 artık geçersiz (moved)
    println!("s2 = {s2}"); // sadece s2 geçerli

    // Fonksiyona geçirirken de move olur:
    let data = String::from("account_data");
    takes_ownership(data);
    // println!("{data}"); // HATA: data moved

    // Fonksiyondan değer almak:
    let returned = gives_ownership();
    println!("returned = {returned}");

    // =========================================================
    // KURAL 3: Sahip scope'tan çıkınca değer drop edilir
    // =========================================================
    println!("\n--- Kural 3: Drop ---");
    {
        let scoped = String::from("geçici");
        println!("scope içinde: {scoped}");
    } // scoped burada drop edildi, bellek serbest bırakıldı
    // println!("{scoped}"); // HATA: scope dışı

    // =========================================================
    // COPY tipler — move olmaz, kopyalanır
    // Stack'te saklanan basit tipler: u8, u64, i64, bool, char, f64
    // Anchor account alanlarının büyük çoğunluğu Copy'dir
    // =========================================================
    println!("\n--- Copy Tipler ---");

    let amount: u64 = 1_000_000;
    let amount2 = amount; // kopyalandı, move DEĞİL
    println!("amount={amount}, amount2={amount2}"); // ikisi de geçerli

    let bump: u8 = 254;
    let bump2 = bump;
    println!("bump={bump}, bump2={bump2}");

    let is_locked: bool = false;
    let is_locked2 = is_locked;
    println!("is_locked={is_locked}, is_locked2={is_locked2}");

    // =========================================================
    // CLONE — heap verisini açıkça kopyala
    // Anchor'da çok sık kullanılmaz; gereksiz kopyalamadan kaçının
    // =========================================================
    println!("\n--- Clone ---");

    let original = String::from("pubkey_data");
    let cloned = original.clone(); // derin kopya, her ikisi de geçerli
    println!("original={original}, cloned={cloned}");

    // =========================================================
    // Ownership ve Anchor bağlantısı
    // =========================================================
    println!("\n--- Anchor Bağlantısı ---");

    // Anchor'da account context yapısı:
    // pub struct Deposit<'info> {
    //     pub vault: Account<'info, VaultState>,
    //     pub depositor: Signer<'info>,
    // }
    //
    // `ctx.accounts.vault` — Account<'info, VaultState>'in sahipliği context'tedir
    // `&mut ctx.accounts.vault` — mutable borrow ile değiştirilir
    // Hesaplama tamamlanınca context drop edilir, accounts serbest kalır

    // Simülasyon:
    let vault = VaultState { total: 0, bump: 254 };
    println!("vault oluşturuldu: total={}", vault.total);
    let vault = process_deposit(vault, 500_000);
    println!("deposit sonrası: total={}", vault.total);
} // vault burada drop edilir

// Sahipliği tüketir (consume eder)
fn takes_ownership(s: String) {
    println!("takes_ownership aldı: {s}");
} // s burada drop edilir

// Sahipliği verir
fn gives_ownership() -> String {
    String::from("yeni_veri")
}

// Basit struct (Copy olmayan)
struct VaultState {
    total: u64,
    bump: u8,
}

// Sahipliği alıp geri verir (Anchor'da borrow kullanılır, bu sadece örnek)
fn process_deposit(mut vault: VaultState, amount: u64) -> VaultState {
    vault.total += amount;
    println!("bump={}", vault.bump);
    vault // sahipliği döndür
}
