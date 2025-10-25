use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub nft_mint: Pubkey,
    pub owner: Pubkey,
    pub staked_at: i64,
    pub last_claim: i64,
    pub total_rewards_claimed: u64,
    pub is_active: bool,
    pub bump: u8,
}

impl StakeAccount {
    pub fn calculate_rewards(&self, current_time: i64, reward_rate: u64) -> Result<u64> {
        let time_elapsed = current_time
            .checked_sub(self.last_claim)
            .ok_or(ProgramError::InvalidArgument)?;
        
        if time_elapsed < 0 {
            return Ok(0);
        }

        // reward_rate is MONK tokens per day
        // Convert seconds to days and calculate rewards
        let days_elapsed = time_elapsed as u64 / 86400;
        let rewards = days_elapsed
            .checked_mul(reward_rate)
            .ok_or(ProgramError::InvalidArgument)?;
        
        Ok(rewards)
    }
}