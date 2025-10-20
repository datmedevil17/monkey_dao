use anchor_lang::prelude::*;

#[account]
pub struct StakingInfo {
    pub owner: Pubkey,                 // 32 - The user who owns this staking info
    pub staked_nft_count: u64,         // 8  - Number of NFTs currently staked
    pub total_rewards_claimed: u64,    // 8  - Total rewards claimed over time
    pub last_claim_timestamp: i64,     // 8  - Last time rewards were claimed
    pub total_staking_duration: i64,   // 8  - Total time NFTs have been staked
    pub bump: u8,                      // 1  - PDA bump
}

impl StakingInfo {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        8 +  // staked_nft_count
        8 +  // total_rewards_claimed
        8 +  // last_claim_timestamp
        8 +  // total_staking_duration
        1;   // bump

    pub fn new(owner: Pubkey, bump: u8) -> Self {
        let current_time = Clock::get().unwrap().unix_timestamp;
        
        Self {
            owner,
            staked_nft_count: 0,
            total_rewards_claimed: 0,
            last_claim_timestamp: current_time,
            total_staking_duration: 0,
            bump,
        }
    }

    pub fn add_staked_nft(&mut self) -> Result<()> {
        self.staked_nft_count = self.staked_nft_count
            .checked_add(1)
            .ok_or(error!(crate::errors::DealError::ArithmeticOverflow))?;
        
        // Update last claim timestamp when adding new NFT
        self.last_claim_timestamp = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn remove_staked_nft(&mut self) -> Result<()> {
        if self.staked_nft_count == 0 {
            return Err(error!(crate::errors::DealError::ArithmeticUnderflow));
        }
        
        self.staked_nft_count = self.staked_nft_count
            .checked_sub(1)
            .ok_or(error!(crate::errors::DealError::ArithmeticUnderflow))?;
        
        Ok(())
    }

    pub fn claim_rewards(&mut self, amount: u64) -> Result<()> {
        self.total_rewards_claimed = self.total_rewards_claimed
            .checked_add(amount)
            .ok_or(error!(crate::errors::DealError::ArithmeticOverflow))?;
        
        self.last_claim_timestamp = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn update_staking_duration(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        let duration_since_last_update = current_time
            .checked_sub(self.last_claim_timestamp)
            .ok_or(error!(crate::errors::DealError::ArithmeticUnderflow))?;
        
        self.total_staking_duration = self.total_staking_duration
            .checked_add(duration_since_last_update)
            .ok_or(error!(crate::errors::DealError::ArithmeticOverflow))?;
        
        Ok(())
    }
}
