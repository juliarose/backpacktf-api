use backpacktf_api::BackpackAPI;
use backpacktf_api::request::UpdateListing;
use backpacktf_api::error::Error;
use std::env;
use tf2_price::{Currencies, ref_to_weps};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let backpacktf = BackpackAPI::builder()
        .key(env::var("KEY").unwrap())
        .token(env::var("TOKEN").unwrap())
        .build();
    let updates = [
        UpdateListing {
            id: "440_76561198080179568_192a94e963374b02cc25081eeb36a13b".into(),
            currencies: Currencies {
                keys: 0,
                weapons: ref_to_weps!(0.33),
            },
            details: Some("Buying".into()),
        },
    ];
    let updates = backpacktf.update_listings(&updates).await?;
    
    for result in updates {
        match result {
            Ok(listing) => {
                println!("Listing updated: {listing:?}");
            },
            Err(error) => {
                println!("Error updating listing {}: {}", error.query.id, error.message)
            },
        }
    }
    
    Ok(())
}