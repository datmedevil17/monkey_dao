use anchor_lang::prelude::*;

pub mod merchant;
pub mod listing;
pub mod pool;
pub mod review;
pub mod staking;

pub use merchant::*;
pub use listing::*;
pub use pool::*;
pub use review::*;
pub use staking::*;

#[account]
#[derive(InitSpace)]
pub struct PlatformConfig {
    pub authority: Pubkey,
    pub monk_mint: Pubkey,
    pub platform_wallet: Pubkey,
    pub platform_fee_bps: u64,
    pub staking_reward_rate: u64, // MONK tokens per day per NFT
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserStats {
    pub user: Pubkey,
    pub total_purchases: u64,
    pub total_monk_earned: u64,
    pub nfts_staked: u64,
    pub bump: u8,
}