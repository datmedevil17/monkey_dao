use anchor_lang::prelude::*;
use crate::state::*;
use crate:: ANCHOR_DISCRIMINATOR;
use crate::error::ErrorCode;
use crate::MAX_RATING;

#[derive(Accounts)]
pub struct AddReview<'info> {
    #[account(mut)]
    pub reviewer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"listing", listing.nft_mint.as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(
        init,
        payer = reviewer,
        space = ANCHOR_DISCRIMINATOR + Review::INIT_SPACE,
        seeds = [b"review", listing.key().as_ref(), reviewer.key().as_ref()],
        bump
    )]
    pub review: Account<'info, Review>,
    
    pub system_program: Program<'info, System>,
}

pub fn add_review(
    ctx: Context<AddReview>,
    rating: u8,
    comment: String,
) -> Result<()> {
    require!(rating >= 1 && rating <= MAX_RATING, ErrorCode::InvalidRating);
    require!(comment.len() <= 500, ErrorCode::InvalidRating);
    
    let clock = Clock::get()?;
    let review = &mut ctx.accounts.review;
    
    review.listing = ctx.accounts.listing.key();
    review.reviewer = ctx.accounts.reviewer.key();
    review.rating = rating;
    review.comment = comment;
    review.created_at = clock.unix_timestamp;
    review.bump = ctx.bumps.review;

    // Update listing average rating
    let listing = &mut ctx.accounts.listing;
    let total_reviews = listing.total_reviews;
    let current_total = (listing.average_rating as u64) * total_reviews;
    let new_total = current_total + (rating as u64 * 20); // Convert 1-5 to 20-100
    let new_count = total_reviews.checked_add(1)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    
    listing.average_rating = (new_total / new_count) as u8;
    listing.total_reviews = new_count;

    msg!("Review added with rating: {}/5", rating);
    msg!("New average rating: {}", listing.average_rating as f32 / 20.0);
    
    Ok(())
}