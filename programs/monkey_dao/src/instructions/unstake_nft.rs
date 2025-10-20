use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct UnstakeNft<'info> {
    #[account(
        mut,
        constraint = stake.owner == owner.key() @ DealError::NotNftOwner,
        constraint = stake.is_active @ DealError::NftNotStaked,
        close = owner
    )]
    pub stake: Account<'info, Stake>,

    pub nft_mint: Account<'info, Mint>,

    /// Owner's NFT token account
    #[account(
        mut,
        constraint = owner_nft_account.mint == nft_mint.key(),
        constraint = owner_nft_account.owner == owner.key()
    )]
    pub owner_nft_account: Account<'info, TokenAccount>,

    /// Escrow token account holding staked NFT
    #[account(
        mut,
        seeds = [ESCROW_SEED, b"nft", nft_mint.key().as_ref()],
        bump
    )]
    pub escrow_nft_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: PDA authority for escrow
    #[account(
        seeds = [b"stake_authority"],
        bump
    )]
    pub stake_authority: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, owner.key().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub token_program: Program<'info, Token>,
}

pub fn unstake_nft(ctx: Context<UnstakeNft>) -> Result<()> {
    let stake = &mut ctx.accounts.stake;
    let clock = Clock::get()?;

    // Mark as inactive
    stake.is_active = false;

    // Transfer NFT back to owner
    let authority_bump = ctx.bumps.stake_authority;
    let authority_seeds = &[b"stake_authority".as_ref(), &[authority_bump]];
    let signer = &[&authority_seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_nft_account.to_account_info(),
        to: ctx.accounts.owner_nft_account.to_account_info(),
        authority: ctx.accounts.stake_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, 1)?;

    // Update user profile
    ctx.accounts.user_profile.update_activity(clock.unix_timestamp);

    emit!(NftUnstakedEvent {
        stake: stake.key(),
        nft_mint: stake.nft_mint,
        owner: stake.owner,
        staking_duration_days: stake.get_staking_duration_days(clock.unix_timestamp),
        total_rewards_claimed: stake.total_rewards_claimed,
        timestamp: clock.unix_timestamp,
    });

    // Stake account will be closed automatically due to close constraint
    Ok(())
}

#[event]
pub struct NftUnstakedEvent {
    pub stake: Pubkey,
    pub nft_mint: Pubkey,
    pub owner: Pubkey,
    pub staking_duration_days: i64,
    pub total_rewards_claimed: u64,
    pub timestamp: i64,
}