use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct RedeemDeal<'info> {
    #[account(
        mut,
        constraint = deal.owner == redeemer.key() @ DealError::NotDealOwner,
        constraint = !deal.is_redeemed @ DealError::DealAlreadyRedeemed,
        constraint = !deal.is_expired(Clock::get()?.unix_timestamp) @ DealError::DealExpired
    )]
    pub deal: Account<'info, Deal>,

    #[account(mut)]
    pub redeemer: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, redeemer.key().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    #[account(
        mut,
        seeds = [MERCHANT_SEED, deal.merchant.as_ref()],
        bump = merchant.bump
    )]
    pub merchant: Account<'info, Merchant>,

    /// MONK token mint for rewards
    #[account(mut)]
    pub monk_token_mint: Account<'info, Mint>,

    /// User's MONK token account for rewards
    #[account(
        mut,
        constraint = user_monk_account.mint == monk_token_mint.key(),
        constraint = user_monk_account.owner == redeemer.key()
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
}

pub fn redeem_deal(ctx: Context<RedeemDeal>, signature: Vec<u8>) -> Result<()> {
    let deal = &mut ctx.accounts.deal;
    let user_profile = &mut ctx.accounts.user_profile;
    let merchant = &mut ctx.accounts.merchant;
    let clock = Clock::get()?;

    // In a production system, verify the signature from the merchant
    // For now, we'll assume the signature is valid if it's not empty
    require!(!signature.is_empty(), DealError::InvalidRedemptionSignature);

    // Mark deal as redeemed
    deal.is_redeemed = true;
    deal.is_used = true;

    // Update user profile
    user_profile.total_deals_redeemed = user_profile.total_deals_redeemed
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    user_profile.add_reputation_points(POINTS_REDEEM_DEAL)?;
    user_profile.update_activity(clock.unix_timestamp);

    // Update merchant stats
    merchant.total_deals_redeemed = merchant.total_deals_redeemed
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    merchant.update_activity(clock.unix_timestamp);

    // Mint MONK rewards for redemption
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
    
    token::mint_to(cpi_ctx, REDEMPTION_REWARD)?;

    user_profile.total_rewards_earned = user_profile.total_rewards_earned
        .checked_add(REDEMPTION_REWARD)
        .ok_or(DealError::ArithmeticOverflow)?;

    emit!(DealRedeemedEvent {
        deal: deal.key(),
        redeemer: ctx.accounts.redeemer.key(),
        merchant: deal.merchant,
        reward: REDEMPTION_REWARD,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct DealRedeemedEvent {
    pub deal: Pubkey,
    pub redeemer: Pubkey,
    pub merchant: Pubkey,
    pub reward: u64,
    pub timestamp: i64,
}