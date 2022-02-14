# backpacktf-api

Interface for backpack.tf API endpoints.

```rs
use backpacktf_api::{
    BackpackAPI,
    request::listing::create_listing::{self, CreateListing},
    tf2_price::{Currencies, scrap},
    tf2_enum::{Quality, KillstreakTier},
};

let key = "XXXXXXXXXXXXXXXXXXXXXXXX";
let token = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX=";
let backpacktf = BackpackAPI::new(key, token);
let currencies = request::currencies::Currencies {
    keys: 0,
    metal: scrap!(1),
};
let mut item = create_listing::buy_listing::Item::new(1071, Quality::Strange);
let details = Some(format!("Buying Golden Frying Pan for {}!", &currencies));

item.killstreak_tier = Some(KillstreakTier::Professional);

match backpacktf.create_listing(CreateListing::Buy {
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