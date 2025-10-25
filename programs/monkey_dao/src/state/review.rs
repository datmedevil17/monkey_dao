use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Review {
    pub listing: Pubkey,
    pub reviewer: Pubkey,
    pub rating: u8, // 1-5 stars
    #[max_len(500)]
    pub comment: String,
    pub created_at: i64,
    pub bump: u8,
}