use anchor_lang::prelude::*;

#[error_code]
pub enum DealError {
    #[msg("Deal has already been used")]
    DealAlreadyUsed,
    
    #[msg("Deal has expired")]
    DealExpired,
    
    #[msg("Invalid price")]
    InvalidPrice,
    
    #[msg("Location string is too long")]
    LocationTooLong,
    
    #[msg("Event name is too long")]
    EventNameTooLong,
    
    #[msg("Event description is too long")]
    EventDescriptionTooLong,
    
    #[msg("Merchant ID is too long")]
    MerchantIdTooLong,
    
    #[msg("Not the deal owner")]
    NotDealOwner,
    
    #[msg("Deal is not transferable")]
    DealNotTransferable,
    
    #[msg("Invalid discount percentage")]
    InvalidDiscountPercentage,
    
    #[msg("Max supply reached")]
    MaxSupplyReached,
    
    #[msg("Invalid rating value")]
    InvalidRating,
    
    #[msg("Comment is too long")]
    CommentTooLong,
    
    #[msg("Pool not active")]
    PoolNotActive,
    
    #[msg("Pool target not reached")]
    PoolTargetNotReached,
    
    #[msg("Pool target exceeded")]
    PoolTargetExceeded,
    
    #[msg("Pool already executed")]
    PoolAlreadyExecuted,
    
    #[msg("Not pool starter")]
    NotPoolStarter,
    
    #[msg("Invalid pool participant count")]
    InvalidPoolParticipants,
    
    #[msg("Pool expired")]
    PoolExpired,
    
    #[msg("Already joined pool")]
    AlreadyJoinedPool,
    
    #[msg("Insufficient pool contribution")]
    InsufficientPoolContribution,
    
    #[msg("NFT not staked")]
    NftNotStaked,
    
    #[msg("NFT already staked")]
    NftAlreadyStaked,
    
    #[msg("Not NFT owner")]
    NotNftOwner,
    
    #[msg("No rewards to claim")]
    NoRewardsToClaim,
    
    #[msg("Insufficient reputation for badge")]
    InsufficientReputationForBadge,
    
    #[msg("Badge already minted")]
    BadgeAlreadyMinted,
    
    #[msg("Invalid badge level")]
    InvalidBadgeLevel,
    
    #[msg("Not authorized merchant")]
    NotAuthorizedMerchant,
    
    #[msg("Merchant already registered")]
    MerchantAlreadyRegistered,
    
    #[msg("Invalid redemption signature")]
    InvalidRedemptionSignature,
    
    #[msg("Deal already redeemed")]
    DealAlreadyRedeemed,
    
    #[msg("Group prices required for group deals")]
    GroupPricesRequired,
    
    #[msg("Event details required for crypto-based deals")]
    EventDetailsRequired,
    
    #[msg("Deal is not a group deal")]
    NotGroupDeal,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Arithmetic underflow")]
    ArithmeticUnderflow,
    
    #[msg("Invalid activity type")]
    InvalidActivityType,
    
    #[msg("Merchant name too long")]
    MerchantNameTooLong,
}