use backpacktf_api::BackpackAPI;
use backpacktf_api::request::{BuyListingItem, CreateListing};
use backpacktf_api::error::Error;
use tf2_price::{Currencies, ref_to_weps};
use tf2_enum::prelude::*;
use tf2_enum::{StrangePartSet, SpellSet};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let backpacktf = BackpackAPI::builder()
        .key(env::var("KEY").unwrap())
        .token(env::var("TOKEN").unwrap())
        .build();
    let currencies = Currencies {
        keys: 0,
        weapons: ref_to_weps!(0.11),
    };
    let item =  BuyListingItem::new(1071, Quality::Unusual)
        .killstreak_tier(KillstreakTier::Professional)
        .strange_parts(StrangePartSet::single(StrangePart::Dominations))
        .spells(SpellSet::double(
            Spell::DieJob,
            Spell::HeadlessHorseshoes,
        ))
        .particle(17)
        .australium(true)
        .festivized(true)
        .strange(true)
        .paint(Paint::PinkAsHell);
    let listing = backpacktf.create_listing(&CreateListing::Buy {
        currencies,
        item,
        details: Some(format!("Buying Golden Frying Pan for {currencies}!")),
        buyout: true,
        offers: true,
    }).await?;
    
    println!("Listing created: {listing:?}");
    
    Ok(())
}