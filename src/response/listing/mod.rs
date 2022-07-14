mod user;
mod user_agent;
mod value;
mod item;
pub mod attributes;

pub mod create_listing;
pub mod update_listing;
pub use user::{User, Ban};
pub use item::Item;
pub use value::Value;
pub use user_agent::UserAgent;

use serde::{Serialize, Deserialize};
use crate::{
    SteamID,
    ListingIntent,
    time::ServerTime,
    response::{currencies::Currencies, deserializers::listing_intent_enum_from_str},
};
use chrono::serde::ts_seconds;
use std::time::Duration;
use chrono::{Utc, Duration as ChronoDuration};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub id: String,
    pub steamid: SteamID,
    pub appid: u32,
    pub currencies: Currencies,
    pub value: Option<Value>,
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
    pub item: Item,
    pub count: u32,
    pub status: String,
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
    use tf2_enum::{ItemSlot, Quality};
    
    #[test]
    fn parses_listing() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/listing.json")).unwrap();
        let strange = listing.item.strange;
        
        assert_eq!(listing.item.slot, Some(ItemSlot::Misc));
        assert_eq!(listing.item.quality, Quality::Unique);
        assert_eq!(listing.item.base_name, "Lucky Cat Hat");
        assert_eq!(strange, false);
    }
    
    #[test]
    fn parses_strange_item() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/Strange Massed Flies Crone's Dome.json")).unwrap();
        let item = listing.item;
        
        assert_eq!(item.strange, true);
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
}

