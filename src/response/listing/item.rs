use serde::{Serialize, Deserialize};
use crate::response::deserializers::{
    from_optional_number_or_string,
    map_to_enum,
    map_to_enum_option,
    map_to_enum_option_from_name,
    presence,
};
use super::attributes;
use tf2_enum::{Wear, KillstreakTier, Quality, Paint};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub appid: u32,
    pub base_name: String,
    pub market_name: String,
    pub name: String,
    pub class: Option<Vec<String>>,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub original_id: Option<u64>,
    pub defindex: u32,
    #[serde(default)]
    pub craftable: bool,
    #[serde(default)]
    pub australium: bool,
    #[serde(default)]
    pub festivized: bool,
    #[serde(default)]
    #[serde(deserialize_with = "presence", rename = "elevated_quality")]
    pub strange: bool,
    pub image_url: String,
    pub slot: String,
    pub summary: String,
    #[serde(deserialize_with = "map_to_enum")]
    pub quality: Quality,
    #[serde(default)]
    #[serde(deserialize_with = "map_to_enum_option", rename = "wear_tier")]
    pub wear: Option<Wear>,
    #[serde(default)]
    #[serde(deserialize_with = "map_to_enum_option_from_name")]
    pub paint: Option<Paint>,
    #[serde(default)]
    #[serde(deserialize_with = "map_to_enum_option")]
    pub killstreak_tier: Option<KillstreakTier>,
    pub particle: Option<attributes::ParticleAttribute>,
    pub texture: Option<attributes::TextureAttribute>,
    pub kill_eaters: Option<Vec<attributes::KillEaterAttribute>>,
}