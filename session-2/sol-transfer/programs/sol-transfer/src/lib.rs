use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("3hPBfAvueR7SskEDgd29f5NFbPocDvdXP2RqQuV3CGXa");

#[program]
pub mod sol_transfer {
    use super::*;

    /// Gönderenden alıcıya `amount` kadar lamport transfer eder.
    /// CPI: Bu program -> System Program -> transfer instruction
    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        require!(amount > 0, TransferError::ZeroAmount);
        require!(
            ctx.accounts.sender.lamports() >= amount,
            TransferError::InsufficientFunds
        );

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.sender.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        );
        system_program::transfer(cpi_ctx, amount)?;

        msg!(
            "{} lamport transfer edildi: {} -> {}",
            amount,
            ctx.accounts.sender.key(),
            ctx.accounts.recipient.key()
        );
        Ok(())
    }

    /// Program'ın PDA vault'unu başlatır.
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.total_deposited = 0;
        msg!("Vault başlatıldı: {}", vault.key());
        Ok(())
    }

    /// Kullanıcı vault'a SOL yatırır (CPI).
    pub fn deposit_to_vault(ctx: Context<DepositToVault>, amount: u64) -> Result<()> {
        require!(amount > 0, TransferError::ZeroAmount);

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.depositor.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );
        system_program::transfer(cpi_ctx, amount)?;

        ctx.accounts.vault.total_deposited = ctx
            .accounts
            .vault
            .total_deposited
            .checked_add(amount)
            .unwrap();

        msg!("Vault'a {} lamport yatırıldı", amount);
        Ok(())
    }

    /// Authority vault'tan SOL çeker (PDA imzalı CPI).
    pub fn withdraw_from_vault(ctx: Context<WithdrawFromVault>, amount: u64) -> Result<()> {
        require!(amount > 0, TransferError::ZeroAmount);
        require!(
            ctx.accounts.vault.to_account_info().lamports() >= amount,
            TransferError::InsufficientFunds
        );

        let authority_key = ctx.accounts.authority.key();
        let seeds = &[
            b"vault",
            authority_key.as_ref(),
            &[ctx.bumps.vault],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.authority.to_account_info(),
            },
            signer_seeds,
        );
        system_program::transfer(cpi_ctx, amount)?;

        msg!("Vault'tan {} lamport çekildi", amount);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Account yapıları
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: Alıcı hesap - herhangi bir hesap olabilir
    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = Vault::SPACE,
        seeds = [b"vault", authority.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositToVault<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.authority.as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[account]
pub struct Vault {
    pub authority: Pubkey,       // 32
    pub total_deposited: u64,    // 8
}

impl Vault {
    // 8 (discriminator) + 32 (authority) + 8 (total_deposited)
    pub const SPACE: usize = 8 + 32 + 8;
}

// ---------------------------------------------------------------------------
// Hatalar
// ---------------------------------------------------------------------------

#[error_code]
pub enum TransferError {
    #[msg("Transfer miktarı 0 olamaz.")]
    ZeroAmount,
    #[msg("Yetersiz bakiye.")]
    InsufficientFunds,
}
