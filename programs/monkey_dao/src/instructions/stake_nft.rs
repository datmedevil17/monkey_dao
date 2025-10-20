use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct StakeNft<'info> {
    #[account(
        init,
        payer = owner,
        space = Stake::LEN,
        seeds = [STAKE_SEED, nft_mint.key().as_ref()],
        bump
    )]
    pub stake: Account<'info, Stake>,

    pub nft_mint: Account<'info, Mint>,

    /// Owner's NFT token account
    #[account(
        mut,
        constraint = owner_nft_account.mint == nft_mint.key(),
        constraint = owner_nft_account.owner == owner.key(),
        constraint = owner_nft_account.amount == 1 @ DealError::NotNftOwner
    )]
    pub owner_nft_account: Account<'info, TokenAccount>,

    /// Escrow token account to hold staked NFT
    #[account(
        init,
        payer = owner,
        token::mint = nft_mint,
        token::authority = stake_authority,
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
    pub system_program: Program<'info, System>,
}

pub fn stake_nft(ctx: Context<StakeNft>) -> Result<()> {
    let stake = &mut ctx.accounts.stake;
    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // Transfer NFT to escrow
    let cpi_accounts = Transfer {
        from: ctx.accounts.owner_nft_account.to_account_info(),
        to: ctx.accounts.escrow_nft_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, 1)?;

    // Initialize stake
    stake.nft_mint = ctx.accounts.nft_mint.key();
    stake.owner = ctx.accounts.owner.key();
    stake.staked_at = clock.unix_timestamp;
    stake.last_claim_at = clock.unix_timestamp;
    stake.total_rewards_claimed = 0;
    stake.is_active = true;
    stake.bump = ctx.bumps.stake;

    // Update user profile
    user_profile.total_nfts_staked = user_profile.total_nfts_staked
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    user_profile.add_reputation_points(POINTS_STAKE_NFT)?;
    user_profile.update_activity(clock.unix_timestamp);

    emit!(NftStakedEvent {
        stake: stake.key(),
        nft_mint: stake.nft_mint,
        owner: stake.owner,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct NftStakedEvent {
    pub stake: Pubkey,
    pub nft_mint: Pubkey,
    pub owner: Pubkey,
    pub timestamp: i64,
}