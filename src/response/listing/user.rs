use serde::{Serialize, Deserialize};
use crate::response::deserializers::default_on_null;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserBan {
    // todo fill this out
    // but you probably won't see this appear often in responses for listings
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub avatar_full: String,
    pub premium: bool,
    pub online: bool,
    pub banned: bool,
    pub custom_name_style: String,
    pub accepted_suggestions: u32,
    pub class: String,
    pub style: String,
    pub trade_offer_url: String,
    pub is_marketplace_seller: bool,
    #[serde(default)]
    #[serde(deserialize_with = "default_on_null")]
    pub flag_impersonated: bool,
    pub bans: Vec<UserBan>,
}