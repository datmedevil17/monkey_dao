use anchor_lang::prelude::*;

#[account]
pub struct Stake {
    pub nft_mint: Pubkey,           // 32
    pub owner: Pubkey,              // 32
    pub staked_at: i64,             // 8
    pub last_claim_at: i64,         // 8
    pub total_rewards_claimed: u64, // 8
    pub is_active: bool,            // 1
    pub bump: u8,                   // 1
}

impl Stake {
    pub const LEN: usize = 8 + // discriminator
        32 + // nft_mint
        32 + // owner
        8 +  // staked_at
        8 +  // last_claim_at
        8 +  // total_rewards_claimed
        1 +  // is_active
        1;   // bump

    pub fn calculate_pending_rewards(&self, current_timestamp: i64, reward_per_day: u64) -> Result<u64> {
        if !self.is_active {
            return Ok(0);
        }

        let time_staked = current_timestamp
            .checked_sub(self.last_claim_at)
            .ok_or(error!(crate::errors::DealError::ArithmeticUnderflow))?;

        if time_staked <= 0 {
            return Ok(0);
        }

        let days_staked = time_staked / crate::constants::SECONDS_PER_DAY;
        let rewards = (days_staked as u64)
            .checked_mul(reward_per_day)
            .ok_or(error!(crate::errors::DealError::ArithmeticOverflow))?;

        Ok(rewards)
    }

    pub fn get_staking_duration_days(&self, current_timestamp: i64) -> i64 {
        (current_timestamp - self.staked_at) / crate::constants::SECONDS_PER_DAY
    }
}