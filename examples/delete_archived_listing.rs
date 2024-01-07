use backpacktf_api::BackpackAPI;
use backpacktf_api::error::Error;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let backpacktf = BackpackAPI::builder()
        .key(&env::var("KEY").unwrap())
        .token(&env::var("TOKEN").unwrap())
        .build();
    let (
        listings,
        cursor,
    ) = backpacktf.get_archived_listings(0, 10).await?;
   
    println!("Listing {listings:?}");
    println!("Total {} archived listings", cursor.total);
    
    if let Some(listing) = listings.into_iter().next() {
        backpacktf.delete_archived_listing(&listing.id).await?;
        
        println!("Deleted archived listing {}", listing.id);
    }
    
    Ok(())
}