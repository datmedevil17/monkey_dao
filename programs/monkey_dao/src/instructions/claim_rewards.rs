use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        constraint = stake.owner == owner.key() @ DealError::NotNftOwner,
        constraint = stake.is_active @ DealError::NftNotStaked
    )]
    pub stake: Account<'info, Stake>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, owner.key().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, UserProfile>,

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
}

pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let stake = &mut ctx.accounts.stake;
    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // Calculate pending rewards
    let rewards = stake.calculate_pending_rewards(
        clock.unix_timestamp,
        STAKING_REWARD_PER_DAY
    )?;

    require!(rewards > 0, DealError::NoRewardsToClaim);

    // Mint rewards
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
    
    token::mint_to(cpi_ctx, rewards)?;

    // Update stake
    stake.last_claim_at = clock.unix_timestamp;
    stake.total_rewards_claimed = stake.total_rewards_claimed
        .checked_add(rewards)
        .ok_or(DealError::ArithmeticOverflow)?;

    // Update user profile
    user_profile.total_rewards_earned = user_profile.total_rewards_earned
        .checked_add(rewards)
        .ok_or(DealError::ArithmeticOverflow)?;
    user_profile.update_activity(clock.unix_timestamp);

    emit!(RewardsClaimedEvent {
        stake: stake.key(),
        owner: stake.owner,
        rewards,
        total_claimed: stake.total_rewards_claimed,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct RewardsClaimedEvent {
    pub stake: Pubkey,
    pub owner: Pubkey,
    pub rewards: u64,
    pub total_claimed: u64,
    pub timestamp: i64,
}