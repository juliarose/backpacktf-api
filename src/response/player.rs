use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{
    SteamID,
    time::ServerTime,
    response::deserializers::bool_from_int
};
use chrono::serde::ts_seconds_option;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Trust {
    pub r#for: u32,
    pub against: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Ban {
    pub r#type: u32,
    pub reason: String,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub start: Option<ServerTime>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub end: Option<ServerTime>,
}

impl Ban {
    pub fn permabanned(&self) -> bool {
        self.start > self.end
    }
}

pub type Players = HashMap<SteamID, Player>;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Player {
    pub steamid: SteamID,
    #[serde(deserialize_with = "bool_from_int")]
    pub success: bool,
    pub name: String,
    pub backpack_value: HashMap<u32, f32>,
    pub backpack_tf_banned: Option<Ban>,
    pub backpack_tf_reputation: i32,
    #[serde(default)]
    pub backpack_tf_group: bool,
    pub backpack_tf_trust: Trust
}