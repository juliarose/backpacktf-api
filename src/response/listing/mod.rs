mod user;
mod user_agent;
mod value;
mod item;
mod status;
pub mod attributes;

pub mod create_listing;
pub mod update_listing;
pub use user::{User, Ban};
pub use item::Item;
pub use value::Value;
pub use user_agent::UserAgent;
pub use status::Status;

use crate::{SteamID, ListingIntent};
use crate::time::ServerTime;
use crate::response::currencies::ResponseCurrencies;
use crate::response::deserializers;
use std::time::Duration;
use chrono::serde::ts_seconds;
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration as ChronoDuration};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub id: String,
    pub steamid: SteamID,
    pub appid: u32,
    pub currencies: ResponseCurrencies,
    pub value: Option<Value>,
    #[serde(default)]
    pub trade_offers_preferred: bool,
    #[serde(default)]
    pub buyout_only: bool,
    #[serde(default)]
    pub archived: bool,
    pub details: Option<String>,
    #[serde(with = "ts_seconds")]
    pub listed_at: ServerTime,
    #[serde(with = "ts_seconds")]
    pub bumped_at: ServerTime,
    #[serde(deserialize_with = "deserializers::listing_intent_enum_from_str")]
    pub intent: ListingIntent,
    pub item: Item,
    pub count: u32,
    #[serde(default)]
    pub status: Status,
    pub user_agent: Option<UserAgent>,
    pub user: Option<User>,
}

impl Listing {
    pub fn relistable(&self, interval: Duration) -> bool {
        if let Ok(interval) = ChronoDuration::from_std(interval) {
            let cutoff = Utc::now() - interval;
            
            self.listed_at < cutoff
        } else {
            false
        }
    }
    
    pub fn access_token(&self) -> Option<String> {
        if let Some(user) = &self.user {
            user.access_token()
        } else {
            None
        }
    }

    pub fn is_automatic(&self) -> bool {
        self.user_agent.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tf2_enum::{ItemSlot, Spell, Quality};
    
    #[test]
    fn parses_listing() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/listing.json")).unwrap();
        let strange = listing.item.strange;
        
        assert_eq!(listing.item.slot, Some(ItemSlot::Misc));
        assert_eq!(listing.item.quality, Quality::Unique);
        assert_eq!(listing.item.base_name, "Lucky Cat Hat");
        assert!(!strange);
    }
    
    #[test]
    fn parses_strange_item() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/Strange Massed Flies Crone's Dome.json")).unwrap();
        let item = listing.item;
        
        assert!(item.strange);
    }
    
    #[test]
    fn missing_paint_is_none() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/missing_paint.json")).unwrap();
        let item = listing.item;
        
        assert!(item.paint.is_none());
    }
    
    #[test]
    fn parses_websocket_listing() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/websocket_listing.json")).unwrap();
        
        assert_eq!(listing.item.particle.unwrap().name, "Disco Beat Down");
    }
    
    #[test]
    fn parses_recipe_listing() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/Collector's Shortstop Chemistry Set.json")).unwrap();
        let recipe = listing.item.recipe.unwrap();
        let output_item = recipe.output_item.unwrap();
        
        assert_eq!(output_item.quality, Quality::Collectors);
        assert_eq!(output_item.defindex, 220);
    }
    
    #[test]
    fn parses_spelled_listing() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/spelled.json")).unwrap();
        let attribute = listing.item.spells.unwrap().into_iter().next().unwrap();
        
        assert_eq!(attribute.spell, Spell::VoicesFromBelow);
    }
    
    #[test]
    fn parses_status_not_enough_currency() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/not_enough_currency.json")).unwrap();
        
        assert_eq!(listing.status, Status::NotEnoughCurrency);
    }
    
    #[test]
    fn parses_status_unknown() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/unknown_status.json")).unwrap();
        
        assert_eq!(listing.status, Status::Other("sus".into()));
    }
}

