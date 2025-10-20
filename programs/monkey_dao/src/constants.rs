// PDA Seeds
pub const DEAL_SEED: &[u8] = b"deal";
pub const USER_PROFILE_SEED: &[u8] = b"user_profile";
pub const POOL_SEED: &[u8] = b"pool";
pub const STAKE_SEED: &[u8] = b"stake";
pub const REPUTATION_SEED: &[u8] = b"reputation";
pub const MERCHANT_SEED: &[u8] = b"merchant";
pub const RATING_SEED: &[u8] = b"rating";
pub const ESCROW_SEED: &[u8] = b"escrow";
pub const BADGE_SEED: &[u8] = b"badge";

// Token Constants
pub const MONK_TOKEN_DECIMALS: u8 = 9;
pub const MONK_TOKEN_SYMBOL: &str = "MONK";

// Reward Constants
pub const REDEMPTION_REWARD: u64 = 100_000_000; // 0.1 MONK per redemption
pub const STAKING_REWARD_PER_DAY: u64 = 10_000_000; // 0.01 MONK per day
pub const BASE_REWARD_PER_NFT_PER_SECOND: u64 = 115; // Base reward per NFT per second (approximately 0.01 MONK per day)
pub const LISTING_REWARD: u64 = 50_000_000; // 0.05 MONK for listing
pub const POOL_PARTICIPATION_REWARD: u64 = 20_000_000; // 0.02 MONK for pool participation

// Badge Reward Multipliers (percentage)
pub const BRONZE_REWARD_MULTIPLIER: u32 = 110; // 1.1x multiplier
pub const SILVER_REWARD_MULTIPLIER: u32 = 125; // 1.25x multiplier
pub const GOLD_REWARD_MULTIPLIER: u32 = 150; // 1.5x multiplier
pub const PLATINUM_REWARD_MULTIPLIER: u32 = 200; // 2.0x multiplier
pub const DIAMOND_REWARD_MULTIPLIER: u32 = 300; // 3.0x multiplier

// Reputation Points
pub const POINTS_LIST_DEAL: u64 = 10;
pub const POINTS_BUY_DEAL: u64 = 5;
pub const POINTS_REDEEM_DEAL: u64 = 15;
pub const POINTS_JOIN_POOL: u64 = 8;
pub const POINTS_STAKE_NFT: u64 = 12;
pub const POINTS_RATE_DEAL: u64 = 3;

// Badge Levels
pub const BADGE_BRONZE: u8 = 1;
pub const BADGE_SILVER: u8 = 2;
pub const BADGE_GOLD: u8 = 3;
pub const BADGE_PLATINUM: u8 = 4;
pub const BADGE_DIAMOND: u8 = 5;

// Badge Thresholds (reputation points)
pub const BRONZE_THRESHOLD: u64 = 50;
pub const SILVER_THRESHOLD: u64 = 200;
pub const GOLD_THRESHOLD: u64 = 500;
pub const PLATINUM_THRESHOLD: u64 = 1000;
pub const DIAMOND_THRESHOLD: u64 = 2500;

// Pool Constants
pub const MIN_POOL_PARTICIPANTS: u8 = 2;
pub const MAX_POOL_PARTICIPANTS: u8 = 8;
pub const POOL_EXPIRY_SECONDS: i64 = 86400 * 7; // 7 days

// Deal Constants
pub const MAX_LOCATION_LENGTH: usize = 100;
pub const MAX_EVENT_NAME_LENGTH: usize = 50;
pub const MAX_EVENT_DESC_LENGTH: usize = 200;
pub const MAX_MERCHANT_ID_LENGTH: usize = 50;
pub const MAX_COMMENT_LENGTH: usize = 500;
pub const MAX_MERCHANT_NAME_LENGTH: usize = 100;
pub const MIN_DISCOUNT_PERCENTAGE: u8 = 1;
pub const MAX_DISCOUNT_PERCENTAGE: u8 = 99;

// Rating Constants
pub const MIN_RATING: u8 = 1;
pub const MAX_RATING: u8 = 5;

// Time Constants
pub const SECONDS_PER_DAY: i64 = 86400;

// Activity Types for Reputation
pub const ACTIVITY_LIST_DEAL: u8 = 1;
pub const ACTIVITY_BUY_DEAL: u8 = 2;
pub const ACTIVITY_REDEEM_DEAL: u8 = 3;
pub const ACTIVITY_JOIN_POOL: u8 = 4;
pub const ACTIVITY_STAKE_NFT: u8 = 5;
pub const ACTIVITY_RATE_DEAL: u8 = 6;