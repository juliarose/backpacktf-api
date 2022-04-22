use serde::{Deserialize, Serialize};
pub use tf2_price::{ListingCurrencies, USDCurrencies};
use std::fmt;

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum Currencies {
    InGame(ListingCurrencies),
    Cash(USDCurrencies)
}

impl fmt::Display for Currencies {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currencies::InGame(currencies) => write!(f, "{}", currencies),
            Currencies::Cash(currencies) => write!(f, "{}", currencies),
        }
    }
}

impl Currencies {
    
    pub fn is_in_game(&self) -> bool {
        match self {
            Currencies::InGame(_) => true,
            _ => false,
        }
    }
}

impl PartialEq<ListingCurrencies> for Currencies {
    
    fn eq(&self, other: &ListingCurrencies) -> bool {
        match self {
            Currencies::InGame(currencies) => currencies == other,
            _ => false,
        }
    }
}

impl PartialEq<tf2_price::Currencies> for Currencies {
    
    fn eq(&self, other: &tf2_price::Currencies) -> bool {
        match self {
            Currencies::InGame(currencies) => currencies == other,
            _ => false,
        }
    }
}