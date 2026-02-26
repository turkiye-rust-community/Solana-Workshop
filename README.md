# Solana Workshop

Solana üzerinde program geliştirmeyi öğrenmek için hazırlanmış adım adım workshop serisi.
Her session bağımsız bir klasörde, kendi Cargo/Anchor projesiyle yer alır.

---

## Session'lar

### Session 1 — Anchor için Rust (`session-1/`)

Anchor ile Solana programı yazmadan önce bilmeniz gereken Rust temelleri.
Her konu kendi başına çalışan bir binary dosyasıdır.

**Konular:**

| # | Dosya | İçerik |
|---|-------|---------|
| 01 | `01_variables.rs` | `let`, `mut`, shadowing, `const` |
| 02 | `02_data_types.rs` | Sayısal tipler, tuple, array, `String`, `Vec<T>` |
| 03 | `03_functions.rs` | Fonksiyonlar, dönüş değerleri, early return |
| 04 | `04_ownership.rs` | Ownership kuralları, move semantics, Clone/Copy |
| 05 | `05_borrowing.rs` | Referanslar, borrow kuralları, slice |
| 06 | `06_structs.rs` | Struct tanımı, `impl`, metotlar, update syntax |
| 07 | `07_enums.rs` | Enum, `match`, `Option<T>`, `Result<T,E>` |
| 08 | `08_error_handling.rs` | `Result`, `?` operatörü, özel hata tipleri |
| 09 | `09_traits.rs` | Trait tanımı, default impl, `derive`, operator overloading |
| 10 | `10_generics.rs` | Generic fonksiyonlar/struct'lar, trait bound, `where` |
| 11 | `11_lifetimes.rs` | Lifetime annotation, `'static`, elision |
| 12 | `12_closures_iterators.rs` | Closure, `Fn`/`FnMut`/`FnOnce`, iterator adaptörleri |
| 13 | `13_anchor_patterns.rs` | Account struct, PDA, CPI, events, constraints |

**Çalıştırma:**

```bash
cd session-1
cargo run --bin 01_variables
```

---

### Session 2 — Anchor Program Örnekleri (`session-2/`)

Anchor framework ile yazılmış dört farklı örnek program.
Her örnek kendi Anchor workspace'idir; `programs/` altında program kodu, `tests/` altında TypeScript testleri bulunur.

| Klasör | Açıklama |
|--------|----------|
| `counter/` | Temel sayaç programı — hesap başlatma, artırma, azaltma |
| `custom-error/` | `#[error_code]` ile özel hata tanımlama ve fırlatma |
| `event-emit/` | `emit!` macro ile zincir üzerinde event yayınlama |
| `sol-transfer/` | SOL transferi — system program CPI kullanımı |

**Build:**

```bash
cd session-2/counter
anchor build

cd ../custom-error
anchor build

cd ../event-emit
anchor build

cd ../sol-transfer
anchor build
```

**Test:**

```bash
cd session-2/counter
anchor test
```

---

## Gereksinimler

| Araç | Versiyon |
|------|----------|
| Rust | 1.85+ |
| Solana CLI | 1.18+ |
| Anchor CLI | 0.32.1 |
| Node.js | 18+ |
| Yarn | 1.x |

```bash
rustc --version
solana --version
anchor --version
```
