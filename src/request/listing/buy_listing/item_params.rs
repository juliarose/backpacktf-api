use serde::{Deserialize, Serialize};
use super::{Attributable, BuyListingItemAttribute, BuyListingItem};
use tf2_enums::Quality;

// use num_enum::IntoPrimitive;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemParams {
    pub defindex: u32,
    pub quality: Quality,
    pub craftable: bool,
    pub attributes: Vec<BuyListingItemAttribute>,
}

// todo combine these borrowed/unborrowed implementations somehow...
impl From<&BuyListingItem> for ItemParams {
    fn from(query: &BuyListingItem) -> ItemParams {
        ItemParams {
            defindex: query.defindex,
            quality: query.quality.clone(),
            craftable: query.craftable,
            attributes: query.as_attributes(),
        }
    }
}

impl From<BuyListingItem> for ItemParams {
    fn from(query: BuyListingItem) -> ItemParams {
        ItemParams {
            defindex: query.defindex,
            quality: query.quality.clone(),
            craftable: query.craftable,
            attributes: query.as_attributes(),
        }
    }
}
