pub mod buy_listing;

use buy_listing::serializers::buy_listing_item_into_params;

use crate::request::serializers::as_string;
use serde::{Deserialize, Serialize};

/// Parameters for creating a listing.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(tag = "intent")]
pub enum CreateListing<T> {
    /// Parameters for creating a sell listing.
    #[serde(rename = "sell")]
    Sell {
        /// The id of the item to list.
        #[serde(serialize_with = "as_string")]
        id: u64,
        /// The currencies. Any type that can be serialized can be supplied. It should contain
        /// "keys" and/or "metal" fields.
        currencies: T,
        /// The message of the listing.
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        /// Whether buyout is enabled.
        buyout: bool,
        /// Whether offers are enabled.
        offers: bool,
    },
    /// Parameters for creating a buy listing.
    #[serde(rename = "buy")]
    Buy {
        /// The item to list.
        #[serde(serialize_with = "buy_listing_item_into_params")]
        item: buy_listing::Item,
        /// The currencies. Any type that can be serialized can be supplied. It should contain
        /// "keys" and/or "metal" fields.
        currencies: T,
        /// The message of the listing.
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        /// Whether buyout is enabled.
        buyout: bool,
        /// Whether offers are enabled.
        offers: bool,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tf2_price::{Currencies, ref_to_weps};
    use tf2_enum::{KillstreakTier, Quality, Spell, SpellSet, StrangePart, StrangePartSet};
    use assert_json_diff::assert_json_eq;
    use serde_json::{self, json, Value};
    
    #[test]
    fn serializes_correctly() {
        let json = serde_json::to_string(&CreateListing::Buy {
            item: buy_listing::Item::new(1, Quality::Unusual),
            details: Some("hello".into()),
            currencies: Currencies {
                keys: 5,
                weapons: ref_to_weps!(5),
            },
            buyout: false,
            offers: false,
        }).unwrap();
        let actual: Value = serde_json::from_str(&json).unwrap();
        let expected = json!({
            "intent": "buy",
            "item": {
                "defindex": 1,
                "quality": 5,
                "craftable": true
            },
            "currencies": {
                "keys": 5,
                "metal": 5
            },
            "details": "hello",
            "buyout": false,
            "offers": false
        });
        
        assert_json_eq!(
            actual,
            expected,
        );
    }
    
    #[test]
    fn serializes_attributes_correctly() {
        let item = buy_listing::Item::new(10, Quality::Strange)
            .killstreak_tier(KillstreakTier::Professional)
            .spells(SpellSet::double(
                Spell::DieJob,
                Spell::HeadlessHorseshoes,
            ))
            .strange_parts(StrangePartSet::single(
                StrangePart::TauntKills,
            ));
        let listing = CreateListing::Buy {
            item,
            details: Some("hello".into()),
            currencies: Currencies {
                keys: 5,
                weapons: ref_to_weps!(5.33),
            },
            buyout: false,
            offers: false,
        };
        let json = serde_json::to_string(&listing).unwrap();
        let actual: Value = serde_json::from_str(&json).unwrap();
        let expected = json!({
            "intent": "buy",
            "item": {
                "defindex": 10,
                "quality": 11,
                "craftable": true,
                "attributes": [
                    {
                        "defindex": 2025,
                        "float_value": 3,
                    },
                    {
                        "defindex":1004,
                        "float_value": 0
                    },
                    {
                        "defindex":1005,
                        "float_value": 2
                    },
                    {
                        "defindex": 379,
                        "value": 0
                    },
                    {
                        "defindex": 380,
                        "float_value": 77
                    }
                ]
            },
            "currencies": {
                "keys": 5,
                "metal": 5.33
            },
            "details": "hello",
            "buyout": false,
            "offers": false
        });
        
        assert_json_eq!(
            actual,
            expected,
        );
    }
}