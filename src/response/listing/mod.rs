//! Listing responses.

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
use std::fmt;
use chrono::serde::ts_seconds;
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration as ChronoDuration};

/// A listing.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    /// The ID of the listing.
    pub id: String,
    /// The SteamID of the listing's user.
    pub steamid: SteamID,
    /// The appid of the listing.
    pub appid: u32,
    /// The currencies of the listing.
    pub currencies: ResponseCurrencies,
    /// The value of the listing.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// Whether the user prefers trade offers for this listing.
    #[serde(default)]
    #[serde(skip_serializing_if = "deserializers::is_false")]
    pub trade_offers_preferred: bool,
    /// Whether the listing is a buyout only listing.
    #[serde(default)]
    #[serde(skip_serializing_if = "deserializers::is_false")]
    pub buyout_only: bool,
    /// Whether the listing is archived.
    #[serde(default)]
    #[serde(skip_serializing_if = "deserializers::is_false")]
    pub archived: bool,
    /// The details of the listing.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// The time the listing was listed at.
    #[serde(with = "ts_seconds")]
    pub listed_at: ServerTime,
    /// The time the listing was bumped at.
    #[serde(with = "ts_seconds")]
    pub bumped_at: ServerTime,
    /// The intent of the listing.
    #[serde(deserialize_with = "deserializers::listing_intent_enum_from_str_or_int")]
    pub intent: ListingIntent,
    /// The item of the listing.
    pub item: Item,
    /// The count of the listing.
    pub count: u32,
    /// The status of the listing.
    #[serde(default)]
    pub status: Status,
    /// The user agent of the listing.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<UserAgent>,
    /// The user of the listing.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl Listing {
    /// Whether the listing is relistable by checking if the listing was listed after an interval.
    pub fn relistable(&self, interval: Duration) -> bool {
        if let Ok(interval) = ChronoDuration::from_std(interval) {
            let cutoff = Utc::now() - interval;
            
            self.listed_at < cutoff
        } else {
            false
        }
    }
    
    /// Gets the access token of the listing's user.
    pub fn access_token(&self) -> Option<String> {
        self.user.as_ref().and_then(|user| user.access_token())
    }
    
    /// Whether the listing is managed by an agent.
    pub fn is_automatic(&self) -> bool {
        self.user_agent.is_some()
    }
}

impl fmt::Display for Listing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Looks like 
        write!(
            f,
            "[{}] {} ({} for {})",
            u64::from(self.steamid),
            self.item,
            match self.intent {
                ListingIntent::Buy => "buying",
                ListingIntent::Sell => "selling",
            },
            self.currencies,
        )
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
    fn deserializes_from_serialized_listing() {
        let json_listing: Listing = serde_json::from_str(include_str!("fixtures/Strange Massed Flies Crone's Dome.json")).unwrap();
        let json = serde_json::to_string(&json_listing).unwrap();
        let listing: Listing = serde_json::from_str(&json).unwrap();
        
        assert!(json_listing.eq(&listing));
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
    
    #[test]
    fn parses_hat_currencies() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/hat_currencies.json")).unwrap();
        
        assert_eq!(listing.currencies, ResponseCurrencies::InGameWithHat {
            keys: 0.0,
            metal: 1.5,
            hat: 1.0
        });
    }
}

