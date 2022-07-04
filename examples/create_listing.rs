use backpacktf_api::{
    BackpackAPI,
    request,
    error::Error,
    tf2_price::{Currencies, scrap},
    tf2_enum::{Quality, KillstreakTier},
};
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
        metal: scrap!(1),
    };
    let details = Some(format!("Buying Golden Frying Pan for {}!", &currencies));
    let mut item = request::BuyListingItem::new(1071, Quality::Strange);
    
    item.killstreak_tier = Some(KillstreakTier::Professional);
    
    let listing = backpacktf.create_listing(&request::CreateListing::Buy {
        item,
        currencies,
        details,
        buyout: true,
        offers: true,
    }).await?;
    
    println!("Listing created: {:?}", listing);
    
    Ok(())
}