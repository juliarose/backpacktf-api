use backpacktf_api::{
    BackpackAPI,
    SteamID,
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
    let inventory = backpacktf.get_user_v1(&SteamID::from(76561198080179568)).await?;
    let details = Some(format!("Buying Golden Frying Pan for {}!", &currencies));
    let item = request::BuyListingItem::new(1071, Quality::Strange);
    
    println!("{}", inventory.currencies());
    // let listing = backpacktf.create_listing(&request::CreateListing::Buy {
    //     item,
    //     currencies,
    //     details,
    //     buyout: true,
    //     offers: true,
    // }).await?;
    // println!("Archived listing: {:?}", listing);
    
    // let (listings, _) = backpacktf.get_all_listings().await;
    
    // for listing in listings {
    //     println!("Listing: {:?}", listing);
    // }
    
    println!("Inventory {:?}", inventory);
    let (archived_listings, _) = backpacktf.get_all_archived_listings().await;
    let archived_listing = archived_listings.into_iter().next().unwrap();
    
    // let updated_listing = backpacktf.update_archived_listing(
    //     &archived_listing.id,
    //     Some("Yes".into()),
    //     &currencies,
    // ).await?;
    // let updated_listings = backpacktf.update_archived_listings(
    //     &vec![
    //         request::UpdateListing {
    //             id: archived_listing.id.clone(),
    //             currencies,
    //             details: Some("Yeah".into()),
    //         }
    //     ]
    // ).await?;
    
    // println!("{:?}", updated_listings);
    // println!("Updated Listing: {:?}", updated_listing);
    
    // let (count, error) = backpacktf.delete_archived_listings_chunked(&vec![archived_listing.id]).await;
    backpacktf.delete_archived_listing(&archived_listing.id).await;
    // println!("{:?}", error);
    // println!("Deleted {} archived listing(s)", count);
    
    Ok(())
}