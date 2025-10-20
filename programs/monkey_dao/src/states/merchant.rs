use anchor_lang::prelude::*;

#[account]
pub struct Merchant {
    pub authority: Pubkey,          // 32
    pub merchant_name: String,      // 4 + length
    pub total_deals_listed: u64,    // 8
    pub total_deals_sold: u64,      // 8
    pub total_deals_redeemed: u64,  // 8
    pub total_revenue: u64,         // 8
    pub is_verified: bool,          // 1
    pub registered_at: i64,         // 8
    pub last_activity_at: i64,      // 8
    pub bump: u8,                   // 1
}

impl Merchant {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        (4 + 100) + // merchant_name (max length)
        8 +  // total_deals_listed
        8 +  // total_deals_sold
        8 +  // total_deals_redeemed
        8 +  // total_revenue
        1 +  // is_verified
        8 +  // registered_at
        8 +  // last_activity_at
        1;   // bump

    pub fn add_revenue(&mut self, amount: u64) -> Result<()> {
        self.total_revenue = self.total_revenue
            .checked_add(amount)
            .ok_or(error!(crate::errors::DealError::ArithmeticOverflow))?;
        Ok(())
    }

    pub fn update_activity(&mut self, current_timestamp: i64) {
        self.last_activity_at = current_timestamp;
    }

    pub fn get_success_rate(&self) -> f64 {
        if self.total_deals_sold == 0 {
            0.0
        } else {
            (self.total_deals_redeemed as f64 / self.total_deals_sold as f64) * 100.0
        }
    }
}