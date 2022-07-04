use backpacktf_api::{BackpackAPI, error::Error};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let backpacktf = BackpackAPI::builder()
        .key(&env::var("KEY").unwrap())
        .token(&env::var("TOKEN").unwrap())
        .build();
    
    let (listings, cursor) = backpacktf.get_archived_listings(0, 10).await?;
    
    println!("Listing {:?}", listings);
    println!("Total {} archived listings", cursor.total);
    
    Ok(())
}