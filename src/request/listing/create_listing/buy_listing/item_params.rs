use serde::{Deserialize, Serialize};
use super::{Attributable, ItemAttribute, Item};
use tf2_enum::Quality;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ItemParams {
    pub defindex: u32,
    pub quality: Quality,
    pub craftable: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub attributes: Vec<ItemAttribute>,
}

impl From<&Item> for ItemParams {
    fn from(query: &Item) -> ItemParams {
        ItemParams {
            defindex: query.defindex,
            quality: query.quality.clone(),
            craftable: query.craftable,
            attributes: query.as_attributes(),
        }
    }
}

impl From<Item> for ItemParams {
    fn from(query: Item) -> ItemParams {
        ItemParams {
            defindex: query.defindex,
            quality: query.quality.clone(),
            craftable: query.craftable,
            attributes: query.as_attributes(),
        }
    }
}
