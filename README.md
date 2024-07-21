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

```rust
use backpacktf_api::BackpackAPI;
use backpacktf_api::request::{BuyListingItem, CreateListing};
use tf2_price::{Currencies, scrap};
use tf2_enum::{Quality, KillstreakTier};

let backpacktf = BackpackAPI::builder()
    .key("key")
    .token("token")
    .build();
let currencies = Currencies {
    keys: 0,
    // Metal is defined as an integer as the number of weapons
    // but serializes to { "metal": 0.11 }
    weapons: scrap!(1),
};

match backpacktf.create_listing(&CreateListing::Buy {
    currencies,
    item: BuyListingItem {
        defindex: 1071,
        quality: Quality::Strange,
        killstreak_tier: Some(KillstreakTier::Professional),
        ..Default::default()
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