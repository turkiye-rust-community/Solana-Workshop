use anchor_lang::prelude::*;

declare_id!("7KU5Q7YaEgbu5fu6ohfUaxY7wm78r4iBuFjiqNKSsTUc");

// ---------------------------------------------------------------------------
// Özel hata kodları
// ---------------------------------------------------------------------------

/// #[error_code] ile tanımlanan custom error'lar.
/// Her varyant otomatik olarak 6000'den başlayan bir error code alır.
#[error_code]
pub enum BankError {
    // 6000
    #[msg("Hesap zaten aktif durumda.")]
    AccountAlreadyActive,

    // 6001
    #[msg("Hesap aktif değil. Önce hesabı aktifleştirin.")]
    AccountNotActive,

    // 6002
    #[msg("Yetersiz bakiye. Minimum bakiye koruma kuralı nedeniyle çekim yapılamıyor.")]
    InsufficientBalance,

    // 6003
    #[msg("Transfer miktarı 0 olamaz.")]
    ZeroAmount,

    // 6004
    #[msg("Günlük transfer limiti aşıldı.")]
    DailyLimitExceeded,

    // 6005
    #[msg("Hesap dondurulmuş durumda.")]
    AccountFrozen,

    // 6006
    #[msg("Bu işlemi yapmaya yetkiniz yok.")]
    Unauthorized,

    // 6007
    #[msg("Minimum bakiye altına düşülemez.")]
    BelowMinimumBalance,
}

// ---------------------------------------------------------------------------
// Program
// ---------------------------------------------------------------------------

#[program]
pub mod custom_error {
    use super::*;

    pub const DAILY_LIMIT: u64 = 1_000_000_000; // 1 SOL (lamport)
    pub const MINIMUM_BALANCE: u64 = 10_000;    // 0.00001 SOL (lamport)

    /// Banka hesabını başlatır.
    pub fn open_account(ctx: Context<OpenAccount>, initial_deposit: u64) -> Result<()> {
        require!(initial_deposit >= MINIMUM_BALANCE, BankError::BelowMinimumBalance);

        let account = &mut ctx.accounts.bank_account;
        account.owner = ctx.accounts.owner.key();
        account.balance = initial_deposit;
        account.is_active = true;
        account.is_frozen = false;
        account.daily_transferred = 0;
        account.last_reset_day = Clock::get()?.unix_timestamp / 86400;

        msg!("Hesap açıldı. Bakiye: {} lamport", initial_deposit);
        Ok(())
    }

    /// Para yatırır.
    pub fn deposit(ctx: Context<BankOp>, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.bank_account;

        // Validasyonlar - her biri farklı bir hata fırlatır
        require!(amount > 0, BankError::ZeroAmount);
        require!(account.is_active, BankError::AccountNotActive);
        require!(!account.is_frozen, BankError::AccountFrozen);

        account.balance = account.balance.checked_add(amount).unwrap();
        msg!("Yatırıldı: {} lamport. Yeni bakiye: {}", amount, account.balance);
        Ok(())
    }

    /// Para çeker - birden fazla hata senaryosu gösterir.
    pub fn withdraw(ctx: Context<BankOp>, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.bank_account;

        require!(amount > 0, BankError::ZeroAmount);
        require!(account.is_active, BankError::AccountNotActive);
        require!(!account.is_frozen, BankError::AccountFrozen);

        // Günlük limit sıfırlama
        let today = Clock::get()?.unix_timestamp / 86400;
        if today > account.last_reset_day {
            account.daily_transferred = 0;
            account.last_reset_day = today;
        }

        // Günlük limit kontrolü
        require!(
            account.daily_transferred.checked_add(amount).unwrap() <= DAILY_LIMIT,
            BankError::DailyLimitExceeded
        );

        // Bakiye kontrolü
        require!(
            account.balance >= amount + MINIMUM_BALANCE,
            BankError::InsufficientBalance
        );

        account.balance = account.balance.checked_sub(amount).unwrap();
        account.daily_transferred = account.daily_transferred.checked_add(amount).unwrap();

        msg!("Çekildi: {} lamport. Yeni bakiye: {}", amount, account.balance);
        Ok(())
    }

    /// Hesabı dondurur (sadece owner yapabilir).
    pub fn freeze_account(ctx: Context<BankOp>) -> Result<()> {
        let account = &mut ctx.accounts.bank_account;

        require!(account.is_active, BankError::AccountNotActive);
        require!(!account.is_frozen, BankError::AccountFrozen);

        account.is_frozen = true;
        msg!("Hesap donduruldu.");
        Ok(())
    }

    /// Hesabı çözer.
    pub fn unfreeze_account(ctx: Context<BankOp>) -> Result<()> {
        let account = &mut ctx.accounts.bank_account;

        require!(account.is_active, BankError::AccountNotActive);
        require!(account.is_frozen, BankError::AccountFrozen);

        account.is_frozen = false;
        msg!("Hesap dondurma kaldırıldı.");
        Ok(())
    }

    /// Hesabı kapatır (sadece owner yapabilir, hesap aktif olmalı).
    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        let account = &ctx.accounts.bank_account;

        require!(account.is_active, BankError::AccountNotActive);
        require!(!account.is_frozen, BankError::AccountFrozen);

        msg!("Hesap kapatıldı. Son bakiye: {}", account.balance);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Account yapıları
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct OpenAccount<'info> {
    #[account(
        init,
        payer = owner,
        space = BankAccount::SPACE,
        seeds = [b"bank", owner.key().as_ref()],
        bump,
    )]
    pub bank_account: Account<'info, BankAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BankOp<'info> {
    #[account(
        mut,
        seeds = [b"bank", owner.key().as_ref()],
        bump,
        has_one = owner,
    )]
    pub bank_account: Account<'info, BankAccount>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(
        mut,
        seeds = [b"bank", owner.key().as_ref()],
        bump,
        has_one = owner,
        close = owner,
    )]
    pub bank_account: Account<'info, BankAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[account]
pub struct BankAccount {
    pub owner: Pubkey,           // 32
    pub balance: u64,            // 8
    pub is_active: bool,         // 1
    pub is_frozen: bool,         // 1
    pub daily_transferred: u64,  // 8
    pub last_reset_day: i64,     // 8
}

impl BankAccount {
    // 8 (discriminator) + 32 + 8 + 1 + 1 + 8 + 8
    pub const SPACE: usize = 8 + 32 + 8 + 1 + 1 + 8 + 8;
}
