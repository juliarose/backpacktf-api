use super::Item;
use crate::{SteamID, ListingIntent};
use crate::time::ServerTime;
use crate::response::{listing::UserAgent, currencies::ResponseCurrencies};
use crate::response::deserializers;
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;

/// A snapshot listing.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    /// The SteamID of the user.
    pub steamid: SteamID,
    /// Whether the listing has offers enabled.
    #[serde(deserialize_with = "deserializers::bool_from_int")]
    pub offers: bool,
    /// Whether the listing has buyout enabled.
    #[serde(deserialize_with = "deserializers::bool_from_int")]
    pub buyout: bool,
    /// The intent of the listing.
    #[serde(deserialize_with = "deserializers::listing_intent_enum_from_str_or_int")]
    pub intent: ListingIntent,
    /// The details of the listing.
    pub details: Option<String>,
    /// The timestamp of the listing.
    #[serde(with = "ts_seconds")]
    pub timestamp: ServerTime,
    /// The bump timestamp of the listing.
    #[serde(with = "ts_seconds")]
    pub bump: ServerTime,
    /// The price of the listing.
    pub price: f32,
    /// The item of the listing.
    pub item: Item,
    /// The currencies of the listing.
    pub currencies: ResponseCurrencies,
    /// The user agent the listing is managed by.
    #[serde(default)]
    pub user_agent: Option<UserAgent>,
}

impl Listing {
    /// Checks if the listing is a buy order.
    pub fn is_buy_order(&self) -> bool {
        self.intent == ListingIntent::Buy
    }
    
    /// Checks if the listing is a sell order.
    pub fn is_sell_order(&self) -> bool {
        self.intent == ListingIntent::Sell
    }
    
    /// Checks if the listing is automatic.
    pub fn is_automatic(&self) -> bool {
        self.user_agent.is_some()
    }
}