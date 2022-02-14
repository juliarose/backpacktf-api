use serde::{Deserialize, Serialize};
use tf2_price::Currencies;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct UpdateListing {
    pub details: Option<String>,
    pub currencies: Currencies,
}