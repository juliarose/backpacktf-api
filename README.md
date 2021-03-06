# backpacktf-api

Interface for backpack.tf API endpoints.

## Usage

```rs
use backpacktf_api::{
    BackpackAPI,
    request,
    tf2_price::{Currencies, scrap},
    tf2_enum::{Quality, KillstreakTier},
};

let backpacktf = BackpackAPI::builder()
    .key("key")
    .token("token")
    .build();
let currencies = Currencies {
    keys: 0,
    metal: scrap!(1),
};
let details = Some(format!("Buying Golden Frying Pan for {}!", &currencies));
let mut item = request::BuyListingItem::new(1071, Quality::Strange);

item.killstreak_tier = Some(KillstreakTier::Professional);

match backpacktf.create_listing(&request::CreateListing::Buy {
    item,
    currencies,
    details,
    buyout: true,
    offers: true,
}).await {
    Ok(response) => println!("Listing created successfully: {:?}", response),
    Err(error) => println!("Error creating listing: {}", error),
}
```

## License

MIT