use anchor_lang::prelude::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct ExecutePoolPurchase<'info> {
    #[account(
        mut,
        constraint = pool.is_active @ DealError::PoolNotActive,
        constraint = !pool.is_executed @ DealError::PoolAlreadyExecuted,
        constraint = pool.starter == starter.key() @ DealError::NotPoolStarter,
        constraint = pool.is_target_reached() @ DealError::PoolTargetNotReached
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        constraint = deal.key() == pool.deal
    )]
    pub deal: Account<'info, Deal>,

    #[account(mut)]
    pub starter: Signer<'info>,

    /// Current deal owner (seller)
    #[account(
        mut,
        constraint = deal_owner.key() == deal.owner
    )]
    /// CHECK: Validated through constraint
    pub deal_owner: AccountInfo<'info>,

    /// Pool escrow account
    #[account(
        mut,
        seeds = [crate::constants::ESCROW_SEED, pool.key().as_ref()],
        bump
    )]
    /// CHECK: PDA escrow account
    pub pool_escrow: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [crate::constants::MERCHANT_SEED, deal.merchant.as_ref()],
        bump
    )]
    pub merchant: Account<'info, Merchant>,

    pub system_program: Program<'info, System>,
}

pub fn execute_pool_purchase(ctx: Context<ExecutePoolPurchase>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let deal = &mut ctx.accounts.deal;
    let merchant = &mut ctx.accounts.merchant;
    let clock = Clock::get()?;



    // Transfer pooled funds to deal owner
    let escrow_lamports = ctx.accounts.pool_escrow.lamports();
    **ctx.accounts.pool_escrow.try_borrow_mut_lamports()? = escrow_lamports
        .checked_sub(pool.current_amount)
        .ok_or(DealError::ArithmeticUnderflow)?;
    
    **ctx.accounts.deal_owner.try_borrow_mut_lamports()? = ctx.accounts.deal_owner.lamports()
        .checked_add(pool.current_amount)
        .ok_or(DealError::ArithmeticOverflow)?;

    // Update deal - transfer to pool starter
    deal.owner = pool.starter;
    deal.times_sold = deal.times_sold
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    deal.current_supply = deal.current_supply
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;

    // Update merchant stats
    merchant.total_deals_sold = merchant.total_deals_sold
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    merchant.add_revenue(pool.current_amount)?;
    merchant.update_activity(clock.unix_timestamp);

    // Mark pool as executed
    pool.is_executed = true;
    pool.is_active = false;
    pool.executed_at = Some(clock.unix_timestamp);

    emit!(PoolExecutedEvent {
        pool: pool.key(),
        deal: deal.key(),
        starter: pool.starter,
        amount: pool.current_amount,
        participants: pool.current_participants,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct PoolExecutedEvent {
    pub pool: Pubkey,
    pub deal: Pubkey,
    pub starter: Pubkey,
    pub amount: u64,
    pub participants: u8,
    pub timestamp: i64,
}