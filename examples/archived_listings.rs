use backpacktf_api::{
    BackpackAPI,
    request,
    error::Error,
    tf2_price::{Currencies, scrap, refined},
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
        metal: refined!(2),
    };
    let details = Some(format!("Buying Golden Frying Pan for {}!", &currencies));
    let item = request::BuyListingItem::new(1071, Quality::Strange);
    let listing = backpacktf.create_listing(&request::CreateListing::Buy {
        item,
        currencies,
        details,
        buyout: true,
        offers: true,
    }).await?;
    println!("Archived listing: {:?}", listing);
    
    let (listings, _) = backpacktf.get_all_listings().await;
    let (archived_listings, _) = backpacktf.get_all_archived_listings().await;
    
    for listing in listings {
        println!("Listing: {:?}", listing);
    }
    
    for listing in archived_listings {
        println!("Archived listing: {:?}", listing);
    }
    
    Ok(())
}