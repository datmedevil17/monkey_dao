use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct UpdateReputation<'info> {
    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub user: Signer<'info>,
}

pub fn update_reputation(ctx: Context<UpdateReputation>, activity_type: u8) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // Convert activity type to enum
    let activity = ActivityType::from_u8(activity_type)
        .ok_or(DealError::InvalidActivityType)?;

    // Get reputation points for activity
    let points = activity.get_reputation_points();

    // Add reputation points
    user_profile.add_reputation_points(points)?;
    user_profile.update_activity(clock.unix_timestamp);

    // Check if user is eligible for a new badge
    let eligible_badge = user_profile.get_eligible_badge_level();

    emit!(ReputationUpdatedEvent {
        user: ctx.accounts.user.key(),
        activity_type,
        points_earned: points,
        total_reputation: user_profile.reputation_points,
        eligible_badge_level: eligible_badge,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct ReputationUpdatedEvent {
    pub user: Pubkey,
    pub activity_type: u8,
    pub points_earned: u64,
    pub total_reputation: u64,
    pub eligible_badge_level: u8,
    pub timestamp: i64,
}