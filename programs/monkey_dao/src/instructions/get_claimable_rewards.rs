use anchor_lang::prelude::*;
use crate::states::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct GetClaimableRewards<'info> {
    #[account(
        seeds = [b"user_profile", user.key().as_ref()],
        bump,
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        seeds = [b"staking_info", user.key().as_ref()],
        bump,
    )]
    pub staking_info: Account<'info, StakingInfo>,
    
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<GetClaimableRewards>) -> Result<u64> {
    let staking_info = &ctx.accounts.staking_info;
    let user_profile = &ctx.accounts.user_profile;
    
    // Check if user has any staked NFTs
    if staking_info.staked_nft_count == 0 {
        msg!("No NFTs staked, no rewards to claim");
        return Ok(0);
    }
    
    let current_time = Clock::get()?.unix_timestamp;
    let time_staked = current_time - staking_info.last_claim_timestamp;
    
    // Calculate base rewards based on time staked
    let base_rewards = (time_staked as u64)
        .checked_mul(staking_info.staked_nft_count)
        .and_then(|x| x.checked_mul(BASE_REWARD_PER_NFT_PER_SECOND))
        .unwrap_or(0);
    
    // Apply reputation multiplier based on badge level
    let reputation_multiplier = match user_profile.current_badge_level {
        BADGE_BRONZE => BRONZE_REWARD_MULTIPLIER,
        BADGE_SILVER => SILVER_REWARD_MULTIPLIER,
        BADGE_GOLD => GOLD_REWARD_MULTIPLIER,
        BADGE_PLATINUM => PLATINUM_REWARD_MULTIPLIER,
        BADGE_DIAMOND => DIAMOND_REWARD_MULTIPLIER,
        _ => 100, // 1.0x multiplier for no badge
    };
    
    let total_rewards = base_rewards
        .checked_mul(reputation_multiplier as u64)
        .and_then(|x| x.checked_div(100))
        .unwrap_or(0);
    
    msg!("=== Claimable Rewards Calculation ===");
    msg!("User: {}", ctx.accounts.user.key());
    msg!("Staked NFTs: {}", staking_info.staked_nft_count);
    msg!("Time since last claim: {} seconds", time_staked);
    msg!("Base rewards: {} lamports", base_rewards);
    msg!("Badge level: {}", user_profile.current_badge_level);
    msg!("Reputation multiplier: {}%", reputation_multiplier);
    msg!("Total claimable rewards: {} lamports", total_rewards);
    
    Ok(total_rewards)
}