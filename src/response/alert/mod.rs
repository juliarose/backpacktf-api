mod price;
mod min_max;

pub use min_max::MinMax;
pub use price::Price;

use crate::{SteamID, ListingIntent};
use crate::response::deserializers;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Alert {
    pub id: String,
    pub item_name: String,
    #[serde(deserialize_with = "deserializers::listing_intent_enum_from_str_or_int")]
    pub intent: ListingIntent,
    pub appid: u32,
    pub steamid: SteamID,
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::bool_from_int")]
    pub blanket: bool,
    pub price: Option<MinMax>,
}