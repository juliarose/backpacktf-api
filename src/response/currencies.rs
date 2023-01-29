use serde::{Deserialize, Serialize};
pub use tf2_price::{ListingCurrencies, USDCurrencies};
use std::{fmt, cmp::{Ord, Ordering}};

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Copy, Debug)]
#[serde(untagged)]
pub enum Currencies {
    InGame(ListingCurrencies),
    Cash(USDCurrencies)
}

impl fmt::Display for Currencies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currencies::InGame(currencies) => write!(f, "{currencies}"),
            Currencies::Cash(currencies) => write!(f, "{currencies}"),
        }
    }
}

impl PartialOrd for Currencies {
    fn partial_cmp(&self, other: &Currencies) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for Currencies {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Currencies::InGame(currencies) => {
                if let Currencies::InGame(other) = other {
                    currencies.cmp(other)
                } else {
                    Ordering::Less
                }
            },
            Currencies::Cash(currencies) => {
                if let Currencies::Cash(other) = other {
                    currencies.cmp(other)
                } else {
                    // prefer in-game currencies
                    Ordering::Greater
                }
            },
        }
    }
}

impl Currencies {
    pub fn is_in_game(&self) -> bool {
        matches!(self, Currencies::InGame(_))
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