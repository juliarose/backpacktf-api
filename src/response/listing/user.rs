use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserBan {
    // todo fill this out
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    // todo define this
    // pub flag_impersonated: bool,
    pub bans: Vec<UserBan>,
}