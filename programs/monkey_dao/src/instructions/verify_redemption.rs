use anchor_lang::prelude::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct VerifyRedemption<'info> {
    #[account(
        mut,
        constraint = deal.merchant == merchant.key()
    )]
    pub deal: Account<'info, Deal>,

    #[account(
        constraint = merchant.authority == merchant_authority.key() @ DealError::NotAuthorizedMerchant
    )]
    pub merchant: Account<'info, Merchant>,

    pub merchant_authority: Signer<'info>,
}

pub fn verify_redemption(ctx: Context<VerifyRedemption>) -> Result<()> {
    let deal = &mut ctx.accounts.deal;
    let clock = Clock::get()?;

    // Verify the redemption is valid
    require!(!deal.is_redeemed, DealError::DealAlreadyRedeemed);
    require!(!deal.is_expired(clock.unix_timestamp), DealError::DealExpired);

    // Mark as verified by merchant
    // In a real implementation, this would be called after scanning QR code
    // or some other verification mechanism

    emit!(RedemptionVerifiedEvent {
        deal: deal.key(),
        merchant: ctx.accounts.merchant.key(),
        verified_by: ctx.accounts.merchant_authority.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct RedemptionVerifiedEvent {
    pub deal: Pubkey,
    pub merchant: Pubkey,
    pub verified_by: Pubkey,
    pub timestamp: i64,
}