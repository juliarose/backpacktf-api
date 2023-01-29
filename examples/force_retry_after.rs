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
    
    loop {
        if let Err(error) = backpacktf.get_snapshot("Strange Kritzkreig").await {
            match error {
                Error::TooManyRequests(retry_after) => {
                    println!("RETRY!!!! {retry_after}");
                }
                error => {
                    println!("{error}");
                }
            }
            
            break;
        }
    }
    
    Ok(())
}