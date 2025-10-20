use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
#[instruction(price: u64)]
pub struct ListDeal<'info> {
    #[account(
        init,
        payer = owner,
        space = Deal::LEN,
        seeds = [DEAL_SEED, nft_mint.key().as_ref()],
        bump
    )]
    pub deal: Account<'info, Deal>,

    #[account(mut)]
    pub owner: Signer<'info>,

    /// NFT Mint account
    pub nft_mint: Account<'info, Mint>,

    /// Owner's NFT token account (must own the NFT)
    #[account(
        constraint = owner_nft_account.mint == nft_mint.key(),
        constraint = owner_nft_account.owner == owner.key(),
        constraint = owner_nft_account.amount == 1
    )]
    pub owner_nft_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = owner,
        space = UserProfile::LEN,
        seeds = [USER_PROFILE_SEED, owner.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    #[account(
        seeds = [MERCHANT_SEED, merchant.key().as_ref()],
        bump
    )]
    pub merchant: Account<'info, Merchant>,

    /// MONK token mint for rewards
    #[account(mut)]
    pub monk_token_mint: Account<'info, Mint>,

    /// User's MONK token account for rewards
    #[account(
        mut,
        constraint = user_monk_account.mint == monk_token_mint.key(),
        constraint = user_monk_account.owner == owner.key()
    )]
    pub user_monk_account: Account<'info, TokenAccount>,

    /// Program's token authority for minting rewards
    /// CHECK: PDA used as token authority
    #[account(
        seeds = [b"token_authority"],
        bump
    )]
    pub token_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn list_deal(
    ctx: Context<ListDeal>,
    price: u64,
    location: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
    is_group_deal: bool,
    group_prices: Option<GroupPrices>,
    is_crypto_based: bool,
    event_name: Option<String>,
    event_description: Option<String>,
    discount_percentage: u8,
    expiry_timestamp: i64,
    merchant_id: String,
    max_supply: u64,
) -> Result<()> {
    let deal = &mut ctx.accounts.deal;
    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // Validations
    require!(price > 0, DealError::InvalidPrice);
    require!(
        location.len() <= MAX_LOCATION_LENGTH,
        DealError::LocationTooLong
    );
    require!(
        merchant_id.len() <= MAX_MERCHANT_ID_LENGTH,
        DealError::MerchantIdTooLong
    );
    require!(
        discount_percentage >= MIN_DISCOUNT_PERCENTAGE && 
        discount_percentage <= MAX_DISCOUNT_PERCENTAGE,
        DealError::InvalidDiscountPercentage
    );

    // Validate group deal requirements
    if is_group_deal {
        require!(group_prices.is_some(), DealError::GroupPricesRequired);
    }

    // Validate crypto-based deal requirements
    if is_crypto_based {
        require!(event_name.is_some() && event_description.is_some(), DealError::EventDetailsRequired);
        
        if let Some(ref name) = event_name {
            require!(name.len() <= MAX_EVENT_NAME_LENGTH, DealError::EventNameTooLong);
        }
        if let Some(ref desc) = event_description {
            require!(desc.len() <= MAX_EVENT_DESC_LENGTH, DealError::EventDescriptionTooLong);
        }
    }

    // Initialize deal
    deal.nft_mint = ctx.accounts.nft_mint.key();
    deal.owner = ctx.accounts.owner.key();
    deal.merchant = ctx.accounts.merchant.key();
    deal.price = price;
    deal.location = location;
    deal.latitude = latitude;
    deal.longitude = longitude;
    deal.is_used = false;
    deal.is_redeemed = false;
    deal.is_group_deal = is_group_deal;
    deal.group_prices = group_prices;
    deal.is_crypto_based = is_crypto_based;
    deal.event_name = event_name;
    deal.event_description = event_description;
    deal.discount_percentage = discount_percentage;
    deal.expiry_timestamp = expiry_timestamp;
    deal.merchant_id = merchant_id;
    deal.created_at = clock.unix_timestamp;
    deal.total_ratings = 0;
    deal.total_rating_value = 0;
    deal.times_sold = 0;
    deal.current_supply = 0;
    deal.max_supply = max_supply;
    deal.bump = ctx.bumps.deal;

    // Initialize user profile if needed
    if user_profile.owner == Pubkey::default() {
        user_profile.owner = ctx.accounts.owner.key();
        user_profile.created_at = clock.unix_timestamp;
        user_profile.bump = ctx.bumps.user_profile;
    }

    // Update user profile
    user_profile.total_deals_listed = user_profile.total_deals_listed
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    user_profile.update_activity(clock.unix_timestamp);

    // Award reputation points
    user_profile.add_reputation_points(POINTS_LIST_DEAL)?;

    // Mint MONK rewards for listing
    let authority_bump = ctx.bumps.token_authority;
    let authority_seeds = &[b"token_authority".as_ref(), &[authority_bump]];
    let signer = &[&authority_seeds[..]];

    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.monk_token_mint.to_account_info(),
        to: ctx.accounts.user_monk_account.to_account_info(),
        authority: ctx.accounts.token_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
    token::mint_to(cpi_ctx, LISTING_REWARD)?;

    user_profile.total_rewards_earned = user_profile.total_rewards_earned
        .checked_add(LISTING_REWARD)
        .ok_or(DealError::ArithmeticOverflow)?;

    emit!(DealListedEvent {
        deal: deal.key(),
        nft_mint: deal.nft_mint,
        owner: deal.owner,
        price: deal.price,
        merchant: deal.merchant,
        is_group_deal: deal.is_group_deal,
        is_crypto_based: deal.is_crypto_based,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct DealListedEvent {
    pub deal: Pubkey,
    pub nft_mint: Pubkey,
    pub owner: Pubkey,
    pub price: u64,
    pub merchant: Pubkey,
    pub is_group_deal: bool,
    pub is_crypto_based: bool,
    pub timestamp: i64,
}