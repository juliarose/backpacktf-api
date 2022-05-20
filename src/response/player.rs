use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{SteamID, time::ServerTime};
use chrono::serde::ts_seconds_option;
use super::deserializers::string_or_number;
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Trust {
    pub r#for: u32,
    pub against: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Ban {
    #[serde(deserialize_with = "string_or_number")]
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
    pub name: String,
    pub backpack_value: HashMap<u32, f32>,
    pub backpack_tf_banned: Option<Ban>,
    pub backpack_tf_bans: Option<HashMap<String, Ban>>,
    pub backpack_tf_reputation: i32,
    #[serde(default)]
    pub backpack_tf_group: bool,
    pub backpack_tf_trust: Trust,
    #[serde(default)]
    pub steamrep_scammer: bool,
}

#[derive(Serialize_repr, Deserialize_repr, Hash, Eq, PartialEq, Clone, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum BanType {
    AllFeatures = 1,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parses_ban() {
        let player: Player = serde_json::from_str(include_str!("fixtures/player.json")).unwrap();
        
        assert_eq!(player.backpack_tf_banned.unwrap().permabanned(), true);
        assert_eq!(player.steamrep_scammer, true);
    }
}