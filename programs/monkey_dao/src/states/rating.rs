use anchor_lang::prelude::*;

#[account]
pub struct Rating {
    pub deal: Pubkey,           // 32
    pub user: Pubkey,           // 32
    pub rating: u8,             // 1 (1-5 stars)
    pub comment: String,        // 4 + length
    pub created_at: i64,        // 8
    pub is_verified_purchase: bool, // 1 - True if user actually purchased the deal
    pub bump: u8,               // 1
}

impl Rating {
    pub const LEN: usize = 8 + // discriminator
        32 + // deal
        32 + // user
        1 +  // rating
        (4 + 500) + // comment (max length)
        8 +  // created_at
        1 +  // is_verified_purchase
        1;   // bump

    pub fn is_valid_rating(&self) -> bool {
        use crate::constants::*;
        self.rating >= MIN_RATING && self.rating <= MAX_RATING
    }
}