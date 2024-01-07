use super::{Attributable, ItemAttribute, Item};
use serde::{Deserialize, Serialize};
use tf2_enum::Quality;

/// Parameters for an item.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ItemParams {
    /// The defindex of the item.
    pub defindex: u32,
    /// The quality of the item.
    pub quality: Quality,
    /// Whether the item is craftable.
    pub craftable: bool,
    /// The attributes of the item.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub attributes: Vec<ItemAttribute>,
}

impl From<&Item> for ItemParams {
    fn from(query: &Item) -> ItemParams {
        ItemParams {
            defindex: query.defindex,
            quality: query.quality,
            craftable: query.craftable,
            attributes: query.as_attributes(),
        }
    }
}

impl From<Item> for ItemParams {
    fn from(query: Item) -> ItemParams {
        ItemParams {
            defindex: query.defindex,
            quality: query.quality,
            craftable: query.craftable,
            attributes: query.as_attributes(),
        }
    }
}
