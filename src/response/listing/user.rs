use serde::{Serialize, Deserialize};
use crate::response::deserializers::{deserialize_listing_bans, default_on_null};
use crate::SteamID;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    // todo fill this out
    // but you probably won't see this appear often in responses for listings
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: SteamID,
    pub name: String,
    pub avatar: String,
    pub avatar_full: String,
    #[serde(default)]
    #[serde(deserialize_with = "default_on_null")]
    pub premium: bool,
    #[serde(default)]
    #[serde(deserialize_with = "default_on_null")]
    pub online: bool,
    #[serde(default)]
    #[serde(deserialize_with = "default_on_null")]
    pub banned: bool,
    pub custom_name_style: String,
    pub accepted_suggestions: u32,
    pub class: String,
    pub style: String,
    pub trade_offer_url: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "default_on_null")]
    pub is_marketplace_seller: bool,
    #[serde(default)]
    #[serde(deserialize_with = "default_on_null")]
    pub flag_impersonated: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_listing_bans")]
    pub bans: Vec<Ban>,
}

impl User {
    pub fn access_token(&self) -> Option<String> {
        if let Some(trade_offer_url) = &self.trade_offer_url {
            if let Some(index) = trade_offer_url.find("token=") {
                let start = index + 6;
                // always 8 chars
                let slice = &trade_offer_url[start..(start + 8)];
                
                return Some(slice.into());
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_listing() {
        let response: User = serde_json::from_str(include_str!("fixtures/user.json")).unwrap();
        let token = response.access_token();
        
        assert_eq!(token, Some("iF6QGWOa".into()));
    }
}

