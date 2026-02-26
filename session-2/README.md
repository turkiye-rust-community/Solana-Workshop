# Session 2 — Anchor Program Örnekleri

Anchor framework ile yazılmış dört bağımsız program örneği.
Her örnek kendi Anchor workspace'idir; `programs/` altında Rust kodu, `tests/` altında TypeScript testleri bulunur.

---

## Projeler

### 1. Counter (`counter/`)

PDA tabanlı basit sayaç programı. Anchor'da hesap yaşam döngüsünü ve kısıtlamaları gösterir.

**Account yapısı:**

```
Counter {
    authority: Pubkey,  // 32 byte
    count:     u64,     // 8 byte
}
// Toplam space: 8 (discriminator) + 32 + 8 = 48 byte
```

**PDA seed:** `["counter", authority]`

**Instructions:**

| Instruction | Açıklama |
|-------------|----------|
| `initialize` | Yeni sayaç hesabı oluşturur, count = 0 |
| `increment` | Sayacı 1 artırır (`checked_add`) |
| `decrement` | Sayacı 1 azaltır — count = 0 ise `AlreadyZero` hatası |
| `reset` | Sayacı 0'a sıfırlar |

**Öne çıkan konular:** `init` constraint, `has_one`, `seeds + bump`, `checked_add/sub`, `#[error_code]`

---

### 2. Custom Error (`custom-error/`)

`#[error_code]` ile özel hata tanımlama ve çoklu validasyon senaryolarını gösteren banka hesabı simülasyonu.

**Account yapısı:**

```
BankAccount {
    owner:             Pubkey,  // 32 byte
    balance:           u64,     // 8 byte
    is_active:         bool,    // 1 byte
    is_frozen:         bool,    // 1 byte
    daily_transferred: u64,     // 8 byte
    last_reset_day:    i64,     // 8 byte
}
// Toplam space: 8 + 32 + 8 + 1 + 1 + 8 + 8 = 66 byte
```

**PDA seed:** `["bank", owner]`

**Tanımlı hatalar (`BankError`):**

| Kod | Varyant | Mesaj |
|-----|---------|-------|
| 6000 | `AccountAlreadyActive` | Hesap zaten aktif |
| 6001 | `AccountNotActive` | Hesap aktif değil |
| 6002 | `InsufficientBalance` | Yetersiz bakiye |
| 6003 | `ZeroAmount` | Miktar 0 olamaz |
| 6004 | `DailyLimitExceeded` | Günlük limit aşıldı (1 SOL) |
| 6005 | `AccountFrozen` | Hesap dondurulmuş |
| 6006 | `Unauthorized` | Yetkisiz işlem |
| 6007 | `BelowMinimumBalance` | Minimum bakiye altı (0.00001 SOL) |

**Instructions:** `open_account`, `deposit`, `withdraw`, `freeze_account`, `unfreeze_account`, `close_account`

**Öne çıkan konular:** `#[error_code]`, `require!`, `Clock::get()`, günlük limit takibi, `close = owner` constraint

---

### 3. Event Emit (`event-emit/`)

`#[event]` ve `emit!` macro'su ile zincir üzerinde event yayınlamayı gösteren kullanıcı profili programı.

**Account yapısı:**

```
UserProfile {
    authority:     Pubkey,  // 32 byte
    username:      String,  // 4 + username.len() byte (dinamik)
    points:        u64,     // 8 byte
    message_count: u64,     // 8 byte
}
```

**PDA seed:** `["profile", authority]`

**Tanımlı eventler:**

| Event | Alanlar | Tetikleyen |
|-------|---------|-----------|
| `UserRegistered` | `user`, `username`, `timestamp` | `register_user` |
| `MessageSent` | `sender`, `recipient`, `message`, `timestamp` | `send_message` |
| `PointsAwarded` | `user`, `points`, `reason`, `total_points` | `award_points` |

**Instructions:** `register_user`, `send_message`, `award_points`

**Öne çıkan konular:** `#[event]`, `emit!`, `Clock::get()`, dinamik `space` hesabı (`#[instruction]`), event indexing

---

### 4. SOL Transfer (`sol-transfer/`)

System Program CPI kullanarak SOL transferini ve PDA imzalı (signer seeds) transferi gösteren vault programı.

**Account yapısı:**

```
Vault {
    authority:       Pubkey,  // 32 byte
    total_deposited: u64,     // 8 byte
}
// Toplam space: 8 + 32 + 8 = 48 byte
```

**PDA seed:** `["vault", authority]`

**Instructions:**

| Instruction | Açıklama | CPI Türü |
|-------------|----------|----------|
| `transfer_sol` | Gönderenden alıcıya direkt transfer | `CpiContext::new` |
| `initialize_vault` | PDA vault hesabı oluşturur | — |
| `deposit_to_vault` | Kullanıcıdan vault'a SOL yatırır | `CpiContext::new` |
| `withdraw_from_vault` | Vault'tan authority'ye SOL çeker | `CpiContext::new_with_signer` |

**Öne çıkan konular:** `system_program::transfer`, `CpiContext::new`, `CpiContext::new_with_signer`, `signer_seeds`, `ctx.bumps`

---

## Build & Test

```bash
# Tek proje build
cd counter
anchor build

# Tüm projeleri build et
for p in counter custom-error event-emit sol-transfer; do
  echo "--- $p ---"
  cd /path/to/session-2/$p && anchor build
done

# Test çalıştırma
cd counter
anchor test
```

---

## Anchor Kavramları — Özet

| Kavram | Kullanıldığı Proje |
|--------|--------------------|
| `init` + `seeds` + `bump` (PDA hesap) | counter, custom-error, event-emit, sol-transfer |
| `has_one` constraint | counter, custom-error, event-emit, sol-transfer |
| `close = owner` (hesap kapatma) | custom-error |
| `#[error_code]` + `require!` | counter, custom-error, event-emit, sol-transfer |
| `Clock::get()` (zaman damgası) | custom-error, event-emit |
| `#[event]` + `emit!` | event-emit |
| `CpiContext::new` (System Program CPI) | sol-transfer |
| `CpiContext::new_with_signer` (PDA imzalı CPI) | sol-transfer |
| Dinamik `space` hesabı | event-emit |
