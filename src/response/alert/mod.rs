mod price;
mod min_max;

pub use min_max::MinMax;
pub use price::Price;

use serde::{Deserialize, Serialize};
use crate::{
    SteamID,
    ListingIntent,
    response::{
        deserializers::bool_from_int,
        deserializers::listing_intent_enum_from_str
    },
};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Alert {
    pub id: String,
    pub item_name: String,
    #[serde(deserialize_with = "listing_intent_enum_from_str")]
    pub intent: ListingIntent,
    pub appid: u32,
    pub steamid: SteamID,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub blanket: bool,
    pub price: Option<MinMax>,
}