use anchor_lang::prelude::*;
use crate::states::*;

#[derive(Accounts)]
pub struct GetPoolStatus<'info> {
    pub pool: Account<'info, Pool>,
}

pub fn get_pool_status(ctx: Context<GetPoolStatus>) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let clock = Clock::get()?;

    msg!("=== Pool Status ===");
    msg!("Deal: {}", pool.deal);
    msg!("Starter: {}", pool.starter);
    msg!("Target Amount: {} lamports", pool.target_amount);
    msg!("Current Amount: {} lamports", pool.current_amount);
    msg!("Progress: {:.2}%", (pool.current_amount as f64 / pool.target_amount as f64) * 100.0);
    msg!("Target Participants: {}", pool.target_participants);
    msg!("Current Participants: {}", pool.current_participants);
    msg!("Is Active: {}", pool.is_active);
    msg!("Is Executed: {}", pool.is_executed);
    msg!("Is Target Reached: {}", pool.is_target_reached());
    msg!("Is Expired: {}", pool.is_expired(clock.unix_timestamp));
    msg!("Created At: {}", pool.created_at);
    msg!("Expires At: {}", pool.expires_at);
    
    if let Some(executed_at) = pool.executed_at {
        msg!("Executed At: {}", executed_at);
    }

    msg!("=== Participants ===");
    for (i, participant) in pool.participants.iter().enumerate() {
        msg!("  {}: {} - {} lamports", i + 1, participant.user, participant.contribution);
    }

    Ok(())
}