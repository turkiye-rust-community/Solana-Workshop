/// # Lifetimes (Yaşam Süreleri)
///
/// Lifetime'lar Rust'ın borrow checker'ının referansların geçerliliğini
/// doğrulaması için kullandığı annotation'lardır.
///
/// Anchor'da lifetimes:
/// - `Account<'info, T>` — `'info` tüm account referanslarının yaşam süresi
/// - `Context<'_, '_, 'info, T>` — instruction context'inin lifetime'ları
/// - Struct'lardaki referans alanları lifetime gerektir
///
/// Edition 2024 notları:
/// - `impl Trait` dönüş tiplerinde `use<'a>` syntax eklendi
///   (Precise Capturing: RFC 3617)
/// - `+ '_` yerine `+ use<'_>` kullanılabilir
/// - Lifetime elision kuralları genişletildi (daha az annotation gerekli)

fn main() {
    // =========================================================
    // 1. Lifetime neden gerekli?
    // =========================================================
    println!("--- Lifetime Motivasyonu ---");

    let result;
    {
        let s1 = String::from("uzun string");
        let s2 = String::from("kısa");
        result = longest(s1.as_str(), s2.as_str());
        println!("en uzun: {result}");
    } // s1 ve s2 burada drop, result artık geçersiz — ama kullanmıyoruz

    // =========================================================
    // 2. Lifetime annotation syntax
    // =========================================================
    println!("\n--- Lifetime Annotation ---");

    let s1 = String::from("abcde");
    let s2 = String::from("xy");
    // Her iki string de aynı scope'ta — result güvenli
    let result2 = longest(s1.as_str(), s2.as_str());
    println!("en uzun: {result2}");

    // =========================================================
    // 3. Struct içinde referans — lifetime zorunlu
    // =========================================================
    println!("\n--- Struct Lifetime ---");

    let vault_name = String::from("EscrowVault");
    let config_data = String::from("bump=254,locked=false");

    let account_meta = AccountMeta {
        name: &vault_name,
        config: &config_data,
    };
    println!("meta.name = {}", account_meta.name);
    println!("meta.config = {}", account_meta.config);
    println!("meta.summary = {}", account_meta.summary());

    // =========================================================
    // 4. 'static lifetime — programın tüm ömrü boyunca geçerli
    // =========================================================
    println!("\n--- 'static Lifetime ---");

    // String literal'leri 'static'tir — binary'e gömülü
    let seed: &'static str = "escrow_seed";
    println!("seed = {seed}");

    let msg = static_message();
    println!("static msg = {msg}");

    // =========================================================
    // 5. Lifetime elision — çoğu durumda annotation gerekmiyor
    // =========================================================
    println!("\n--- Lifetime Elision ---");

    // Rust şu kuralları uygular:
    // 1. Her referans parametre kendi lifetime'ını alır
    // 2. Tek input lifetime varsa, output da aynı lifetime'ı alır
    // 3. &self veya &mut self varsa, output self'in lifetime'ını alır

    let data = String::from("program_data");
    let first = first_token(&data);
    println!("first token = {first}");

    // =========================================================
    // 6. Edition 2024: impl Trait + use<'a> (Precise Capturing)
    // =========================================================
    println!("\n--- Edition 2024: Precise Capturing ---");

    let name = String::from("vault");
    // use<'_> ile input lifetime'ı açıkça capture et
    let label = make_label(&name);
    println!("label = {label}");

    // =========================================================
    // 7. Birden fazla lifetime parametresi
    // =========================================================
    println!("\n--- Çoklu Lifetime ---");

    let authority = String::from("8xJ3...");
    let program_id = String::from("EPjF...");
    let ctx = InstructionContext {
        authority: &authority,
        program_id: &program_id,
    };
    let key = ctx.relevant_key();
    println!("relevant key = {key}");
    println!("program_id = {}", ctx.program_id);

    // =========================================================
    // 8. Anchor bağlantısı
    // =========================================================
    println!("\n--- Anchor 'info Lifetime ---");
    // Anchor'da her program şu yapıyı kullanır:
    //
    // #[derive(Accounts)]
    // pub struct Initialize<'info> {
    //     #[account(init, payer = user, space = 8 + EscrowState::INIT_SPACE)]
    //     pub escrow_state: Account<'info, EscrowState>,
    //     #[account(mut)]
    //     pub user: Signer<'info>,
    //     pub system_program: Program<'info, System>,
    // }
    //
    // `'info` tüm account referanslarının Solana runtime'ından geldiğini
    // ve instruction boyunca yaşadığını ifade eder.

    println!("'info lifetime: tüm Account<'info, T> referansları instruction boyunca geçerlidir");
    println!("Bu, Anchor'ın runtime'dan gelen account verilerini güvenle borrow etmesini sağlar");
}

// =========================================================
// Lifetime annotation'lı fonksiyonlar
// =========================================================

// 'a: hem x hem y en az 'a kadar yaşamalı; sonuç da 'a kadar yaşar
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Static mesaj döndürür
fn static_message() -> &'static str {
    "Solana program initialized" // string literal = 'static
}

// Lifetime elision — Rust otomatik çıkarır
fn first_token(data: &str) -> &str {
    data.split('_').next().unwrap_or(data)
}

// =========================================================
// Struct içinde referans
// =========================================================

struct AccountMeta<'a> {
    name: &'a str,
    config: &'a str,
}

impl<'a> AccountMeta<'a> {
    fn summary(&self) -> String {
        format!("{}: {}", self.name, self.config)
    }
}

// =========================================================
// Edition 2024: impl Trait precise capturing
// =========================================================

// `use<'_>` ile input lifetime'ı açıkça capture ediyoruz
// Edition 2021'de `+ '_` kullanılırdı
fn make_label<'a>(name: &'a str) -> impl std::fmt::Display + use<'a> {
    format!("[label: {name}]")
}

// =========================================================
// Çoklu lifetime
// =========================================================

struct InstructionContext<'a, 'b> {
    authority: &'a str,
    program_id: &'b str,
}

impl<'a, 'b> InstructionContext<'a, 'b> {
    // authority'nin lifetime'ını döndürür (daha kısa olanı)
    fn relevant_key(&self) -> &'a str {
        self.authority
    }
}
