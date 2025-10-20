use anchor_lang::prelude::*;

#[account]
pub struct ReputationBadge {
    pub owner: Pubkey,          // 32
    pub badge_level: u8,        // 1
    pub nft_mint: Pubkey,       // 32 - Badge NFT mint
    pub minted_at: i64,         // 8
    pub reputation_at_mint: u64, // 8
    pub bump: u8,               // 1
}

impl ReputationBadge {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        1 +  // badge_level
        32 + // nft_mint
        8 +  // minted_at
        8 +  // reputation_at_mint
        1;   // bump

    pub fn get_badge_name(&self) -> &str {
        use crate::constants::*;
        
        match self.badge_level {
            BADGE_BRONZE => "Bronze Member",
            BADGE_SILVER => "Silver Member",
            BADGE_GOLD => "Gold Member",
            BADGE_PLATINUM => "Platinum Member",
            BADGE_DIAMOND => "Diamond Member",
            _ => "Unknown Badge",
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ActivityType {
    ListDeal,
    BuyDeal,
    RedeemDeal,
    JoinPool,
    StakeNft,
    RateDeal,
}

impl ActivityType {
    pub fn get_reputation_points(&self) -> u64 {
        use crate::constants::*;
        
        match self {
            ActivityType::ListDeal => POINTS_LIST_DEAL,
            ActivityType::BuyDeal => POINTS_BUY_DEAL,
            ActivityType::RedeemDeal => POINTS_REDEEM_DEAL,
            ActivityType::JoinPool => POINTS_JOIN_POOL,
            ActivityType::StakeNft => POINTS_STAKE_NFT,
            ActivityType::RateDeal => POINTS_RATE_DEAL,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        use crate::constants::*;
        
        match value {
            ACTIVITY_LIST_DEAL => Some(ActivityType::ListDeal),
            ACTIVITY_BUY_DEAL => Some(ActivityType::BuyDeal),
            ACTIVITY_REDEEM_DEAL => Some(ActivityType::RedeemDeal),
            ACTIVITY_JOIN_POOL => Some(ActivityType::JoinPool),
            ACTIVITY_STAKE_NFT => Some(ActivityType::StakeNft),
            ACTIVITY_RATE_DEAL => Some(ActivityType::RateDeal),
            _ => None,
        }
    }
}