use super::attributes;
use crate::response::deserializers;
use serde::{Serialize, Deserialize};
use tf2_enum::{Wear, KillstreakTier, Killstreaker, Sheen, Quality, Paint, ItemSlot, Class, Origin};

/// An item.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub appid: u32,
    pub base_name: String,
    pub market_name: String,
    pub name: String,
    #[serde(default)]
    pub class: Vec<Class>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string")]
    pub original_id: Option<u64>,
    // must be i32 to account for marketplace cross-listing SKUs such as -100 (Random Craft Hat)
    pub defindex: i32,
    #[serde(default)]
    pub craftable: bool,
    #[serde(default)]
    pub australium: bool,
    #[serde(default)]
    pub festivized: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::presence", rename = "elevatedQuality")]
    pub strange: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::presence")]
    pub dupe: bool,
    pub image_url: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option")]
    pub origin: Option<Origin>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<ItemSlot>,
    pub summary: String,
    #[serde(deserialize_with = "deserializers::map_to_enum")]
    pub quality: Quality,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option", rename = "wearTier")]
    pub wear: Option<Wear>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option_from_name")]
    pub paint: Option<Paint>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_series: Option<u8>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option")]
    pub killstreak_tier: Option<KillstreakTier>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option")]
    pub sheen: Option<Sheen>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option")]
    pub killstreaker: Option<Killstreaker>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle: Option<attributes::ParticleAttribute>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<attributes::TextureAttribute>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kill_eaters: Option<Vec<attributes::KillEaterAttribute>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<attributes::RecipeAttribute>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string")]
    pub quantity: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strange_parts: Option<Vec<attributes::KillEaterAttribute>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spells: Option<Vec<attributes::SpellAttribute>>,
}

