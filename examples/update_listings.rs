use backpacktf_api::BackpackAPI;
use backpacktf_api::request::UpdateListing;
use backpacktf_api::error::Error;
use tf2_price::{Currencies, scrap};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let backpacktf = BackpackAPI::builder()
        .key(&env::var("KEY").unwrap())
        .token(&env::var("TOKEN").unwrap())
        .build();
    let listing = backpacktf.update_listings(&[UpdateListing {
            id: "440_76561198080179568_86755d7981f2b4ffb983b9d054ec0c27".into(),
            currencies: Currencies {
                keys: 0,
                metal: scrap!(3),
            },
            details: Some("yup".into()),
        }]).await?;
    
    println!("Listings updated: {listing:?}");
    
    Ok(())
}