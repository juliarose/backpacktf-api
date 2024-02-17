//! Listing user.

use crate::SteamID;
use crate::response::deserializers;
use serde::{Serialize, Deserialize};
use url::Url;
use std::borrow::Cow;

/// A ban.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    // todo fill this out
    // you probably won't see this appear often in responses for listings
}

/// A user.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The user's SteamID.
    pub id: SteamID,
    /// The user's name.
    pub name: String,
    /// The user's avatar URL.
    pub avatar: String,
    /// The user's full avatar URL.
    pub avatar_full: String,
    /// Whether the user is a premium user.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::default_on_null")]
    pub premium: bool,
    /// Whether the user is online.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::default_on_null")]
    pub online: bool,
    /// Whether the user is banned.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::default_on_null")]
    pub banned: bool,
    /// The user's custom name style.
    pub custom_name_style: String,
    /// The user's number of accepted price suggestions.
    pub accepted_suggestions: u32,
    /// The user's class.
    pub class: String,
    /// The user's style.
    pub style: String,
    /// The user's trade offer URL, if set.
    pub trade_offer_url: Option<String>,
    /// The user's backpack URL.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::default_on_null")]
    pub is_marketplace_seller: bool,
    /// Whether the user is impersonated.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::default_on_null")]
    pub flag_impersonated: bool,
    /// The user's bans.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::deserialize_listing_bans")]
    pub bans: Vec<Ban>,
}

impl User {
    /// Returns the user's access token.
    pub fn access_token(&self) -> Option<String> {
        let trade_offer_url = self.trade_offer_url.as_ref()?;
        let url = Url::parse(trade_offer_url).ok()?;
            
        for (key, value) in url.query_pairs() {
            if key == Cow::Borrowed("token") {
                if value.len() == 8 {
                    return Some(value.to_string());
                }
                
                // not a valid token
                return None;
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

