use anchor_lang::prelude::*;

#[account]
pub struct Deal {
    pub nft_mint: Pubkey,              // 32
    pub owner: Pubkey,                 // 32
    pub merchant: Pubkey,              // 32
    pub price: u64,                    // 8
    pub location: String,              // 4 + length
    pub latitude: Option<f64>,         // 1 + 8
    pub longitude: Option<f64>,        // 1 + 8
    pub is_used: bool,                 // 1
    pub is_redeemed: bool,             // 1
    pub is_group_deal: bool,           // 1
    pub group_prices: Option<GroupPrices>, // 1 + 24
    pub is_crypto_based: bool,         // 1
    pub event_name: Option<String>,    // 1 + 4 + length
    pub event_description: Option<String>, // 1 + 4 + length
    pub discount_percentage: u8,       // 1
    pub expiry_timestamp: i64,         // 8
    pub merchant_id: String,           // 4 + length
    pub created_at: i64,               // 8
    pub total_ratings: u64,            // 8
    pub total_rating_value: u64,       // 8
    pub times_sold: u64,               // 8
    pub current_supply: u64,           // 8
    pub max_supply: u64,               // 8
    pub bump: u8,                      // 1
}

impl Deal {
    pub const LEN: usize = 8 + // discriminator
        32 + // nft_mint
        32 + // owner
        32 + // merchant
        8 +  // price
        (4 + 100) + // location (max length)
        (1 + 8) + // latitude
        (1 + 8) + // longitude
        1 +  // is_used
        1 +  // is_redeemed
        1 +  // is_group_deal
        (1 + 24) + // group_prices
        1 +  // is_crypto_based
        (1 + 4 + 50) + // event_name (max length)
        (1 + 4 + 200) + // event_description (max length)
        1 +  // discount_percentage
        8 +  // expiry_timestamp
        (4 + 50) + // merchant_id (max length)
        8 +  // created_at
        8 +  // total_ratings
        8 +  // total_rating_value
        8 +  // times_sold
        8 +  // current_supply
        8 +  // max_supply
        1;   // bump

    pub fn is_expired(&self, current_timestamp: i64) -> bool {
        current_timestamp > self.expiry_timestamp
    }

    pub fn average_rating(&self) -> f64 {
        if self.total_ratings == 0 {
            0.0
        } else {
            self.total_rating_value as f64 / self.total_ratings as f64
        }
    }

    pub fn get_group_price(&self, participant_count: u8) -> Option<u64> {
        if let Some(prices) = &self.group_prices {
            match participant_count {
                2 => Some(prices.price_for_2),
                4 => Some(prices.price_for_4),
                8 => Some(prices.price_for_8),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct GroupPrices {
    pub price_for_2: u64,   // 8
    pub price_for_4: u64,   // 8
    pub price_for_8: u64,   // 8
}