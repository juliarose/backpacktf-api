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