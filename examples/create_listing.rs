use backpacktf_api::BackpackAPI;
use backpacktf_api::request::{BuyListingItem, CreateListing};
use backpacktf_api::error::Error;
use tf2_price::{Currencies, scrap};
use tf2_enum::{Quality, KillstreakTier};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let backpacktf = BackpackAPI::builder()
        .key(&env::var("KEY").unwrap())
        .token(&env::var("TOKEN").unwrap())
        .build();
    let currencies = Currencies {
        keys: 0,
        weapons: scrap!(1),
    };
    let listing = backpacktf.create_listing(&CreateListing::Buy {
        currencies,
        item: BuyListingItem {
            defindex: 1071,
            quality: Quality::Strange,
            killstreak_tier: Some(KillstreakTier::Professional),
            ..Default::default()
        },
        details: Some(format!("Buying Golden Frying Pan for {currencies}!")),
        buyout: true,
        offers: true,
    }).await?;
    
    println!("Listing created: {listing:?}");
    
    Ok(())
}