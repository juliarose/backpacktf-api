use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{SteamID, time::ServerTime, tf2_price::{get_metal_from_float, Currencies}};
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
pub type PlayersV1 = HashMap<SteamID, PlayerV1>;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Slots {
    pub used: i32,
    pub total: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Inventory {
    pub ranking: Option<i32>,
    #[serde(default)]
    pub value: f32,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub updated: Option<ServerTime>,
    pub metal: f32,
    pub keys: i32,
    pub slots: Slots,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct PlayerV1 {
    pub name: String,
    pub avatar: String,
    #[serde(default)]
    pub inventory: HashMap<u32, Inventory>,
}

impl PlayerV1 {
    
    pub fn currencies(&self) -> Currencies {
        if let Some(inventory) = &self.inventory.get(&440) {
            let metal = get_metal_from_float(inventory.metal);
            
            Currencies {
            keys: inventory.keys,
            metal,
            }
        } else {
            Currencies::new()
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Player {
    pub steamid: SteamID,
    pub name: String,
    #[serde(default)]
    pub backpack_value: HashMap<u32, f32>,
    #[serde(default)]
    pub backpack_tf_banned: Option<Ban>,
    #[serde(default)]
    pub backpack_tf_bans: Option<HashMap<String, Ban>>,
    #[serde(default)]
    pub backpack_tf_reputation: i32,
    #[serde(default)]
    pub backpack_tf_group: bool,
    pub backpack_tf_trust: Trust,
    #[serde(default)]
    pub steamrep_scammer: bool,
}

// todo fill in remaining reasons
#[derive(Serialize_repr, Deserialize_repr, Hash, Eq, PartialEq, Clone, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum BanType {
    AllFeatures = 1,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parses_ban() {
        let player: Player = serde_json::from_str(include_str!("fixtures/player.json")).unwrap();
        
        assert!(player.backpack_tf_banned.unwrap().permabanned());
        assert!(player.steamrep_scammer);
    }
}