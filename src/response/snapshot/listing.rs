use serde::{Serialize, Deserialize};
use crate::ListingIntent;
use crate::response::attributes::{AttributeValue};
use crate::response::deserializers::{
    bool_from_int,
    listing_intent_enum_from_str
};
use tf2_price::Currencies;
use chrono::serde::{ts_seconds};
use crate::time::ServerTime;
use steamid_ng::SteamID;
use super::Item;

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
}

impl Listing {
    
    pub fn is_buy_order(&self) -> bool {
        self.intent == ListingIntent::Buy
    }
    
    pub fn is_sell_order(&self) -> bool {
        self.intent == ListingIntent::Sell
    }
    
    pub fn get_particle_value(&self) -> Option<u64> {
        if let Some(particle_attribute) = self.item.attributes.get(&134) {
            if let Some(value) = &particle_attribute.float_value {
                match value {
                    AttributeValue::Number(particle_value) => Some(*particle_value),
                    _ => None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}