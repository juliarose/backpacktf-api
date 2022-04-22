use serde::{Deserialize, Serialize};
pub use tf2_price::{ListingCurrencies, USDCurrencies};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum Currencies {
    InGame(ListingCurrencies),
    Cash(USDCurrencies)
}

impl Currencies {
    
    pub fn is_in_game(&self) -> bool {
        match self {
            Currencies::InGame(_) => true,
            _ => false,
        }
    }
}

// impl<'de> Deserialize<'de> for Currencies {
//     fn deserialize<D>(deserializer: D) -> Result<Currencies, D::Error>
//         where D: Deserializer<'de>
//     {
//         match ListingCurrencies::deserialize(deserializer) {
//             Ok(currencies) => {
//                 Ok(Currencies::InGame(currencies))
//             },
//             Err(error) => {
//                 if let Ok(currencies) = USDCurrencies::deserialize(deserializer) {
//                     Ok(Currencies::Cash(currencies))
//                 } else {
//                     Err(error)
//                 }
//             }
//         }
//     }
// }