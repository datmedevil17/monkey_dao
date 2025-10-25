use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Merchant {
    pub authority: Pubkey,
    #[max_len(100)]
    pub business_name: String,
    #[max_len(50)]
    pub business_type: String,
    #[max_len(100)]
    pub contact_email: String,
    #[max_len(20)]
    pub phone: String,
    #[max_len(200)]
    pub business_address: String,
    #[max_len(50)]
    pub tax_id: String,
    pub is_verified: bool,
    pub total_listings: u64,
    pub registration_date: i64,
    pub bump: u8,
}