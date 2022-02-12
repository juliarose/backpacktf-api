use serde::{Deserialize, Serialize};
use crate::request::serializers::{currencies_into_form};
use super::serializers::buy_listing_item_into_params;
use tf2_price::Currencies;
use super::BuyListingItem;

#[derive(Deserialize, Serialize, Debug, Clone)]
// includes intent when serializing
#[serde(tag = "intent", rename = "buy")]
pub struct BuyListing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    pub buyout: bool,
    pub offers: bool,
    #[serde(serialize_with = "currencies_into_form")]
    pub currencies: Currencies,
    #[serde(serialize_with = "buy_listing_item_into_params")]
    pub item: BuyListingItem,
}