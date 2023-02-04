# backpacktf-api

Interface for backpack.tf API endpoints.

## Installation

### Cargo.toml
```
[dependencies]
backpacktf-api = { git = "https://github.com/juliarose/backpacktf-api" }
```

### With websocket
```
[dependencies]
backpacktf-api = { git = "https://github.com/juliarose/backpacktf-api", features = ["websocket"] }
```

## Usage

```rs
use backpacktf_api::{
    BackpackAPI,
    request::{BuyListingItem, CreateListing},
    tf2_price::{Currencies, scrap},
    tf2_enum::{Quality, KillstreakTier},
};

let backpacktf = BackpackAPI::builder()
    .key("key")
    .token("token")
    .build();
let currencies = Currencies { keys: 0, metal: scrap!(1) };
let details = Some(format!("Buying Golden Frying Pan for {currencies}!"));
let mut item = BuyListingItem::new(1071, Quality::Strange);

item.killstreak_tier = Some(KillstreakTier::Professional);

match backpacktf.create_listing(&CreateListing::Buy {
    item,
    currencies,
    details,
    buyout: true,
    offers: true,
}).await {
    Ok(response) => println!("Listing created successfully: {response:?}"),
    Err(error) => println!("Error creating listing: {error}"),
}
```

## License

MIT