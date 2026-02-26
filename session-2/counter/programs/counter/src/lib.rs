use anchor_lang::prelude::*;

declare_id!("ebRm2oeqhPV9DTonK99FNKh34E1UCsMEX4fh5wgAdMw");

#[program]
pub mod counter {
    use super::*;

    /// Yeni bir sayaç hesabı oluşturur ve başlangıç değerini 0 olarak ayarlar.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = ctx.accounts.authority.key();
        msg!("Counter initialized. Count: {}", counter.count);
        Ok(())
    }

    /// Sayacı 1 artırır.
    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Count: {}", counter.count);
        Ok(())
    }

    /// Sayacı 1 azaltır. 0'ın altına düşemez.
    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(counter.count > 0, CounterError::AlreadyZero);
        counter.count = counter.count.checked_sub(1).unwrap();
        msg!("Counter decremented. Count: {}", counter.count);
        Ok(())
    }

    /// Sayacı 0'a sıfırlar.
    pub fn reset(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter reset to 0");
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Account yapıları
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Counter::SPACE,
        seeds = [b"counter", authority.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [b"counter", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub counter: Account<'info, Counter>,

    pub authority: Signer<'info>,
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[account]
pub struct Counter {
    pub authority: Pubkey, // 32
    pub count: u64,        // 8
}

impl Counter {
    // 8 (discriminator) + 32 (authority) + 8 (count)
    pub const SPACE: usize = 8 + 32 + 8;
}

// ---------------------------------------------------------------------------
// Hatalar
// ---------------------------------------------------------------------------

#[error_code]
pub enum CounterError {
    #[msg("Sayaç zaten 0, daha fazla azaltılamaz.")]
    AlreadyZero,
}
