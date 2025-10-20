use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct BuyDeal<'info> {
    #[account(
        mut,
        constraint = !deal.is_used @ DealError::DealAlreadyUsed,
        constraint = !deal.is_expired(Clock::get()?.unix_timestamp) @ DealError::DealExpired,
        constraint = deal.current_supply < deal.max_supply @ DealError::MaxSupplyReached
    )]
    pub deal: Account<'info, Deal>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    /// Previous owner of the deal
    #[account(
        mut,
        constraint = previous_owner.key() == deal.owner
    )]
    /// CHECK: Validated through constraint
    pub previous_owner: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = UserProfile::LEN,
        seeds = [USER_PROFILE_SEED, buyer.key().as_ref()],
        bump
    )]
    pub buyer_profile: Account<'info, UserProfile>,

    #[account(
        mut,
        seeds = [MERCHANT_SEED, deal.merchant.as_ref()],
        bump
    )]
    pub merchant: Account<'info, Merchant>,

    pub system_program: Program<'info, System>,
}

pub fn buy_deal(ctx: Context<BuyDeal>) -> Result<()> {
    let deal = &mut ctx.accounts.deal;
    let buyer_profile = &mut ctx.accounts.buyer_profile;
    let merchant = &mut ctx.accounts.merchant;
    let clock = Clock::get()?;

    // Transfer payment to previous owner
    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.previous_owner.to_account_info(),
        },
    );
    system_program::transfer(transfer_ctx, deal.price)?;

    // Update deal
    let previous_owner = deal.owner;
    deal.owner = ctx.accounts.buyer.key();
    deal.times_sold = deal.times_sold
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    deal.current_supply = deal.current_supply
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;

    // Initialize buyer profile if needed
    if buyer_profile.owner == Pubkey::default() {
        buyer_profile.owner = ctx.accounts.buyer.key();
        buyer_profile.created_at = clock.unix_timestamp;
        buyer_profile.bump = ctx.bumps.buyer_profile;
    }

    // Update buyer profile
    buyer_profile.total_deals_purchased = buyer_profile.total_deals_purchased
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    buyer_profile.add_reputation_points(POINTS_BUY_DEAL)?;
    buyer_profile.update_activity(clock.unix_timestamp);

    // Update merchant stats
    merchant.total_deals_sold = merchant.total_deals_sold
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    merchant.add_revenue(deal.price)?;
    merchant.update_activity(clock.unix_timestamp);

    emit!(DealPurchasedEvent {
        deal: deal.key(),
        buyer: ctx.accounts.buyer.key(),
        previous_owner,
        price: deal.price,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct DealPurchasedEvent {
    pub deal: Pubkey,
    pub buyer: Pubkey,
    pub previous_owner: Pubkey,
    pub price: u64,
    pub timestamp: i64,
}