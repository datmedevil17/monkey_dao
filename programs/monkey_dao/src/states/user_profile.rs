use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub owner: Pubkey,                  // 32
    pub total_deals_listed: u64,        // 8
    pub total_deals_purchased: u64,     // 8
    pub total_deals_redeemed: u64,      // 8
    pub total_pools_joined: u64,        // 8
    pub total_nfts_staked: u64,         // 8
    pub total_rewards_earned: u64,      // 8
    pub reputation_points: u64,         // 8
    pub current_badge_level: u8,        // 1
    pub created_at: i64,                // 8
    pub last_activity_at: i64,          // 8
    pub bump: u8,                       // 1
}

impl UserProfile {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        8 +  // total_deals_listed
        8 +  // total_deals_purchased
        8 +  // total_deals_redeemed
        8 +  // total_pools_joined
        8 +  // total_nfts_staked
        8 +  // total_rewards_earned
        8 +  // reputation_points
        1 +  // current_badge_level
        8 +  // created_at
        8 +  // last_activity_at
        1;   // bump

    pub fn get_eligible_badge_level(&self) -> u8 {
        use crate::constants::*;
        
        if self.reputation_points >= DIAMOND_THRESHOLD {
            BADGE_DIAMOND
        } else if self.reputation_points >= PLATINUM_THRESHOLD {
            BADGE_PLATINUM
        } else if self.reputation_points >= GOLD_THRESHOLD {
            BADGE_GOLD
        } else if self.reputation_points >= SILVER_THRESHOLD {
            BADGE_SILVER
        } else if self.reputation_points >= BRONZE_THRESHOLD {
            BADGE_BRONZE
        } else {
            0
        }
    }

    pub fn can_mint_badge(&self, badge_level: u8) -> bool {
        self.get_eligible_badge_level() >= badge_level && self.current_badge_level < badge_level
    }

    pub fn add_reputation_points(&mut self, points: u64) -> Result<()> {
        self.reputation_points = self.reputation_points
            .checked_add(points)
            .ok_or(error!(crate::errors::DealError::ArithmeticOverflow))?;
        Ok(())
    }

    pub fn update_activity(&mut self, current_timestamp: i64) {
        self.last_activity_at = current_timestamp;
    }
}