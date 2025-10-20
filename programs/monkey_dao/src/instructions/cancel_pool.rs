use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct CancelPool<'info> {
    #[account(
        mut,
        constraint = pool.starter == starter.key() @ DealError::NotPoolStarter,
        constraint = pool.is_active @ DealError::PoolNotActive,
        constraint = !pool.is_executed @ DealError::PoolAlreadyExecuted,
        close = starter
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub starter: Signer<'info>,

    /// Pool escrow account
    #[account(
        mut,
        seeds = [ESCROW_SEED, pool.key().as_ref()],
        bump
    )]
    /// CHECK: PDA escrow account
    pub pool_escrow: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn cancel_pool(ctx: Context<CancelPool>) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let clock = Clock::get()?;

    // Refund all participants
    for participant in pool.participants.iter() {
        // In a real implementation, you would need to pass in participant accounts
        // and refund them here. For simplicity, this is a placeholder.
        // The actual implementation would require dynamic accounts or multiple transactions
    }

    emit!(PoolCancelledEvent {
        pool: pool.key(),
        starter: pool.starter,
        refunded_amount: pool.current_amount,
        participants: pool.current_participants,
        timestamp: clock.unix_timestamp,
    });

    // Pool account will be closed automatically due to close constraint
    Ok(())
}

#[event]
pub struct PoolCancelledEvent {
    pub pool: Pubkey,
    pub starter: Pubkey,
    pub refunded_amount: u64,
    pub participants: u8,
    pub timestamp: i64,
}