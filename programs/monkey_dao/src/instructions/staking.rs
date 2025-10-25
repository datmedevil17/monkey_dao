use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer, transfer, MintTo, mint_to},
};
use crate::state::*;
use crate:: ANCHOR_DISCRIMINATOR;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct StakeNFT<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump = listing.bump,
        constraint = !listing.is_used @ ErrorCode::CannotStakeUsedCoupon,
    )]
    pub listing: Account<'info, Listing>,
    
    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = owner,
        constraint = owner_token_account.amount == 1 @ ErrorCode::Unauthorized,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR + StakeAccount::INIT_SPACE,
        seeds = [b"stake", nft_mint.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        init,
        payer = owner,
        associated_token::mint = nft_mint,
        associated_token::authority = stake_account,
    )]
    pub stake_vault: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"user_stats", owner.key().as_ref()],
        bump = user_stats.bump,
    )]
    pub user_stats: Account<'info, UserStats>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnstakeNFT<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"stake", nft_mint.key().as_ref()],
        bump = stake_account.bump,
        constraint = stake_account.owner == owner.key() @ ErrorCode::Unauthorized,
        constraint = stake_account.is_active @ ErrorCode::NotStaked,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = stake_account,
    )]
    pub stake_vault: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, PlatformConfig>,
    
    #[account(
        mut,
        constraint = monk_mint.key() == config.monk_mint @ ErrorCode::Unauthorized
    )]
    pub monk_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = monk_mint,
        associated_token::authority = owner,
    )]
    pub owner_monk_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"user_stats", owner.key().as_ref()],
        bump = user_stats.bump,
    )]
    pub user_stats: Account<'info, UserStats>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimStakingRewards<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"stake", nft_mint.key().as_ref()],
        bump = stake_account.bump,
        constraint = stake_account.owner == owner.key() @ ErrorCode::Unauthorized,
        constraint = stake_account.is_active @ ErrorCode::NotStaked,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, PlatformConfig>,
    
    #[account(
        mut,
        constraint = monk_mint.key() == config.monk_mint @ ErrorCode::Unauthorized
    )]
    pub monk_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = monk_mint,
        associated_token::authority = owner,
    )]
    pub owner_monk_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"user_stats", owner.key().as_ref()],
        bump = user_stats.bump,
    )]
    pub user_stats: Account<'info, UserStats>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn stake_nft(ctx: Context<StakeNFT>) -> Result<()> {
    let clock = Clock::get()?;
    let stake_account = &mut ctx.accounts.stake_account;
    
    stake_account.nft_mint = ctx.accounts.nft_mint.key();
    stake_account.owner = ctx.accounts.owner.key();
    stake_account.staked_at = clock.unix_timestamp;
    stake_account.last_claim = clock.unix_timestamp;
    stake_account.total_rewards_claimed = 0;
    stake_account.is_active = true;
    stake_account.bump = ctx.bumps.stake_account;

    // Transfer NFT to stake vault
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.owner_token_account.to_account_info(),
            to: ctx.accounts.stake_vault.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        },
    );
    transfer(transfer_ctx, 1)?;

    // Update user stats
    let user_stats = &mut ctx.accounts.user_stats;
    user_stats.nfts_staked = user_stats.nfts_staked.checked_add(1)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    msg!("NFT staked successfully");
    Ok(())
}

pub fn unstake_nft(ctx: Context<UnstakeNFT>) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    let clock = Clock::get()?;
    
    // Calculate and mint pending rewards
    let rewards = stake_account.calculate_rewards(
        clock.unix_timestamp,
        ctx.accounts.config.staking_reward_rate,
    )?;

    if rewards > 0 {
        let config_seeds = &[
            b"config",
            &[ctx.accounts.config.bump][..],
        ];
        let config_signer = &[&config_seeds[..]];

        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.monk_mint.to_account_info(),
                to: ctx.accounts.owner_monk_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            config_signer,
        );
        mint_to(mint_ctx, rewards)?;

        stake_account.total_rewards_claimed = stake_account.total_rewards_claimed
            .checked_add(rewards)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
            
        // Update user stats
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.total_monk_earned = user_stats.total_monk_earned
            .checked_add(rewards)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    }

    // Transfer NFT back to owner
    let binding = ctx.accounts.nft_mint.key();
    let seeds = &[
        b"stake",
        binding.as_ref(),
        &[stake_account.bump],
    ];
    let signer = &[&seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.stake_vault.to_account_info(),
            to: ctx.accounts.owner_token_account.to_account_info(),
            authority: stake_account.to_account_info(),
        },
        signer,
    );
    transfer(transfer_ctx, 1)?;

    stake_account.is_active = false;

    // Update user stats
    let user_stats = &mut ctx.accounts.user_stats;
    user_stats.nfts_staked = user_stats.nfts_staked.checked_sub(1)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    msg!("NFT unstaked successfully. Total rewards claimed: {}", rewards);
    Ok(())
}

pub fn claim_staking_rewards(ctx: Context<ClaimStakingRewards>) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    let clock = Clock::get()?;
    
    let rewards = stake_account.calculate_rewards(
        clock.unix_timestamp,
        ctx.accounts.config.staking_reward_rate,
    )?;

    require!(rewards > 0, ErrorCode::InsufficientTimeElapsed);

    // Mint MONK tokens
    let config_seeds: &[&[u8]] = &[
        b"config",
        &[ctx.accounts.config.bump],
    ];
    let config_signer = &[config_seeds];

    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.monk_mint.to_account_info(),
            to: ctx.accounts.owner_monk_account.to_account_info(),
            authority: ctx.accounts.config.to_account_info(),
        },
        config_signer,
    );
    mint_to(mint_ctx, rewards)?;

    // Update stake account
    stake_account.last_claim = clock.unix_timestamp;
    stake_account.total_rewards_claimed = stake_account.total_rewards_claimed
        .checked_add(rewards)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Update user stats
    let user_stats = &mut ctx.accounts.user_stats;
    user_stats.total_monk_earned = user_stats.total_monk_earned
        .checked_add(rewards)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    msg!("Staking rewards claimed: {}", rewards);
    Ok(())
}