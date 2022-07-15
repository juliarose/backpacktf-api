use backpacktf_api::{
    BackpackAPI,
    error::Error,
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
    let (listings, error) = backpacktf.get_all_listings(0).await;
    
    if let Some(error) = error {
        panic!("{}", error);
    }
    
    println!("Total listings: {}", listings.len());
    
    Ok(())
}