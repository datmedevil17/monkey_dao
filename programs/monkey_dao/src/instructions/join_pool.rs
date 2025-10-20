use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use crate::constants::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct JoinPool<'info> {
    #[account(
        mut,
        constraint = pool.is_active @ DealError::PoolNotActive,
        constraint = !pool.is_executed @ DealError::PoolAlreadyExecuted,
        constraint = !pool.is_expired(Clock::get()?.unix_timestamp) @ DealError::PoolExpired
    )]
    pub pool: Account<'info, Pool>,

    pub deal: Account<'info, Deal>,

    #[account(mut)]
    pub participant: Signer<'info>,

    /// Pool escrow account to hold funds
    #[account(
        mut,
        seeds = [ESCROW_SEED, pool.key().as_ref()],
        bump
    )]
    /// CHECK: PDA escrow account
    pub pool_escrow: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = participant,
        space = UserProfile::LEN,
        seeds = [USER_PROFILE_SEED, participant.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    /// MONK token mint for rewards
    #[account(mut)]
    pub monk_token_mint: Account<'info, Mint>,

    /// User's MONK token account for rewards
    #[account(
        mut,
        constraint = user_monk_account.mint == monk_token_mint.key(),
        constraint = user_monk_account.owner == participant.key()
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

pub fn join_pool(ctx: Context<JoinPool>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // Validations
    require!(
        !pool.has_participant(&ctx.accounts.participant.key()),
        DealError::AlreadyJoinedPool
    );
    require!(amount > 0, DealError::InsufficientPoolContribution);
    
    let new_amount = pool.current_amount
        .checked_add(amount)
        .ok_or(DealError::ArithmeticOverflow)?;
    require!(new_amount <= pool.target_amount, DealError::PoolTargetExceeded);

    // Transfer funds to escrow
    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.participant.to_account_info(),
            to: ctx.accounts.pool_escrow.to_account_info(),
        },
    );
    system_program::transfer(transfer_ctx, amount)?;

    // Add participant to pool
    pool.participants.push(Participant {
        user: ctx.accounts.participant.key(),
        contribution: amount,
    });
    pool.current_participants = pool.current_participants
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    pool.current_amount = new_amount;

    // Initialize user profile if needed
    if user_profile.owner == Pubkey::default() {
        user_profile.owner = ctx.accounts.participant.key();
        user_profile.created_at = clock.unix_timestamp;
        user_profile.bump = ctx.bumps.user_profile;
    }

    // Update user profile
    user_profile.total_pools_joined = user_profile.total_pools_joined
        .checked_add(1)
        .ok_or(DealError::ArithmeticOverflow)?;
    user_profile.add_reputation_points(POINTS_JOIN_POOL)?;
    user_profile.update_activity(clock.unix_timestamp);

    // Mint MONK rewards for joining pool
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
    
    token::mint_to(cpi_ctx, POOL_PARTICIPATION_REWARD)?;

    user_profile.total_rewards_earned = user_profile.total_rewards_earned
        .checked_add(POOL_PARTICIPATION_REWARD)
        .ok_or(DealError::ArithmeticOverflow)?;

    emit!(PoolJoinedEvent {
        pool: pool.key(),
        participant: ctx.accounts.participant.key(),
        contribution: amount,
        current_amount: pool.current_amount,
        current_participants: pool.current_participants,
        is_target_reached: pool.is_target_reached(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct PoolJoinedEvent {
    pub pool: Pubkey,
    pub participant: Pubkey,
    pub contribution: u64,
    pub current_amount: u64,
    pub current_participants: u8,
    pub is_target_reached: bool,
    pub timestamp: i64,
}