use anchor_lang::prelude::*;
use crate::states::*;

#[derive(Accounts)]
pub struct GetDealInfo<'info> {
    pub deal: Account<'info, Deal>,
}

pub fn get_deal_info(ctx: Context<GetDealInfo>) -> Result<()> {
    let deal = &ctx.accounts.deal;
    let clock = Clock::get()?;

    msg!("=== Deal Information ===");
    msg!("NFT Mint: {}", deal.nft_mint);
    msg!("Owner: {}", deal.owner);
    msg!("Merchant: {}", deal.merchant);
    msg!("Price: {} lamports", deal.price);
    msg!("Location: {}", deal.location);
    msg!("Is Used: {}", deal.is_used);
    msg!("Is Redeemed: {}", deal.is_redeemed);
    msg!("Is Group Deal: {}", deal.is_group_deal);
    msg!("Is Crypto Based: {}", deal.is_crypto_based);
    msg!("Discount: {}%", deal.discount_percentage);
    msg!("Average Rating: {:.2}", deal.average_rating());
    msg!("Total Ratings: {}", deal.total_ratings);
    msg!("Times Sold: {}", deal.times_sold);
    msg!("Current Supply: {}/{}", deal.current_supply, deal.max_supply);
    msg!("Is Expired: {}", deal.is_expired(clock.unix_timestamp));
    
    if let Some(ref prices) = deal.group_prices {
        msg!("=== Group Prices ===");
        msg!("  For 2: {} lamports", prices.price_for_2);
        msg!("  For 4: {} lamports", prices.price_for_4);
        msg!("  For 8: {} lamports", prices.price_for_8);
    }
    
    if deal.is_crypto_based {
        if let Some(ref name) = deal.event_name {
            msg!("Event: {}", name);
        }
        if let Some(ref desc) = deal.event_description {
            msg!("Description: {}", desc);
        }
    }

    Ok(())
}