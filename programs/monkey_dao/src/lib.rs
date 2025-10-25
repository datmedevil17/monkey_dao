#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

declare_id!("4hmqotpqtTjt3fDoyX1HR7QLqcxdPSb2V6ZctRnkiCfY");

pub mod state;
pub mod instructions;
pub mod error;
pub mod constants;

use instructions::*;
pub use constants::*;

#[program]
pub mod nft_coupon_platform {
    use super::*;

    // ==================== MERCHANT INSTRUCTIONS ====================
    pub fn register_merchant(
        ctx: Context<RegisterMerchant>,
        business_name: String,
        business_type: String,
        contact_email: String,
        phone: String,
        business_address: String,
        tax_id: String,
    ) -> Result<()> {
        instructions::merchant::register_merchant(
            ctx,
            business_name,
            business_type,
            contact_email,
            phone,
            business_address,
            tax_id,
        )
    }

    pub fn verify_merchant(ctx: Context<VerifyMerchant>) -> Result<()> {
        instructions::merchant::verify_merchant(ctx)
    }

    // ==================== LISTING INSTRUCTIONS ====================
    pub fn list_nft(
        ctx: Context<ListNFT>,
        price: u64,
        is_group_deal: bool,
        deal_price_2: Option<u64>,
        deal_price_4: Option<u64>,
        deal_price_6: Option<u64>,
        coupon_description: String,
        expiry_date: i64,
    ) -> Result<()> {
        instructions::listing::list_nft(
            ctx,
            price,
            is_group_deal,
            deal_price_2,
            deal_price_4,
            deal_price_6,
            coupon_description,
            expiry_date,
        )
    }

    pub fn relist_nft(ctx: Context<RelistNFT>, new_price: u64) -> Result<()> {
        instructions::listing::relist_nft(ctx, new_price)
    }

    pub fn delist_nft(ctx: Context<DelistNFT>) -> Result<()> {
        instructions::listing::delist_nft(ctx)
    }

    // ==================== TRADING INSTRUCTIONS ====================
    pub fn buy_nft(ctx: Context<BuyNFT>) -> Result<()> {
        instructions::trading::buy_nft(ctx)
    }

    // ==================== POOL INSTRUCTIONS ====================
    pub fn create_pool(ctx: Context<CreatePool>, pool_size: u8) -> Result<()> {
        instructions::pool::create_pool(ctx, pool_size)
    }

    pub fn join_pool(ctx: Context<JoinPool>) -> Result<()> {
        instructions::pool::join_pool(ctx)
    }

    pub fn cancel_pool(ctx: Context<CancelPool>) -> Result<()> {
        instructions::pool::cancel_pool(ctx)
    }

    // ==================== REVIEW INSTRUCTIONS ====================
    pub fn add_review(
        ctx: Context<AddReview>,
        rating: u8,
        comment: String,
    ) -> Result<()> {
        instructions::review::add_review(ctx, rating, comment)
    }

    // ==================== STAKING INSTRUCTIONS ====================
    pub fn stake_nft(ctx: Context<StakeNFT>) -> Result<()> {
        instructions::staking::stake_nft(ctx)
    }

    pub fn unstake_nft(ctx: Context<UnstakeNFT>) -> Result<()> {
        instructions::staking::unstake_nft(ctx)
    }

    pub fn claim_staking_rewards(ctx: Context<ClaimStakingRewards>) -> Result<()> {
        instructions::staking::claim_staking_rewards(ctx)
    }

    // ==================== REDEMPTION INSTRUCTIONS ====================
    pub fn redeem_nft(
        ctx: Context<RedeemNFT>,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::redemption::redeem_nft(ctx, signature)
    }

    // ==================== MONK TOKEN INSTRUCTIONS ====================
    pub fn initialize_monk_mint(ctx: Context<InitializeMonkMint>) -> Result<()> {
        instructions::monk_token::initialize_monk_mint(ctx)
    }
}