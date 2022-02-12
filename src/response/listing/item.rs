use serde::{Serialize, Deserialize};
use crate::response::deserializers::{
    from_optional_number_or_string
};
use super::{ListingSummary, attributes::*};

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub craftable: bool,
    pub tradable: bool,
    pub australium: bool,
    pub festivized: bool,
    pub image_url: String,
    pub priceindex: String,
    pub slot: String,
    pub summary: ListingSummary,
    pub quality: QualityAttribute,
    pub particle: Option<ParticleAttribute>,
    pub paint: Option<PaintAttribute>,
    pub wear_tier: Option<WearTierAttribute>,
    pub texture: Option<TextureAttribute>,
    pub elevated_quality: Option<QualityAttribute>,
    pub kill_eaters: Option<Vec<KillEaterAttribute>>,
}