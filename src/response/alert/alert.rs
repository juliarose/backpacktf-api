use serde::{Deserialize, Serialize};
use crate::response::deserializers::{
    listing_intent_enum_from_str
};
use crate::response::deserializers::bool_from_int;
use crate::ListingIntent;
use steamid_ng::SteamID;
use super::Price;

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
    pub price: Option<Price>,
}