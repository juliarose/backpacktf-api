//! Update listing.
//! 
use crate::{SteamID, ListingIntent};
use crate::request;
use crate::response::currencies::ResponseCurrencies;
use crate::response::deserializers;
use crate::time::ServerTime;
use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds;

/// An error occurred when updating a listing.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ErrorListing<T> {
    /// The message in the response.
    pub message: String,
    /// The query used to create the listing.
    pub query: request::UpdateListing<T>,
}

/// A listing was successfully created.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuccessListing {
    /// The ID of the listing.
    pub id: String,
    /// The app ID of the item.
    pub appid: u32,
    /// The SteamID of the user.
    pub steamid: SteamID,
    /// The currencies of the listing.
    pub currencies: ResponseCurrencies,
    /// Whether the listing prefers trade offers.
    #[serde(default)]
    pub trade_offers_preferred: bool,
    /// Whether the listing has buyout only.
    #[serde(default)]
    pub buyout_only: bool,
    /// Whether the listing is archived.
    #[serde(default)]
    pub archived: bool,
    /// Details of the listing.
    #[serde(default)]
    pub details: Option<String>,
    /// The count of the listing.
    pub count: u32,
    /// The time the listing was listed at.
    #[serde(with = "ts_seconds")]
    pub listed_at: ServerTime,
    /// The time the listing was last bumped at.
    #[serde(with = "ts_seconds")]
    pub bumped_at: ServerTime,
    /// The intent of the listing.
    #[serde(deserialize_with = "deserializers::listing_intent_enum_from_str_or_int")]
    pub intent: ListingIntent,
}

/// The result of updating a listing.
pub type Result<T> = std::result::Result<SuccessListing, ErrorListing<T>>;