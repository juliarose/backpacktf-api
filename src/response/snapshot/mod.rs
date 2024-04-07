//! Snapshot.

mod listing;
mod item;

pub use item::Item;
pub use listing::Listing;

use crate::time::ServerTime;
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;

/// A snapshot.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    /// The time the snapshot was taken at.
    #[serde(with = "ts_seconds")]
    pub created_at: ServerTime,
    /// The appid of the snapshot.
    pub appid: Option<u32>,
    /// The SKU of the snapshot.
    pub sku: String,
    /// The listings in the snapshot.
    #[serde(default)]
    pub listings: Vec<Listing>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ListingIntent,
        tf2_enum::{Quality, Killstreaker, Sheen, StrangePart, Origin},
        response::attributes::Value as AttributeValue,
        response::currencies::ResponseCurrencies,
    };
    use tf2_price::FloatCurrencies;
    
    #[test]
    fn parses_get_classifieds_snapshot_quality() {
        let snapshot: Snapshot = serde_json::from_str(include_str!("fixtures/snapshot.json")).unwrap();
        let listing = snapshot.listings.first().unwrap();
        
        assert_eq!(listing.intent, ListingIntent::Sell);
        assert_eq!(listing.item.id, Some(10080129222));
        assert_eq!(listing.item.quality, Quality::Unusual);
        assert_eq!(listing.item.origin, Some(Origin::FoundInCrate));
        assert_eq!(listing.currencies, ResponseCurrencies::InGame(FloatCurrencies { keys: 180.0, metal: 0.0 }));
    }
    
    #[test]
    fn parses_get_classifieds_snapshot_australium_rocket_launcher() {
        let snapshot: Snapshot = serde_json::from_str(include_str!("fixtures/snapshot_strange_professional_killstreak_australium_rocket_launcher.json")).unwrap();
        let listing = snapshot.listings.iter().find(|listing| listing.item.id == Some(11459331376)).unwrap();
        
        assert_eq!(listing.intent, ListingIntent::Sell);
        assert_eq!(listing.item.get_killstreaker().unwrap(), Killstreaker::CerebralDischarge);
        assert_eq!(listing.item.get_sheen().unwrap(), Sheen::Manndarin);
        assert_eq!(listing.item.get_strange_parts().unwrap().len(), 2);
        assert!(listing.item.get_strange_parts().unwrap().into_iter().any(|strange_part| strange_part == StrangePart::GibKills));
    }
    
    #[test]
    fn parses_marketplace_listing() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/snapshot_listing_marketplace.json")).unwrap();
        
        if let ResponseCurrencies::Cash(currencies) = listing.currencies {
            assert_eq!(currencies, 89.99);
        } else {
            panic!("Currencies are not cash");
        }
    }
    
    #[test]
    fn parses_get_classifieds_snapshot_float_attribute() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/snapshot_listing_float_attribute.json")).unwrap();
        let attribute = listing.item.attributes.get(&725).unwrap();
        
        assert!(matches!(attribute.value.as_ref().unwrap(), AttributeValue::Float(_)));
    }
    
    #[test]
    fn parses_listings() {
        let listing: Listing = serde_json::from_str(include_str!("fixtures/snapshot_listing_string_attributes.json")).unwrap();
        
        assert!(listing.is_automatic());
        
        let intent = listing.intent;
        let attribute_380 = listing.item.attributes.get(&380).unwrap();
        let attribute_383 = listing.item.attributes.get(&383).unwrap();
        
        assert_eq!(intent, ListingIntent::Buy);
        assert_eq!(listing.item.quality, Quality::Strange);
        
        if let ResponseCurrencies::InGame(currencies) = listing.currencies {
            assert_eq!(currencies.keys, 44.0);
            assert_eq!(currencies.metal, 0.0);
        } else {
            panic!("Currencies are not in-game");
        }
        
        assert_eq!(listing.details, Some("Looking for a spelled exorcism strange frying pan (without parts is ok) ! Feel free to add me :D\ncan also buy a strange pan with these parts for 44 keys".into()));
        assert_eq!(attribute_380.float_value.unwrap(), 82.0);
        assert_eq!(attribute_383.value.as_ref().unwrap(), &AttributeValue::Number(0));
    }
}