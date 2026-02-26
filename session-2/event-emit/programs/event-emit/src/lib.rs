use anchor_lang::prelude::*;

declare_id!("YHSeQSEvdfXy4cRP7oPhgV5pqTLsZL375CVUMoafiqm");

// ---------------------------------------------------------------------------
// Event tanımları
// ---------------------------------------------------------------------------

/// Kullanıcı kaydedildiğinde fırlatılan event.
#[event]
pub struct UserRegistered {
    pub user: Pubkey,
    pub username: String,
    pub timestamp: i64,
}

/// Mesaj gönderildiğinde fırlatılan event.
#[event]
pub struct MessageSent {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub message: String,
    pub timestamp: i64,
}

/// Puan verildiğinde fırlatılan event.
#[event]
pub struct PointsAwarded {
    pub user: Pubkey,
    pub points: u64,
    pub reason: String,
    pub total_points: u64,
}

// ---------------------------------------------------------------------------
// Program
// ---------------------------------------------------------------------------

#[program]
pub mod event_emit {
    use super::*;

    /// Yeni kullanıcı kaydeder ve UserRegistered event'i fırlatır.
    pub fn register_user(ctx: Context<RegisterUser>, username: String) -> Result<()> {
        require!(username.len() > 0, EventError::EmptyUsername);
        require!(username.len() <= 32, EventError::UsernameTooLong);

        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.authority.key();
        profile.username = username.clone();
        profile.points = 0;
        profile.message_count = 0;

        let clock = Clock::get()?;

        // UserRegistered event'ini fırlat
        emit!(UserRegistered {
            user: ctx.accounts.authority.key(),
            username,
            timestamp: clock.unix_timestamp,
        });

        msg!("Kullanıcı kaydedildi ve event fırlatıldı");
        Ok(())
    }

    /// Mesaj gönderir ve MessageSent event'i fırlatır.
    pub fn send_message(
        ctx: Context<SendMessage>,
        recipient: Pubkey,
        message: String,
    ) -> Result<()> {
        require!(message.len() > 0, EventError::EmptyMessage);
        require!(message.len() <= 256, EventError::MessageTooLong);

        let profile = &mut ctx.accounts.profile;
        profile.message_count = profile.message_count.checked_add(1).unwrap();

        let clock = Clock::get()?;

        // MessageSent event'ini fırlat
        emit!(MessageSent {
            sender: ctx.accounts.authority.key(),
            recipient,
            message,
            timestamp: clock.unix_timestamp,
        });

        msg!("Mesaj gönderildi ve event fırlatıldı");
        Ok(())
    }

    /// Kullanıcıya puan verir ve PointsAwarded event'i fırlatır.
    pub fn award_points(ctx: Context<AwardPoints>, points: u64, reason: String) -> Result<()> {
        require!(points > 0, EventError::ZeroPoints);
        require!(reason.len() > 0, EventError::EmptyReason);

        let profile = &mut ctx.accounts.profile;
        profile.points = profile.points.checked_add(points).unwrap();

        // PointsAwarded event'ini fırlat
        emit!(PointsAwarded {
            user: ctx.accounts.authority.key(),
            points,
            reason,
            total_points: profile.points,
        });

        msg!("Puan verildi. Toplam: {}", profile.points);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Account yapıları
// ---------------------------------------------------------------------------

#[derive(Accounts)]
#[instruction(username: String)]
pub struct RegisterUser<'info> {
    #[account(
        init,
        payer = authority,
        space = UserProfile::space(&username),
        seeds = [b"profile", authority.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendMessage<'info> {
    #[account(
        mut,
        seeds = [b"profile", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub profile: Account<'info, UserProfile>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AwardPoints<'info> {
    #[account(
        mut,
        seeds = [b"profile", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub profile: Account<'info, UserProfile>,

    pub authority: Signer<'info>,
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[account]
pub struct UserProfile {
    pub authority: Pubkey,      // 32
    pub username: String,       // 4 + max 32
    pub points: u64,            // 8
    pub message_count: u64,     // 8
}

impl UserProfile {
    pub fn space(username: &str) -> usize {
        8              // discriminator
        + 32           // authority
        + 4 + username.len() // username string
        + 8            // points
        + 8            // message_count
    }
}

// ---------------------------------------------------------------------------
// Hatalar
// ---------------------------------------------------------------------------

#[error_code]
pub enum EventError {
    #[msg("Kullanıcı adı boş olamaz.")]
    EmptyUsername,
    #[msg("Kullanıcı adı 32 karakterden uzun olamaz.")]
    UsernameTooLong,
    #[msg("Mesaj boş olamaz.")]
    EmptyMessage,
    #[msg("Mesaj 256 karakterden uzun olamaz.")]
    MessageTooLong,
    #[msg("Puan miktarı 0 olamaz.")]
    ZeroPoints,
    #[msg("Sebep boş olamaz.")]
    EmptyReason,
}
