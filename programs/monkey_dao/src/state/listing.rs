use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub nft_mint: Pubkey,
    pub seller: Pubkey,
    pub merchant: Pubkey,
    pub original_price: u64,
    pub current_price: u64,
    pub is_group_deal: bool,
    pub deal_price_2: Option<u64>,
    pub deal_price_4: Option<u64>,
    pub deal_price_6: Option<u64>,
    pub is_active: bool,
    pub is_used: bool,
    pub total_sales: u64,
    #[max_len(500)]
    pub coupon_description: String,
    pub expiry_date: i64,
    pub created_at: i64,
    pub average_rating: u8, // 0-100 (representing 0.0-5.0 stars * 20)
    pub total_reviews: u64,
    pub bump: u8,
}