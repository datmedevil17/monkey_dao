use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub deal: Pubkey,                    // 32
    pub starter: Pubkey,                 // 32
    pub target_amount: u64,              // 8
    pub current_amount: u64,             // 8
    pub target_participants: u8,         // 1
    pub current_participants: u8,        // 1
    pub participants: Vec<Participant>,  // 4 + (40 * max_participants)
    pub is_active: bool,                 // 1
    pub is_executed: bool,               // 1
    pub created_at: i64,                 // 8
    pub expires_at: i64,                 // 8
    pub executed_at: Option<i64>,        // 1 + 8
    pub bump: u8,                        // 1
}

impl Pool {
    pub const MAX_PARTICIPANTS: usize = 8;
    
    pub const LEN: usize = 8 + // discriminator
        32 + // deal
        32 + // starter
        8 +  // target_amount
        8 +  // current_amount
        1 +  // target_participants
        1 +  // current_participants
        4 + (40 * Self::MAX_PARTICIPANTS) + // participants
        1 +  // is_active
        1 +  // is_executed
        8 +  // created_at
        8 +  // expires_at
        (1 + 8) + // executed_at
        1;   // bump

    pub fn is_expired(&self, current_timestamp: i64) -> bool {
        current_timestamp > self.expires_at
    }

    pub fn is_target_reached(&self) -> bool {
        self.current_amount >= self.target_amount &&
        self.current_participants >= self.target_participants
    }

    pub fn has_participant(&self, user: &Pubkey) -> bool {
        self.participants.iter().any(|p| p.user == *user)
    }

    pub fn get_participant_contribution(&self, user: &Pubkey) -> Option<u64> {
        self.participants
            .iter()
            .find(|p| p.user == *user)
            .map(|p| p.contribution)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Participant {
    pub user: Pubkey,         // 32
    pub contribution: u64,    // 8
}