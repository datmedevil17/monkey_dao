use anchor_lang::prelude::*;
use crate::constants::*;
use crate::states::*;

#[derive(Accounts)]
pub struct GetUserReputation<'info> {
    pub user_profile: Account<'info, UserProfile>,
}

pub fn get_user_reputation(ctx: Context<GetUserReputation>) -> Result<()> {
    let profile = &ctx.accounts.user_profile;

    msg!("=== User Reputation ===");
    msg!("User: {}", profile.owner);
    msg!("Total Reputation Points: {}", profile.reputation_points);
    msg!("Current Badge Level: {}", profile.current_badge_level);
    
    let eligible_level = profile.get_eligible_badge_level();
    msg!("Eligible Badge Level: {}", eligible_level);
    
    let badge_name = match eligible_level {
        BADGE_BRONZE => "Bronze Member",
        BADGE_SILVER => "Silver Member",
        BADGE_GOLD => "Gold Member",
        BADGE_PLATINUM => "Platinum Member",
        BADGE_DIAMOND => "Diamond Member",
        _ => "No Badge",
    };
    msg!("Eligible Badge: {}", badge_name);

    // Calculate points needed for next level
    let _next_threshold = if eligible_level == BADGE_DIAMOND {
        msg!("Maximum badge level reached!");
        0
    } else {
        let next = match eligible_level {
            0 => BRONZE_THRESHOLD,
            BADGE_BRONZE => SILVER_THRESHOLD,
            BADGE_SILVER => GOLD_THRESHOLD,
            BADGE_GOLD => PLATINUM_THRESHOLD,
            BADGE_PLATINUM => DIAMOND_THRESHOLD,
            _ => 0,
        };
        let needed = next.saturating_sub(profile.reputation_points);
        msg!("Points needed for next level: {}", needed);
        needed
    };

    msg!("=== Activity Breakdown ===");
    msg!("Deals Listed: {} (+{} points each)", profile.total_deals_listed, POINTS_LIST_DEAL);
    msg!("Deals Purchased: {} (+{} points each)", profile.total_deals_purchased, POINTS_BUY_DEAL);
    msg!("Deals Redeemed: {} (+{} points each)", profile.total_deals_redeemed, POINTS_REDEEM_DEAL);
    msg!("Pools Joined: {} (+{} points each)", profile.total_pools_joined, POINTS_JOIN_POOL);
    msg!("NFTs Staked: {} (+{} points each)", profile.total_nfts_staked, POINTS_STAKE_NFT);

    Ok(())
}