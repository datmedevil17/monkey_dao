use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer, transfer, MintTo, mint_to},
};
use crate::state::*;
use crate::constants::*;
use crate:: ANCHOR_DISCRIMINATOR;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct BuyNFT<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,
    
    pub nft_mint: Account<'info, Mint>,
    
    /// CHECK: Seller account to receive payment
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, PlatformConfig>,
    
    /// CHECK: Platform wallet to receive fees
    #[account(
        mut,
        constraint = platform_wallet.key() == config.platform_wallet @ ErrorCode::Unauthorized
    )]
    pub platform_wallet: UncheckedAccount<'info>,
    
    // MONK Token accounts
    #[account(
        mut,
        constraint = monk_mint.key() == config.monk_mint @ ErrorCode::Unauthorized
    )]
    pub monk_mint: Account<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = monk_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_monk_account: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = buyer,
        space = ANCHOR_DISCRIMINATOR + UserStats::INIT_SPACE,
        seeds = [b"user_stats", buyer.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy_nft(ctx: Context<BuyNFT>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    let clock = Clock::get()?;
    
    // Validations
    require!(listing.is_active, ErrorCode::ListingNotActive);
    require!(!listing.is_used, ErrorCode::CouponAlreadyUsed);
    require!(listing.expiry_date > clock.unix_timestamp, ErrorCode::CouponExpired);

    let price = listing.current_price;
    let platform_fee = (price * ctx.accounts.config.platform_fee_bps) / 10000;
    let seller_amount = price.checked_sub(platform_fee)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Transfer SOL to seller
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.seller.to_account_info(),
            },
        ),
        seller_amount,
    )?;

    // Transfer platform fee
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.platform_wallet.to_account_info(),
            },
        ),
        platform_fee,
    )?;

    // Transfer NFT from vault to buyer
    let seeds = &[
        b"listing",
        listing.nft_mint.as_ref(),
        &[listing.bump],
    ];
    let signer = &[&seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: listing.to_account_info(),
        },
        signer,
    );
    transfer(transfer_ctx, 1)?;

    // Calculate MONK token rewards (10% of price)
    let monk_reward = (price * PURCHASE_REWARD_BPS) / 10000;
    
    
    let binding = [ctx.accounts.config.bump];
    let config_signer = &[&[b"config".as_ref(), &binding][..]];

    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.monk_mint.to_account_info(),
            to: ctx.accounts.buyer_monk_account.to_account_info(),
            authority: ctx.accounts.config.to_account_info(),
        },
        config_signer,
    );
    mint_to(mint_ctx, monk_reward)?;

    // Update listing
    listing.is_active = false;
    listing.seller = ctx.accounts.buyer.key();
    listing.total_sales = listing.total_sales.checked_add(1)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Update user stats
    let user_stats = &mut ctx.accounts.user_stats;
    if user_stats.user == Pubkey::default() {
        user_stats.user = ctx.accounts.buyer.key();
        user_stats.bump = ctx.bumps.user_stats;
    }
    user_stats.total_purchases = user_stats.total_purchases.checked_add(1)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    user_stats.total_monk_earned = user_stats.total_monk_earned.checked_add(monk_reward)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    msg!("NFT purchased successfully");
    msg!("Price paid: {} lamports", price);
    msg!("Platform fee: {} lamports", platform_fee);
    msg!("MONK tokens rewarded: {}", monk_reward);
    
    Ok(())
}