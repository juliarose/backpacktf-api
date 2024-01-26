//! Players.

use super::deserializers;
use crate::SteamID;
use crate::time::ServerTime;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds_option;
use tf2_price::{get_metal_from_float, Currencies, Currency};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// The user's trust ratings.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Trust {
    /// How many positive ratings.
    pub r#for: u32,
    /// How many negative ratings.
    pub against: u32,
}

/// A ban.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Ban {
    /// The type of ban.
    #[serde(deserialize_with = "deserializers::string_or_number")]
    pub r#type: u32,
    /// The reason for the ban.
    pub reason: String,
    /// When the ban started.
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub start: Option<ServerTime>,
    /// When the ban ends. If the ban is permanent, this will be before the start. Use 
    /// the `permabanned` method to check if the ban is permanent.
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub end: Option<ServerTime>,
}

impl Ban {
    /// The user is permanently banned.
    pub fn permabanned(&self) -> bool {
        self.start > self.end
    }
}

/// A list of users.
pub type Players = HashMap<SteamID, Player>;
/// A list of users (v1).
pub type PlayersV1 = HashMap<SteamID, PlayerV1>;

/// The number of inventory slots.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Slots {
    /// The number of slots used.
    pub used: i32,
    /// The total number of slots.
    pub total: i32,
}

/// The user's inventory.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Inventory {
    /// The ranking of the inventory against other inventories.
    pub ranking: Option<i32>,
    /// The value of the inventory.
    #[serde(default)]
    pub value: f32,
    /// When the inventory was last updated.
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub updated: Option<ServerTime>,
    /// The amount of metal in the inventory.
    pub metal: f32,
    /// The number of keys in the inventory.
    pub keys: i32,
    /// The number of slots in the inventory.
    pub slots: Slots,
}

/// A user.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct PlayerV1 {
    /// The name of the user.
    pub name: String,
    /// A URL to the user's avatar.
    pub avatar: String,
    /// The user's inventories.
    #[serde(default)]
    pub inventory: HashMap<u32, Inventory>,
}

impl PlayerV1 {
    /// The user's currencies.
    pub fn currencies(&self) -> Currencies {
        if let Some(inventory) = &self.inventory.get(&440) {
            Currencies {
                keys: Currency::from(inventory.keys),
                metal: get_metal_from_float(inventory.metal),
            }
        } else {
            Currencies::new()
        }
    }
}

/// A user.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Player {
    /// The user's Steam ID.
    pub steamid: SteamID,
    /// The user's name.
    pub name: String,
    /// The user's backpack values.
    #[serde(default)]
    pub backpack_value: HashMap<u32, f32>,
    /// Whether the user is banned from backpack.tf.
    #[serde(default)]
    pub backpack_tf_banned: Option<Ban>,
    /// The user's backpack.tf bans.
    #[serde(default)]
    pub backpack_tf_bans: Option<HashMap<String, Ban>>,
    /// The user's backpack.tf reputation.
    #[serde(default)]
    pub backpack_tf_reputation: i32,
    /// Whether the user is in the backpack.tf group.
    #[serde(default)]
    pub backpack_tf_group: bool,
    /// The user's backpack.tf trust ratings.
    pub backpack_tf_trust: Trust,
    /// Whether the user is a SteamREP scammer.
    #[serde(default)]
    pub steamrep_scammer: bool,
}

/// The type of ban.
#[derive(Serialize_repr, Deserialize_repr, Hash, Eq, PartialEq, Clone, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum BanType {
    /// Banned from all features
    AllFeatures = 1,
    // todo fill in remaining reasons
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