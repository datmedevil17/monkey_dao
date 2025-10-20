use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount, Mint};
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
#[instruction(badge_level: u8)]
pub struct MintBadge<'info> {
    #[account(
        init,
        payer = user,
        space = ReputationBadge::LEN,
        seeds = [BADGE_SEED, user.key().as_ref(), &[badge_level]],
        bump
    )]
    pub badge: Account<'info, ReputationBadge>,

    /// Badge NFT mint (created by Metaplex on frontend)
    #[account(mut)]
    pub badge_nft_mint: Account<'info, Mint>,

    /// User's badge NFT token account
    #[account(
        init_if_needed,
        payer = user,
        token::mint = badge_nft_mint,
        token::authority = user
    )]
    pub user_badge_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump = user_profile.bump,
        constraint = user_profile.can_mint_badge(badge_level) @ DealError::InsufficientReputationForBadge
    )]
    pub user_profile: Account<'info, UserProfile>,

    /// CHECK: PDA used as mint authority
    #[account(
        seeds = [b"badge_authority"],
        bump
    )]
    pub badge_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn mint_badge(ctx: Context<MintBadge>, badge_level: u8) -> Result<()> {
    let badge = &mut ctx.accounts.badge;
    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // Validate badge level
    require!(
        badge_level >= BADGE_BRONZE && badge_level <= BADGE_DIAMOND,
        DealError::InvalidBadgeLevel
    );

    // Initialize badge
    badge.owner = ctx.accounts.user.key();
    badge.badge_level = badge_level;
    badge.nft_mint = ctx.accounts.badge_nft_mint.key();
    badge.minted_at = clock.unix_timestamp;
    badge.reputation_at_mint = user_profile.reputation_points;
    badge.bump = ctx.bumps.badge;

    // Update user profile
    user_profile.current_badge_level = badge_level;
    user_profile.update_activity(clock.unix_timestamp);

    // Note: The actual NFT minting with Metaplex metadata happens on the frontend
    // This just tracks the badge on-chain

    emit!(BadgeMintedEvent {
        badge: badge.key(),
        user: ctx.accounts.user.key(),
        badge_level,
        badge_name: badge.get_badge_name().to_string(),
        reputation: user_profile.reputation_points,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct BadgeMintedEvent {
    pub badge: Pubkey,
    pub user: Pubkey,
    pub badge_level: u8,
    pub badge_name: String,
    pub reputation: u64,
    pub timestamp: i64,
}