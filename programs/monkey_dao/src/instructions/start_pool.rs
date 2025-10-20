use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct StartPool<'info> {
    #[account(
        constraint = deal.is_group_deal @ DealError::NotGroupDeal,
        constraint = !deal.is_used @ DealError::DealAlreadyUsed,
        constraint = !deal.is_expired(Clock::get()?.unix_timestamp) @ DealError::DealExpired
    )]
    pub deal: Account<'info, Deal>,

    #[account(
        init,
        payer = starter,
        space = Pool::LEN,
        seeds = [POOL_SEED, deal.key().as_ref(), starter.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub starter: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, starter.key().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

pub fn start_pool(
    ctx: Context<StartPool>,
    target_amount: u64,
    target_participants: u8,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let deal = &ctx.accounts.deal;
    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // Validations
    require!(
        target_participants >= MIN_POOL_PARTICIPANTS && 
        target_participants <= MAX_POOL_PARTICIPANTS,
        DealError::InvalidPoolParticipants
    );

    // Ensure target amount matches a group price
    let group_price = deal.get_group_price(target_participants)
        .ok_or(DealError::InvalidPoolParticipants)?;
    require!(target_amount == group_price, DealError::InvalidPrice);

    // Initialize pool
    pool.deal = deal.key();
    pool.starter = ctx.accounts.starter.key();
    pool.target_amount = target_amount;
    pool.current_amount = 0;
    pool.target_participants = target_participants;
    pool.current_participants = 0;
    pool.participants = Vec::new();
    pool.is_active = true;
    pool.is_executed = false;
    pool.created_at = clock.unix_timestamp;
    pool.expires_at = clock.unix_timestamp
        .checked_add(POOL_EXPIRY_SECONDS)
        .ok_or(DealError::ArithmeticOverflow)?;
    pool.executed_at = None;
    pool.bump = ctx.bumps.pool;

    // Update user profile
    user_profile.update_activity(clock.unix_timestamp);

    emit!(PoolStartedEvent {
        pool: pool.key(),
        deal: deal.key(),
        starter: ctx.accounts.starter.key(),
        target_amount,
        target_participants,
        expires_at: pool.expires_at,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct PoolStartedEvent {
    pub pool: Pubkey,
    pub deal: Pubkey,
    pub starter: Pubkey,
    pub target_amount: u64,
    pub target_participants: u8,
    pub expires_at: i64,
    pub timestamp: i64,
}