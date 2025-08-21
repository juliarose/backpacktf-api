use backpacktf_api::BackpackAPI;
use backpacktf_api::error::Error;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    
    let backpacktf = BackpackAPI::builder()
        .key(env::var("KEY").unwrap())
        .token(env::var("TOKEN").unwrap())
        .build();
    let (
        archived_listings,
        _cursor,
    ) = backpacktf.get_archived_listings(0, 1).await?;
    
    if let Some(archived_listing) = archived_listings.into_iter().next() {
        backpacktf.delete_archived_listing(&archived_listing.id).await?;
    } else {
        println!("No archived listings");
    }
    
    Ok(())
}