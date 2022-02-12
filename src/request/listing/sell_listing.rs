use serde::{Deserialize, Serialize};
use tf2_price::Currencies;
use crate::request::serializers::{as_string};

#[derive(Deserialize, Serialize, Debug, Clone)]
// includes intent when serializing
#[serde(tag = "intent", rename = "sell")]
pub struct SellListing {
    #[serde(serialize_with = "as_string")]
    pub id: u64,
    pub buyout: bool,
    pub offers: bool,
    pub details: Option<String>,
    pub currencies: Currencies,
}