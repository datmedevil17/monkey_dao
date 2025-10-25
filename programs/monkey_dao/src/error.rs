use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid price provided")]
    InvalidPrice,
    
    #[msg("New price must be lower than original price")]
    PriceTooHigh,
    
    #[msg("Listing is not active")]
    ListingNotActive,
    
    #[msg("Coupon has already been used")]
    CouponAlreadyUsed,
    
    #[msg("Unauthorized action")]
    Unauthorized,
    
    #[msg("Invalid pool size. Must be 2, 4, or 6")]
    InvalidPoolSize,
    
    #[msg("This listing is not a group deal")]
    NotGroupDeal,
    
    #[msg("Group deal price not available for this size")]
    DealNotAvailable,
    
    #[msg("Pool is already full")]
    PoolFull,
    
    #[msg("Pool is not active")]
    PoolNotActive,
    
    #[msg("Pool is not yet complete")]
    PoolNotComplete,
    
    #[msg("Only pool initiator can cancel")]
    NotPoolInitiator,
    
    #[msg("Merchant is not verified")]
    MerchantNotVerified,
    
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,
    
    #[msg("NFT is not staked")]
    NotStaked,
    
    #[msg("NFT is already staked")]
    AlreadyStaked,
    
    #[msg("Cannot stake used coupons")]
    CannotStakeUsedCoupon,
    
    #[msg("Invalid signature")]
    InvalidSignature,
    
    #[msg("Coupon has expired")]
    CouponExpired,
    
    #[msg("Insufficient time elapsed for rewards")]
    InsufficientTimeElapsed,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Only platform authority can perform this action")]
    NotPlatformAuthority,
}