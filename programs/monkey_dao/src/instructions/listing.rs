use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer, transfer},
};
use crate::{state::*, error::ErrorCode, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct ListNFT<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"merchant", seller.key().as_ref()],
        bump = merchant.bump,
        constraint = merchant.is_verified @ ErrorCode::MerchantNotVerified
    )]
    pub merchant: Account<'info, Merchant>,
    
    #[account(
        init,
        payer = seller,
        space = ANCHOR_DISCRIMINATOR + Listing::INIT_SPACE,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    
    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller,
        constraint = seller_token_account.amount == 1 @ ErrorCode::Unauthorized
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = seller,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RelistNFT<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump = listing.bump,
        constraint = listing.seller == seller.key() @ ErrorCode::Unauthorized
    )]
    pub listing: Account<'info, Listing>,
    
    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller,
        constraint = seller_token_account.amount == 1 @ ErrorCode::Unauthorized
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DelistNFT<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump = listing.bump,
        constraint = listing.seller == seller.key() @ ErrorCode::Unauthorized
    )]
    pub listing: Account<'info, Listing>,
    
    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn list_nft(
    ctx: Context<ListNFT>,
    price: u64,
    is_group_deal: bool,
    deal_price_2: Option<u64>,
    deal_price_4: Option<u64>,
    deal_price_6: Option<u64>,
    coupon_description: String,
    expiry_date: i64,
) -> Result<()> {
    require!(price > 0, ErrorCode::InvalidPrice);
    let clock = Clock::get()?;
    require!(expiry_date > clock.unix_timestamp, ErrorCode::CouponExpired);
    
    let listing = &mut ctx.accounts.listing;
    listing.nft_mint = ctx.accounts.nft_mint.key();
    listing.seller = ctx.accounts.seller.key();
    listing.merchant = ctx.accounts.merchant.key();
    listing.original_price = price;
    listing.current_price = price;
    listing.is_group_deal = is_group_deal;
    listing.deal_price_2 = deal_price_2;
    listing.deal_price_4 = deal_price_4;
    listing.deal_price_6 = deal_price_6;
    listing.is_active = true;
    listing.is_used = false;
    listing.total_sales = 0;
    listing.coupon_description = coupon_description;
    listing.expiry_date = expiry_date;
    listing.created_at = clock.unix_timestamp;
    listing.average_rating = 0;
    listing.total_reviews = 0;
    listing.bump = ctx.bumps.listing;

    // Transfer NFT to vault
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        },
    );
    transfer(transfer_ctx, 1)?;

    let merchant = &mut ctx.accounts.merchant;
    merchant.total_listings = merchant.total_listings.checked_add(1)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    msg!("NFT listed successfully at price: {} lamports", price);
    Ok(())
}

pub fn relist_nft(ctx: Context<RelistNFT>, new_price: u64) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    
    require!(!listing.is_active, ErrorCode::ListingNotActive);
    require!(new_price > 0, ErrorCode::InvalidPrice);
    require!(new_price < listing.original_price, ErrorCode::PriceTooHigh);
    require!(!listing.is_used, ErrorCode::CannotStakeUsedCoupon);

    listing.current_price = new_price;
    listing.is_active = true;
    
    // Transfer NFT to vault
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        },
    );
    transfer(transfer_ctx, 1)?;

    msg!("NFT relisted at new price: {} lamports", new_price);
    Ok(())
}

pub fn delist_nft(ctx: Context<DelistNFT>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    
    require!(listing.is_active, ErrorCode::ListingNotActive);

    // Transfer NFT back to seller using PDA signer
    let binding = ctx.accounts.nft_mint.key();
    let seeds = &[
        b"listing",
        binding.as_ref(),
        &[listing.bump],
    ];
    let signer = &[&seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: listing.to_account_info(),
        },
        signer,
    );
    transfer(transfer_ctx, 1)?;

    listing.is_active = false;

    msg!("NFT delisted successfully");
    Ok(())
}