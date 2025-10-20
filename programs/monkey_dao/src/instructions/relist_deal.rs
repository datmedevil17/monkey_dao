use anchor_lang::prelude::*;
use crate::errors::*;
use crate::states::*;

#[derive(Accounts)]
pub struct RelistDeal<'info> {
    #[account(
        mut,
        constraint = deal.owner == owner.key() @ DealError::NotDealOwner,
        constraint = !deal.is_used @ DealError::DealAlreadyUsed,
        constraint = !deal.is_redeemed @ DealError::DealAlreadyRedeemed
    )]
    pub deal: Account<'info, Deal>,

    pub owner: Signer<'info>,
}

pub fn relist_deal(ctx: Context<RelistDeal>, new_price: u64) -> Result<()> {
    let deal = &mut ctx.accounts.deal;
    let clock = Clock::get()?;

    require!(new_price > 0, DealError::InvalidPrice);

    let old_price = deal.price;
    deal.price = new_price;

    emit!(DealRelistedEvent {
        deal: deal.key(),
        owner: deal.owner,
        old_price,
        new_price,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct DealRelistedEvent {
    pub deal: Pubkey,
    pub owner: Pubkey,
    pub old_price: u64,
    pub new_price: u64,
    pub timestamp: i64,
}