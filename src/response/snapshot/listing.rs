use serde::{Serialize, Deserialize};
use crate::{
    ListingIntent,
    time::ServerTime,
    response::attributes::Value as AttributeValue,
    response::deserializers::{
        bool_from_int,
        listing_intent_enum_from_str
    },
};
use super::Item;
use tf2_price::Currencies;
use tf2_enum::{Wear, Quality, KillstreakTier};
use chrono::serde::ts_seconds;
use steamid_ng::SteamID;

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
    
    pub fn get_quality(&self) -> Quality {
        self.item.quality.clone()
    }
    
    pub fn get_particle_value(&self) -> Option<u32> {
        if let Some(attribute) = self.item.attributes.get(&134) {
            if let Some(float_value) = &attribute.float_value {
                return Some(*float_value as u32);
            }
        }
        
        None
    }
    
    pub fn get_skin_value(&self) -> Option<u32> {
        if let Some(attribute) = self.item.attributes.get(&834) {
            if let Some(value) = &attribute.value {
                if let AttributeValue::Number(value) = value {
                    return Some(*value as u32);
                }
            }
        }
        
        None
    }
    
    pub fn get_killstreak_tier(&self) -> Option<KillstreakTier> {
        if let Some(attribute) = self.item.attributes.get(&2025) {
            if let Some(float_value) = &attribute.float_value {
                if let Ok(killstreak_tier) = KillstreakTier::try_from(*float_value as u8) {
                    return Some(killstreak_tier);
                }
            }
        }
        
        None
    }
    
    pub fn get_wear(&self) -> Option<Wear> {
        if let Some(attribute) = self.item.attributes.get(&725) {
            if let Some(float_value) = &attribute.float_value {
                let wear_value = (float_value * 5.0).round() as u8;
                
                if let Ok(wear) = Wear::try_from(wear_value) {
                    return Some(wear);
                }
            }
        }
        
        None
    }
    
    pub fn is_festive(&self) -> bool {
        self.item.attributes.contains_key(&2053)
    }
}