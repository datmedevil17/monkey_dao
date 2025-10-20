use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct RateDeal<'info> {
    #[account(mut)]
    pub deal: Account<'info, Deal>,

    #[account(
        init,
        payer = rater,
        space = Rating::LEN,
        seeds = [RATING_SEED, deal.key().as_ref(), rater.key().as_ref()],
        bump
    )]
    pub rating: Account<'info, Rating>,

    #[account(mut)]
    pub rater: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, rater.key().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

pub fn rate_deal(ctx: Context<RateDeal>, rating_value: u8, comment: String) -> Result<()> {
    let deal = &mut ctx.accounts.deal;
    let rating = &mut ctx.accounts.rating;
    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // Validations
    require!(
        rating_value >= MIN_RATING && rating_value <= MAX_RATING,
        DealError::InvalidRating
    );
    require!(
        comment.len() <= MAX_COMMENT_LENGTH,
        DealError::CommentTooLong
    );

    // Initialize rating
    rating.deal = deal.key();
    rating.user = ctx.accounts.rater.key();
    rating.rating = rating_value;
    rating.comment = comment;
    rating.created_at = clock.unix_timestamp;
    rating.is_verified_purchase = deal.times_sold > 0; // Simple check
    rating.bump = ctx.bumps.rating;

    // Update deal rating stats
    deal.total_ratings = deal.total_ratings
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    deal.total_rating_value = deal.total_rating_value
        .checked_add(rating_value as u64)
        .ok_or(DealError::ArithmeticOverflow)?;

    // Update user profile
    user_profile.add_reputation_points(POINTS_RATE_DEAL)?;
    user_profile.update_activity(clock.unix_timestamp);

    emit!(DealRatedEvent {
        deal: deal.key(),
        rater: ctx.accounts.rater.key(),
        rating: rating_value,
        average_rating: deal.average_rating(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct DealRatedEvent {
    pub deal: Pubkey,
    pub rater: Pubkey,
    pub rating: u8,
    pub average_rating: f64,
    pub timestamp: i64,
}