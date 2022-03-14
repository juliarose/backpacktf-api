use serde::{Serialize, Deserialize};
use steamid_ng::SteamID;
use crate::{
    ListingIntent,
    time::ServerTime,
    request::currencies::Currencies,
    response::deserializers::listing_intent_enum_from_str,
};
use chrono::serde::ts_seconds;
use super::{User, UserAgent, Value, Item};

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

use std::time::Duration;
use chrono::{Utc, Duration as ChronoDuration};

impl Listing {
    
    pub fn relistable(&self, interval: Duration) -> bool {
        if let Ok(interval) = ChronoDuration::from_std(interval) {
            let cutoff = Utc::now() - interval;
            
            self.listed_at < cutoff
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tf2_enum::Quality;

    #[test]
    fn parses_listing() {
        let response: Listing = serde_json::from_str(include_str!("fixtures/listing.json")).unwrap();
        let strange = response.item.strange;
        
        assert_eq!(response.item.quality, Quality::Unique);
        assert_eq!(response.item.base_name, "Lucky Cat Hat");
        assert_eq!(strange, false);
    }
}