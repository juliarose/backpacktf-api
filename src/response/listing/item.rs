//! Listing item.

use super::attributes;
use crate::response::deserializers;
use serde::{Serialize, Deserialize};
use tf2_enum::{Wear, KillstreakTier, Killstreaker, Sheen, Quality, Paint, ItemSlot, Class, Origin};

/// An item belonging to a listing.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    /// The appid of the item.
    pub appid: u32,
    /// The item's base name without any suffixes or prefixes e.g. "Pain Train".
    pub base_name: String,
    /// The item's name on the Steam Community Market.
    pub market_name: String,
    /// The item's full name including any suffixes or prefixes e.g. "Strange Professional 
    /// Killstreak Pain Train".
    pub name: String,
    /// The classes that use this item.
    #[serde(default)]
    pub class: Vec<Class>,
    /// The item's ID. `None` if this listing is a buy order.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string")]
    /// The item's ID. `None` if this listing is a buy order.
    pub original_id: Option<u64>,
    // must be i32 to account for marketplace cross-listing SKUs such as -100 (Random Craft Hat)
    /// The item's defindex. This is not necessarily mapped to an item in the schema and can 
    /// include values for non-standard items such as "Random Craft Hat" (-100).
    pub defindex: i32,
    /// Whether the item is craftable.
    #[serde(default)]
    pub craftable: bool,
    /// Whether the item is australium.
    #[serde(default)]
    pub australium: bool,
    /// Whether the item is festivized.
    #[serde(default)]
    pub festivized: bool,
    /// Whether the item is strange.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::presence", rename = "elevatedQuality")]
    pub strange: bool,
    /// Whether the item is duped.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::presence")]
    pub dupe: bool,
    /// The item's image URL.
    pub image_url: String,
    /// The item's origin.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option")]
    pub origin: Option<Origin>,
    /// The item's slot.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<ItemSlot>,
    /// The item's description.
    pub summary: String,
    /// The item's quality.
    #[serde(deserialize_with = "deserializers::map_to_enum")]
    pub quality: Quality,
    /// The item's wear.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option", rename = "wearTier")]
    pub wear: Option<Wear>,
    /// The item's paint.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option_from_name")]
    pub paint: Option<Paint>,
    /// The item's crate series.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_series: Option<u8>,
    /// The item's killstreak tier.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option")]
    pub killstreak_tier: Option<KillstreakTier>,
    /// The item's sheen.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option")]
    pub sheen: Option<Sheen>,
    /// The item's killstreaker.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::map_to_enum_option")]
    pub killstreaker: Option<Killstreaker>,
    /// The item's particle effect.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle: Option<attributes::ParticleAttribute>,
    /// The item's texture.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<attributes::TextureAttribute>,
    /// The item's kill eaters e.g. "Kills" or "Kills While Explosive Jumping". These are not 
    /// necessarily strange parts but can be mapped to strange parts.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kill_eaters: Option<Vec<attributes::KillEaterAttribute>>,
    /// The item's recipe.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<attributes::RecipeAttribute>,
    /// The item's quantity.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string")]
    pub quantity: Option<u32>,
    /// The item's strange parts.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strange_parts: Option<Vec<attributes::KillEaterAttribute>>,
    /// The item's spells.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spells: Option<Vec<attributes::SpellAttribute>>,
}

