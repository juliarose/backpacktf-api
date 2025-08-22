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
use tf2_price::{Currencies, ref_to_weps};
use tf2_enum::{Quality, KillstreakTier};

let backpacktf = BackpackAPI::builder()
    .key("key")
    .token("token")
    .build();
let currencies = Currencies {
    keys: 0,
    // Metal is defined as an integer as the number of weapons
    // but serializes to { "metal": 0.11 }
    weapons: ref_to_weps!(0.11),
};
let item = BuyListingItem::new(1071, Quality::Strange)
    .killstreak_tier(KillstreakTier::Professional);

match backpacktf.create_listing(&CreateListing::Buy {
    currencies,
    item,
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