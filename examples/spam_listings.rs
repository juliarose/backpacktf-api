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
    let mut item = request::BuyListingItem::new(1071, Quality::Strange);
    
    item.killstreak_tier = Some(KillstreakTier::Professional);
    
    let to_create = (0..1200)
        .map(|_| request::CreateListing::Buy {
            item,
            currencies,
            details: Some(format!("Buying Golden Frying Pan for {}!", &currencies)),
            buyout: true,
            offers: true,
        })
        .collect::<Vec<_>>();
    let (
        _listings,
        _error,
    ) = backpacktf.create_listings_chunked(&to_create).await;
    
    Ok(())
}