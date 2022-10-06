use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds;
use crate::{
    SteamID,
    ListingIntent,
    request,
    response::{currencies::Currencies, deserializers::listing_intent_enum_from_str},
    time::ServerTime,
};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ErrorListing<T> {
    pub message: String,
    pub query: request::UpdateListing<T>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuccessListing {
    pub id: String,
    pub appid: u32,
    pub steamid: SteamID,
    pub currencies: Currencies,
    #[serde(default)]
    pub trade_offers_preferred: bool,
    #[serde(default)]
    pub buyout_only: bool,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    pub details: Option<String>,
    pub count: u32,
    #[serde(with = "ts_seconds")]
    pub listed_at: ServerTime,
    #[serde(with = "ts_seconds")]
    pub bumped_at: ServerTime,
    #[serde(deserialize_with = "listing_intent_enum_from_str")]
    pub intent: ListingIntent,
}

pub type Result<T> = std::result::Result<SuccessListing, ErrorListing<T>>;