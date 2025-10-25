use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use crate::state::*;
use crate:: ANCHOR_DISCRIMINATOR;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(pool_size: u8)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,
    
    #[account(
        seeds = [b"listing", listing.nft_mint.as_ref()],
        bump = listing.bump,
        constraint = listing.is_group_deal @ ErrorCode::NotGroupDeal,
        constraint = listing.is_active @ ErrorCode::ListingNotActive,
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(
        init,
        payer = initiator,
        space = ANCHOR_DISCRIMINATOR + Pool::INIT_SPACE,
        seeds = [b"pool", listing.key().as_ref(), initiator.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    
    /// CHECK: Escrow account to hold funds
    #[account(
        mut,
        seeds = [b"escrow", pool.key().as_ref()],
        bump,
    )]
    pub escrow: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinPool<'info> {
    #[account(mut)]
    pub participant: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"pool", pool.listing.as_ref(), pool.initiator.as_ref()],
        bump = pool.bump,
        constraint = pool.is_active @ ErrorCode::PoolNotActive,
        constraint = pool.current_participants < pool.pool_size @ ErrorCode::PoolFull,
    )]
    pub pool: Account<'info, Pool>,
    
    #[account(
        init,
        payer = participant,
        space = ANCHOR_DISCRIMINATOR + PoolParticipant::INIT_SPACE,
        seeds = [b"pool_participant", pool.key().as_ref(), participant.key().as_ref()],
        bump
    )]
    pub pool_participant: Account<'info, PoolParticipant>,
    
    /// CHECK: Escrow account to receive funds
    #[account(
        mut,
        seeds = [b"escrow", pool.key().as_ref()],
        bump,
    )]
    pub escrow: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompletePool<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"pool", pool.listing.as_ref(), initiator.key().as_ref()],
        bump = pool.bump,
        constraint = pool.initiator == initiator.key() @ ErrorCode::NotPoolInitiator,
        constraint = pool.current_participants == pool.pool_size @ ErrorCode::PoolNotComplete,
        constraint = pool.is_active @ ErrorCode::PoolNotActive,
    )]
    pub pool: Account<'info, Pool>,
    
    #[account(
        mut,
        seeds = [b"listing", listing.nft_mint.as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,
    
    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = initiator,
        associated_token::mint = nft_mint,
        associated_token::authority = initiator,
    )]
    pub initiator_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Escrow account
    #[account(
        mut,
        seeds = [b"escrow", pool.key().as_ref()],
        bump,
    )]
    pub escrow: UncheckedAccount<'info>,
    
    /// CHECK: Seller to receive payment
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, PlatformConfig>,
    
    /// CHECK: Platform wallet
    #[account(mut)]
    pub platform_wallet: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelPool<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"pool", pool.listing.as_ref(), initiator.key().as_ref()],
        bump = pool.bump,
        constraint = pool.initiator == initiator.key() @ ErrorCode::NotPoolInitiator,
        constraint = pool.is_active @ ErrorCode::PoolNotActive,
    )]
    pub pool: Account<'info, Pool>,
    
    /// CHECK: Escrow account
    #[account(
        mut,
        seeds = [b"escrow", pool.key().as_ref()],
        bump,
    )]
    pub escrow: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_pool(ctx: Context<CreatePool>, pool_size: u8) -> Result<()> {
    require!(
        pool_size == 2 || pool_size == 4 || pool_size == 6,
        ErrorCode::InvalidPoolSize
    );

    let listing = &ctx.accounts.listing;
    let clock = Clock::get()?;
    
    let price_per_person = match pool_size {
        2 => listing.deal_price_2.ok_or(ErrorCode::DealNotAvailable)?,
        4 => listing.deal_price_4.ok_or(ErrorCode::DealNotAvailable)?,
        6 => listing.deal_price_6.ok_or(ErrorCode::DealNotAvailable)?,
        _ => return Err(ErrorCode::InvalidPoolSize.into()),
    };

    let pool = &mut ctx.accounts.pool;
    pool.listing = listing.key();
    pool.initiator = ctx.accounts.initiator.key();
    pool.pool_size = pool_size;
    pool.current_participants = 0;
    pool.price_per_person = price_per_person;
    pool.total_deposited = 0;
    pool.is_active = true;
    pool.is_completed = false;
    pool.participants = Vec::new();
    pool.created_at = clock.unix_timestamp;
    pool.bump = ctx.bumps.pool;

    msg!("Pool created for {} participants at {} lamports per person", pool_size, price_per_person);
    Ok(())
}

pub fn join_pool(ctx: Context<JoinPool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;
    
    // Transfer funds to escrow
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.participant.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
            },
        ),
        pool.price_per_person,
    )?;

    // Record participant
    let pool_participant = &mut ctx.accounts.pool_participant;
    pool_participant.pool = pool.key();
    pool_participant.participant = ctx.accounts.participant.key();
    pool_participant.amount_deposited = pool.price_per_person;
    pool_participant.joined_at = clock.unix_timestamp;
    pool_participant.bump = ctx.bumps.pool_participant;

    // Update pool
    pool.participants.push(ctx.accounts.participant.key());
    pool.current_participants = pool.current_participants.checked_add(1)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    pool.total_deposited = pool.total_deposited.checked_add(pool.price_per_person)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    msg!("Participant joined pool: {}/{}", pool.current_participants, pool.pool_size);
    
    // Check if pool is complete
    if pool.current_participants == pool.pool_size {
        msg!("Pool is now complete!");
    }

    Ok(())
}

pub fn cancel_pool(ctx: Context<CancelPool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    require!(pool.current_participants == 0, ErrorCode::PoolNotActive);

    pool.is_active = false;
    
    msg!("Pool cancelled");
    Ok(())
}