# Session 1 — Anchor için Rust

Anchor-lang ile Solana programı geliştirmek için bilmeniz gereken Rust konuları.
Her konu kendi başına çalışan bir binary dosyasıdır.

**Rust edition: 2024**

---

## Çalıştırma

```bash
# Tek konu:
cargo run --bin 01_variables

# Tümünü sırayla çalıştır:
for bin in 01 02 03 04 05 06 07 08 09 10 11 12 13 14; do
  cargo run --bin $(ls src/bin/ | grep "^${bin}" | sed 's/.rs//')
done
```

---

## Konular

### 01 — Değişkenler (`01_variables.rs`)
`let`, `mut`, shadowing, sabitler (`const`).
Anchor bağlantısı: `u64` lamport miktarları, `u8` bump, `bool` flag alanları.
**Edition 2024 notu:** `gen` artık rezerve keyword — değişken adı olarak kullanılamaz.

```bash
cargo run --bin 01_variables
```

---

### 02 — Veri Tipleri (`02_data_types.rs`)
`u8`, `u64`, `i64`, `bool`, tuple, array, `String`, `Vec<T>`, tip dönüşümü.
Anchor bağlantısı: `[u8; 32]` = Pubkey, `i64` = Unix timestamp, `u64` = lamport.
**Önemli:** Solana on-chain kodda float (`f32`/`f64`) kullanmaktan kaçının.

```bash
cargo run --bin 02_data_types
```

---

### 03 — Fonksiyonlar (`03_functions.rs`)
Fonksiyon tanımı, dönüş değerleri, expression vs statement, early return, özyineleme.
Anchor bağlantısı: `pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>` pattern'i.
**Edition 2024 notu:** `unsafe fn` içindeki unsafe işlemler artık explicit `unsafe {}` bloğu gerektirir.

```bash
cargo run --bin 03_functions
```

---

### 04 — Ownership (`04_ownership.rs`)
Sahiplik kuralları, move semantics, Clone/Copy, scope ve drop.
Anchor bağlantısı: Account verisi context'in sahibidir; instruction tamamlanınca drop edilir.

```bash
cargo run --bin 04_ownership
```

---

### 05 — Borrowing (`05_borrowing.rs`)
Immutable/mutable referanslar, borrow kuralları, slice referansları, dangling reference.
Anchor bağlantısı: `&ctx.accounts.vault` (immutable) ve `&mut ctx.accounts.vault` (mutable).
**Edition 2024 notu:** NLL (Non-Lexical Lifetimes) iyileştirmeleriyle borrow checker daha akıllı.

```bash
cargo run --bin 05_borrowing
```

---

### 06 — Struct'lar (`06_structs.rs`)
Struct tanımı, `impl` blokları, metotlar, associated functions, tuple struct, update syntax.
Anchor bağlantısı: Her `#[account]` bir struct'tır; space hesabı için alan boyutları toplanır.

```bash
cargo run --bin 06_structs
```

---

### 07 — Enum'lar (`07_enums.rs`)
Enum varyantları, `match` pattern matching, `if let`, `while let`, `Option<T>`, `Result<T,E>`.
Anchor bağlantısı: `#[error_code]` enum'ları, instruction state machine'leri.

```bash
cargo run --bin 07_enums
```

---

### 08 — Hata Yönetimi (`08_error_handling.rs`)
`Result<T,E>`, `?` operatörü, özel hata tipleri, `From` trait, `map_err`, `and_then` zinciri.
Anchor bağlantısı: `require!`, `require_eq!`, `require_keys_eq!` macro'larının altında yatan pattern.

```bash
cargo run --bin 08_error_handling
```

---

### 09 — Trait'ler (`09_traits.rs`)
Trait tanımı, default implementasyon, trait bound, `dyn Trait`, `derive` macro'lar, operator overloading.
Anchor bağlantısı: `AccountSerialize`, `AccountDeserialize`, `Space`, `Owner` trait'leri.
**Edition 2024 notu:** `-> impl Trait + use<'a>` ile precise lifetime capture.

```bash
cargo run --bin 09_traits
```

---

### 10 — Generics (`10_generics.rs`)
Generic fonksiyonlar/struct'lar/enum'lar, trait bound, `where` clause, monomorphization.
Anchor bağlantısı: `Account<'info, T>`, `Context<T>`, `Program<'info, T>` generic tipleri.

```bash
cargo run --bin 10_generics
```

---

### 11 — Lifetimes (`11_lifetimes.rs`)
Lifetime annotation, struct'ta referans, `'static`, lifetime elision, çoklu lifetime.
Anchor bağlantısı: `Account<'info, T>`'deki `'info` lifetime'ı tüm account referanslarını kapsar.
**Edition 2024 notu:** `-> impl Trait + use<'a>` (Precise Capturing, RFC 3617).

```bash
cargo run --bin 11_lifetimes
```

---

### 12 — Closure ve Iterator (`12_closures_iterators.rs`)
Closure syntax, capture türleri, `Fn`/`FnMut`/`FnOnce`, iterator adaptörler (`map`, `filter`, `fold`), tüketiciler.
Anchor bağlantısı: Seed doğrulama, hesap miktarı filtreleme, PDA seed byte işlemleri.

```bash
cargo run --bin 12_closures_iterators
```

---

### 13 — Anchor Kalıpları (`13_anchor_patterns.rs`)
Account struct ve space hesabı, instruction context, PDA seed simülasyonu,
`#[error_code]` pattern, CPI (Cross-Program Invocation), events, constraints (`require!`).

```bash
cargo run --bin 13_anchor_patterns
```

---

## Anchor ile Doğrudan İlişki

| Rust Kavramı | Anchor Karşılığı |
|---|---|
| `struct` | `#[account]` — on-chain veri yapısı |
| `enum` | `#[error_code]` — özel hata kodları |
| `impl` metot | `#[program]` instruction handler |
| `Result<T,E>` | `Result<()>` — tüm handler'ların dönüş tipi |
| `?` operatörü | Anchor hata yayılımı |
| Lifetime `'info` | `Account<'info, T>` — runtime account referansı |
| `derive(Debug, Clone)` | `derive(AnchorSerialize, AnchorDeserialize, Clone)` |
| `trait` bound | `AccountSerialize`, `AccountDeserialize`, `Space`, `Owner` |
| `Vec<T>` / `String` | Değişken boyutlu account alanları (space += 4 + max_len) |

---

## Edition 2024 Değişiklikleri

| Değişiklik | Açıklama |
|---|---|
| `gen` keyword rezervasyonu | `gen` değişken/fonksiyon adı olarak artık kullanılamaz |
| `unsafe fn` içinde explicit `unsafe {}` | Edition 2021'de implicit'ti, artık her unsafe işlem için blok gerekli |
| `impl Trait + use<'a>` | Precise Capturing (RFC 3617) — lifetime capture açıkça belirtilir |
| NLL iyileştirmeleri | Borrow checker daha akıllı, bazı geçerli kodlar artık compile olur |

---

## Gereksinimler

- Rust 1.85+ (`rustup update stable`)
- `edition = "2024"` desteği için Rust 1.85 gereklidir

```bash
rustc --version  # 1.85+ olmalı
cargo run --bin 01_variables
```
