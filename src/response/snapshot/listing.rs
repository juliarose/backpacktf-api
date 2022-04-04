use serde::{Serialize, Deserialize};
use crate::{
    ListingIntent,
    time::ServerTime,
    response::currencies::Currencies,
    response::deserializers::{
        bool_from_int,
        listing_intent_enum_from_str
    },
};
use super::Item;
use chrono::serde::ts_seconds;
use steamid_ng::SteamID;

pub use crate::response::listing::UserAgent;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Listing {
    pub steamid: SteamID,
    #[serde(deserialize_with = "bool_from_int")]
    pub offers: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub buyout: bool,
    #[serde(deserialize_with = "listing_intent_enum_from_str")]
    pub intent: ListingIntent,
    pub details: Option<String>,
    #[serde(with = "ts_seconds")]
    pub timestamp: ServerTime,
    #[serde(with = "ts_seconds")]
    pub bump: ServerTime,
    pub price: f32,
    pub item: Item,
    pub currencies: Currencies,
    pub user_agent: Option<UserAgent>,
}

impl Listing {
    
    pub fn is_buy_order(&self) -> bool {
        self.intent == ListingIntent::Buy
    }
    
    pub fn is_sell_order(&self) -> bool {
        self.intent == ListingIntent::Sell
    }
    
    pub fn is_automatic(&self) -> bool {
        self.user_agent.is_some()
    }
}