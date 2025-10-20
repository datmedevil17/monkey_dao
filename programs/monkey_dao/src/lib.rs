#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;
use states::GroupPrices;

declare_id!("4hmqotpqtTjt3fDoyX1HR7QLqcxdPSb2V6ZctRnkiCfY");

#[program]
pub mod monkey_dao {
    use super::*;

    // Deal Management
    pub fn list_deal(
        ctx: Context<ListDeal>,
        price: u64,
        location: String,
        latitude: Option<f64>,
        longitude: Option<f64>,
        is_group_deal: bool,
        group_prices: Option<GroupPrices>,
        is_crypto_based: bool,
        event_name: Option<String>,
        event_description: Option<String>,
        discount_percentage: u8,
        expiry_timestamp: i64,
        merchant_id: String,
        max_supply: u64,
    ) -> Result<()> {
        instructions::list_deal(
            ctx,
            price,
            location,
            latitude,
            longitude,
            is_group_deal,
            group_prices,
            is_crypto_based,
            event_name,
            event_description,
            discount_percentage,
            expiry_timestamp,
            merchant_id,
            max_supply,
        )
    }

    pub fn buy_deal(ctx: Context<BuyDeal>) -> Result<()> {
        instructions::buy_deal(ctx)
    }

    pub fn relist_deal(ctx: Context<RelistDeal>, new_price: u64) -> Result<()> {
        instructions::relist_deal(ctx, new_price)
    }

    pub fn redeem_deal(ctx: Context<RedeemDeal>, signature: Vec<u8>) -> Result<()> {
        instructions::redeem_deal(ctx, signature)
    }

    pub fn rate_deal(ctx: Context<RateDeal>, rating: u8, comment: String) -> Result<()> {
        instructions::rate_deal(ctx, rating, comment)
    }

    // Group Deal / Pooling
    pub fn start_pool(ctx: Context<StartPool>, target_amount: u64, target_participants: u8) -> Result<()> {
        instructions::start_pool(ctx, target_amount, target_participants)
    }

    pub fn join_pool(ctx: Context<JoinPool>, amount: u64) -> Result<()> {
        instructions::join_pool(ctx, amount)
    }

    pub fn execute_pool_purchase(ctx: Context<ExecutePoolPurchase>) -> Result<()> {
        instructions::execute_pool_purchase(ctx)
    }

    pub fn cancel_pool(ctx: Context<CancelPool>) -> Result<()> {
        instructions::cancel_pool(ctx)
    }

    // NFT Staking & Rewards
    pub fn stake_nft(ctx: Context<StakeNft>) -> Result<()> {
        instructions::stake_nft(ctx)
    }

    pub fn unstake_nft(ctx: Context<UnstakeNft>) -> Result<()> {
        instructions::unstake_nft(ctx)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim_rewards(ctx)
    }

    pub fn get_claimable_rewards(ctx: Context<GetClaimableRewards>) -> Result<u64> {
        instructions::get_claimable_rewards::handler(ctx)
    }

    // Reputation & Loyalty
    pub fn update_reputation(ctx: Context<UpdateReputation>, activity_type: u8) -> Result<()> {
        instructions::update_reputation(ctx, activity_type)
    }

    pub fn mint_badge(ctx: Context<MintBadge>, badge_level: u8) -> Result<()> {
        instructions::mint_badge(ctx, badge_level)
    }

    // Merchant Functions
    pub fn register_merchant(ctx: Context<RegisterMerchant>, merchant_name: String) -> Result<()> {
        instructions::register_merchant(ctx, merchant_name)
    }

    pub fn verify_redemption(ctx: Context<VerifyRedemption>) -> Result<()> {
        instructions::verify_redemption(ctx)
    }

}