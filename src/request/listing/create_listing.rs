use serde::{Deserialize, Serialize};
use super::sell_listing::SellListing;
use super::buy_listing::BuyListing;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum CreateListing {
    Sell(SellListing),
    Buy(BuyListing),
}
