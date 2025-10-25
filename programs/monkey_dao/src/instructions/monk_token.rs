use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token},
    metadata::{
        create_metadata_accounts_v3, CreateMetadataAccountsV3,
        mpl_token_metadata::types::DataV2, Metadata,
    },
};
use crate::state::*;
use crate::constants::*;
use crate::ANCHOR_DISCRIMINATOR;

#[derive(Accounts)]
pub struct InitializeMonkMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR + PlatformConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, PlatformConfig>,
    
    #[account(
        init,
        payer = authority,
        mint::decimals = MONK_DECIMALS,
        mint::authority = config,
        mint::freeze_authority = config,
        seeds = [b"monk_mint"],
        bump,
    )]
    pub monk_mint: Account<'info, Mint>,
    
    /// CHECK: Metadata account derived via PDA
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            monk_mint.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,
    
    /// CHECK: Platform wallet to receive fees
    pub platform_wallet: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_monk_mint(ctx: Context<InitializeMonkMint>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    config.authority = ctx.accounts.authority.key();
    config.monk_mint = ctx.accounts.monk_mint.key();
    config.platform_wallet = ctx.accounts.platform_wallet.key();
    config.platform_fee_bps = PLATFORM_FEE_BPS;
    config.staking_reward_rate = STAKING_REWARD_RATE;
    config.bump = ctx.bumps.config;
    
    // PDA seeds for config signer
    let config_seeds: &[&[u8]] = &[
        b"config",
        &[config.bump],
    ];
    let config_signer = &[config_seeds];
    
    // Create metadata account for MONK mint
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.monk_mint.to_account_info(),
                mint_authority: ctx.accounts.config.to_account_info(),
                update_authority: ctx.accounts.config.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            config_signer,
        ),
        DataV2 {
            name: "MONK Token".to_string(),
            symbol: "MONK".to_string(),
            uri: "https://your-metadata-uri.com/monk.json".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true, // mutable
        true, // update authority is signer
        None,
    )?;
    
    msg!("✅ MONK token mint initialized successfully");
    msg!("Mint address: {}", ctx.accounts.monk_mint.key());
    msg!("Decimals: {}", MONK_DECIMALS);
    msg!(
        "Staking reward rate: {} MONK per day",
        STAKING_REWARD_RATE / 10u64.pow(MONK_DECIMALS as u32)
    );
    
    Ok(())
}
