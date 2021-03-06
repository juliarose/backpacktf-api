pub mod buy_listing;

use serde::{Deserialize, Serialize};
use crate::request::serializers::as_string;
use buy_listing::serializers::buy_listing_item_into_params;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(tag = "intent")]
pub enum CreateListing<T> {
    #[serde(rename = "sell")]
    Sell {
        #[serde(serialize_with = "as_string")]
        id: u64,
        currencies: T,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        buyout: bool,
        offers: bool,
    },
    #[serde(rename = "buy")]
    Buy {
        #[serde(serialize_with = "buy_listing_item_into_params")]
        item: buy_listing::Item,
        currencies: T,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        buyout: bool,
        offers: bool,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tf2_price::{Currencies, refined, scrap};
    use tf2_enum::{Quality, KillstreakTier};
    use assert_json_diff::assert_json_include;
    use serde_json::{self, json, Value};
    
    #[test]
    fn serializes_correctly() {
        let json = serde_json::to_string(&CreateListing::Buy {
            item: buy_listing::Item::new(1, Quality::Unusual),
            details: Some("hello".into()),
            currencies: Currencies {
                keys: 5,
                metal: refined!(5),
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
        
        assert_json_include!(
            actual: actual,
            expected: expected,
        );
    }
    
    #[test]
    fn serializes_attributes_correctly() {
        let json = serde_json::to_string(&CreateListing::Buy {
            item: buy_listing::Item {
                defindex: 10,
                quality: Quality::Strange,
                craftable: true,
                killstreak_tier: Some(KillstreakTier::Professional),
                particle: None,
                wear: None,
                skin: None,
                strange: false,
                festivized: false,
                australium: false,
            },
            details: Some("hello".into()),
            currencies: Currencies {
                keys: 5,
                metal: refined!(5) + scrap!(3),
            },
            buyout: false,
            offers: false,
        }).unwrap();
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
        
        assert_json_include!(
            actual: actual,
            expected: expected,
        );
    }
}