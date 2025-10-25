use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub listing: Pubkey,
    pub initiator: Pubkey,
    pub pool_size: u8, // 2, 4, or 6
    pub current_participants: u8,
    pub price_per_person: u64,
    pub total_deposited: u64,
    pub is_active: bool,
    pub is_completed: bool,
    #[max_len(6)]
    pub participants: Vec<Pubkey>,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PoolParticipant {
    pub pool: Pubkey,
    pub participant: Pubkey,
    pub amount_deposited: u64,
    pub joined_at: i64,
    pub bump: u8,
}