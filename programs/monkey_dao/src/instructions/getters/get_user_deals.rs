use anchor_lang::prelude::*;
use crate::states::*;

#[derive(Accounts)]
pub struct GetUserDeals<'info> {
    pub user_profile: Account<'info, UserProfile>,
}

pub fn get_user_deals(ctx: Context<GetUserDeals>) -> Result<()> {
    let profile = &ctx.accounts.user_profile;

    msg!("=== User Profile ===");
    msg!("Owner: {}", profile.owner);
    msg!("Total Deals Listed: {}", profile.total_deals_listed);
    msg!("Total Deals Purchased: {}", profile.total_deals_purchased);
    msg!("Total Deals Redeemed: {}", profile.total_deals_redeemed);
    msg!("Total Pools Joined: {}", profile.total_pools_joined);
    msg!("Total NFTs Staked: {}", profile.total_nfts_staked);
    msg!("Total Rewards Earned: {} MONK", profile.total_rewards_earned);
    msg!("Reputation Points: {}", profile.reputation_points);
    msg!("Current Badge Level: {}", profile.current_badge_level);
    msg!("Eligible Badge Level: {}", profile.get_eligible_badge_level());
    msg!("Created At: {}", profile.created_at);
    msg!("Last Activity: {}", profile.last_activity_at);

    Ok(())
}