use serde::{Serialize, Deserialize};
use steamid_ng::SteamID;
use tf2_price::Currencies;
use crate::ListingIntent;
use crate::response::deserializers::listing_intent_enum_from_str;
use chrono::serde::ts_seconds;
use crate::time::ServerTime;
use super::{User, UserAgent, Value, Item};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub id: String,
    pub steamid: SteamID,
    pub appid: u32,
    pub currencies: Currencies,
    pub value: Option<Value>,
    #[serde(default)]
    pub trade_offers_preferred: bool,
    #[serde(default)]
    pub buyout_only: bool,
    pub details: Option<String>,
    #[serde(with = "ts_seconds")]
    pub listed_at: ServerTime,
    #[serde(with = "ts_seconds")]
    pub bumped_at: ServerTime,
    #[serde(deserialize_with = "listing_intent_enum_from_str")]
    pub intent: ListingIntent,
    pub item: Item,
    pub count: u32,
    pub status: String,
    pub user_agent: Option<UserAgent>,
    pub user: Option<User>,
}