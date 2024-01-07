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
use backpacktf_api::BackpackAPI;
use backpacktf_api::request::{BuyListingItem, CreateListing};
use backpacktf_api::tf2_price::{Currencies, scrap};
use backpacktf_api::tf2_enum::{Quality, KillstreakTier};

let backpacktf = BackpackAPI::builder()
    .key("key")
    .token("token")
    .build();
let currencies = Currencies { keys: 0, metal: scrap!(1) };

match backpacktf.create_listing(&CreateListing::Buy {
    currencies,
    item: BuyListingItem {
        defindex: 1071,
        quality: Quality::Strange,
        killstreak_tier: Some(KillstreakTier::Professional),
        ..BuyListingItem::default()
    },
    details: Some(format!("Buying Golden Frying Pan for {currencies}!")),
    buyout: true,
    offers: true,
}).await {
    Ok(response) => println!("Listing created successfully: {response:?}"),
    Err(error) => println!("Error creating listing: {error}"),
}
```

## License

MIT