use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct RegisterMerchant<'info> {
    #[account(
        init,
        payer = authority,
        space = Merchant::LEN,
        seeds = [MERCHANT_SEED, authority.key().as_ref()],
        bump
    )]
    pub merchant: Account<'info, Merchant>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn register_merchant(ctx: Context<RegisterMerchant>, merchant_name: String) -> Result<()> {
    let merchant = &mut ctx.accounts.merchant;
    let clock = Clock::get()?;

    // Validate merchant name
    require!(
        merchant_name.len() <= MAX_MERCHANT_NAME_LENGTH,
        DealError::MerchantNameTooLong
    );
    require!(!merchant_name.is_empty(), DealError::MerchantNameTooLong);

    // Initialize merchant
    merchant.authority = ctx.accounts.authority.key();
    merchant.merchant_name = merchant_name.clone();
    merchant.total_deals_listed = 0;
    merchant.total_deals_sold = 0;
    merchant.total_deals_redeemed = 0;
    merchant.total_revenue = 0;
    merchant.is_verified = false; // Requires admin verification
    merchant.registered_at = clock.unix_timestamp;
    merchant.last_activity_at = clock.unix_timestamp;
    merchant.bump = ctx.bumps.merchant;

    emit!(MerchantRegisteredEvent {
        merchant: merchant.key(),
        authority: merchant.authority,
        merchant_name,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct MerchantRegisteredEvent {
    pub merchant: Pubkey,
    pub authority: Pubkey,
    pub merchant_name: String,
    pub timestamp: i64,
}