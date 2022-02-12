use serde::{Serialize, Deserialize};
use tf2_enums::Quality;
use crate::response::attributes::{Attributes};
use crate::response::deserializers::{
    deserialize_attributes,
    from_optional_number_or_string
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub defindex: u32,
    pub quality: Quality,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub original_id: Option<u64>,
    pub level: Option<u8>,
    pub inventory: Option<u32>,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub quantity: Option<u32>,
    pub origin: Option<u32>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_attributes")]
    pub attributes: Attributes,
}