use serde::{Serialize, Deserialize};
use steamid_ng::SteamID;
use tf2_price::Currencies;
use crate::ListingIntent;
use crate::response::deserializers::{
    listing_intent_enum_from_str,
    currencies_from_form
};
use chrono::serde::{ts_seconds};
use crate::time::ServerTime;
pub use crate::response::attributes::{
    Attributes,
    AttributeValue,
    Attribute
};
use super::{User, UserAgent, ListingValue, ListingItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub id: String,
    pub steamid: SteamID,
    pub appid: u32,
    #[serde(deserialize_with = "currencies_from_form")]
    pub currencies: Currencies,
    pub value: Option<ListingValue>,
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
    pub item: ListingItem,
    pub count: u32,
    pub status: String,
    pub user_agent: Option<UserAgent>,
    pub user: Option<User>,
}