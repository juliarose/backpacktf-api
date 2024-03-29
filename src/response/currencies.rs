//! Currencies used in responses.

use std::fmt;
use std::cmp::{Ord, Ordering};
use serde::{Deserialize, Serialize};
pub use tf2_price::{ListingCurrencies, USDCurrencies};

/// Currencies from a response.
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Copy, Debug)]
#[serde(untagged)]
pub enum ResponseCurrencies {
    /// In-game currencies.
    InGame(ListingCurrencies),
    /// Cash currencies for marketplace.tf cross-listings.
    Cash(USDCurrencies)
}

impl fmt::Display for ResponseCurrencies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseCurrencies::InGame(currencies) => write!(f, "{currencies}"),
            ResponseCurrencies::Cash(currencies) => write!(f, "{currencies}"),
        }
    }
}

impl PartialOrd for ResponseCurrencies {
    fn partial_cmp(&self, other: &ResponseCurrencies) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for ResponseCurrencies {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            ResponseCurrencies::InGame(currencies) => {
                if let ResponseCurrencies::InGame(other) = other {
                    currencies.cmp(other)
                } else {
                    Ordering::Less
                }
            },
            ResponseCurrencies::Cash(currencies) => {
                if let ResponseCurrencies::Cash(other) = other {
                    currencies.cmp(other)
                } else {
                    // prefer in-game currencies
                    Ordering::Greater
                }
            },
        }
    }
}

impl ResponseCurrencies {
    /// Checks if the currencies are in-game currencies.
    pub fn is_in_game(&self) -> bool {
        matches!(self, ResponseCurrencies::InGame(_))
    }
}

impl PartialEq<ListingCurrencies> for ResponseCurrencies {
    fn eq(&self, other: &ListingCurrencies) -> bool {
        match self {
            ResponseCurrencies::InGame(currencies) => currencies == other,
            _ => false,
        }
    }
}

impl PartialEq<tf2_price::Currencies> for ResponseCurrencies {
    fn eq(&self, other: &tf2_price::Currencies) -> bool {
        match self {
            ResponseCurrencies::InGame(currencies) => currencies == other,
            _ => false,
        }
    }
}