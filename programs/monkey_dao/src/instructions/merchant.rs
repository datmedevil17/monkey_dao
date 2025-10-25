use anchor_lang::prelude::*;
use crate::state::*;
use crate:: ANCHOR_DISCRIMINATOR;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct RegisterMerchant<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR + Merchant::INIT_SPACE,
        seeds = [b"merchant", authority.key().as_ref()],
        bump
    )]
    pub merchant: Account<'info, Merchant>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyMerchant<'info> {
    #[account(mut)]
    pub platform_authority: Signer<'info>,
    
    #[account(
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.authority == platform_authority.key() @ ErrorCode::NotPlatformAuthority
    )]
    pub config: Account<'info, PlatformConfig>,
    
    #[account(mut)]
    pub merchant: Account<'info, Merchant>,
}

pub fn register_merchant(
    ctx: Context<RegisterMerchant>,
    business_name: String,
    business_type: String,
    contact_email: String,
    phone: String,
    business_address: String,
    tax_id: String,
) -> Result<()> {
    let merchant = &mut ctx.accounts.merchant;
    let clock = Clock::get()?;
    
    merchant.authority = ctx.accounts.authority.key();
    merchant.business_name = business_name;
    merchant.business_type = business_type;
    merchant.contact_email = contact_email;
    merchant.phone = phone;
    merchant.business_address = business_address;
    merchant.tax_id = tax_id;
    merchant.is_verified = false;
    merchant.total_listings = 0;
    merchant.registration_date = clock.unix_timestamp;
    merchant.bump = ctx.bumps.merchant;
    
    msg!("Merchant registered successfully: {}", merchant.business_name);
    Ok(())
}

pub fn verify_merchant(ctx: Context<VerifyMerchant>) -> Result<()> {
    let merchant = &mut ctx.accounts.merchant;
    merchant.is_verified = true;
    
    msg!("Merchant verified: {}", merchant.business_name);
    Ok(())
}