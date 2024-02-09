//! An alert.

mod price;
mod min_max;

pub use min_max::MinMax;
pub use price::Price;

use crate::{SteamID, ListingIntent};
use crate::response::deserializers;
use serde::{Deserialize, Serialize};

/// An alert.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Alert {
    /// The ID of the alert.
    pub id: String,
    /// The name of the item.
    pub item_name: String,
    /// The intent of the alert.
    #[serde(deserialize_with = "deserializers::listing_intent_enum_from_str_or_int")]
    pub intent: ListingIntent,
    /// The appid of the item.
    pub appid: u32,
    /// The SteamID of the user.
    pub steamid: SteamID,
    /// The user's name.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::bool_from_int")]
    pub blanket: bool,
    /// The user's name.
    pub price: Option<MinMax>,
}