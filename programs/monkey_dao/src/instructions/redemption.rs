use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint as SplMint, Token as SplTokenProgram, TokenAccount as SplTokenAccount, Burn, burn},
};
use crate::{state::*};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct RedeemNFT<'info> {
    #[account(mut)]
    pub redeemer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"merchant", merchant.authority.as_ref()],
        bump = merchant.bump,
    )]
    pub merchant: Account<'info, Merchant>,
    
    #[account(mut)]
    pub merchant_authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump = listing.bump,
        constraint = listing.merchant == merchant.key() @ ErrorCode::Unauthorized,
        constraint = !listing.is_used @ ErrorCode::CouponAlreadyUsed,
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(mut)]
    pub nft_mint: Account<'info, SplMint>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = redeemer,
        constraint = redeemer_token_account.amount == 1 @ ErrorCode::Unauthorized,
    )]
    pub redeemer_token_account: Account<'info, SplTokenAccount>,
    
    pub token_program: Program<'info, SplTokenProgram>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn redeem_nft(
    ctx: Context<RedeemNFT>,
    signature: Vec<u8>,
) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    let clock = Clock::get()?;
    
    // Verify merchant authority
    require!(
        ctx.accounts.merchant.authority == ctx.accounts.merchant_authority.key(),
        ErrorCode::Unauthorized
    );
    
    // Check expiry
    require!(
        listing.expiry_date > clock.unix_timestamp,
        ErrorCode::CouponExpired
    );
    
    // Simple length check for signature (replace with actual verification if required)
    require!(signature.len() == 64, ErrorCode::InvalidSignature);
    
    // Mark coupon as used
    listing.is_used = true;
    
    // Burn the NFT (1 token)
    let burn_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.nft_mint.to_account_info(),
            from: ctx.accounts.redeemer_token_account.to_account_info(),
            authority: ctx.accounts.redeemer.to_account_info(),
        },
    );
    burn(burn_ctx, 1)?;
    
    msg!("NFT coupon redeemed and burned successfully");
    msg!("Redeemer: {}", ctx.accounts.redeemer.key());
    msg!("Merchant: {}", ctx.accounts.merchant.business_name);
    
    Ok(())
}
