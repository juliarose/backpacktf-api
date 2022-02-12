use serde::{Deserialize, Serialize};
use tf2_price::Currencies;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateListing {
    pub details: Option<String>,
    pub currencies: Currencies,
}