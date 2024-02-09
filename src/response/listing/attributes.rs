//! Listing attributes.

use super::Item;
use crate::response::serializers::to_display;
use crate::response::deserializers;
use serde::{Serialize, Deserialize};
use tf2_enum::{StrangePart, Grade, Spell};

/// Represents a particle effect.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticleAttribute {
    /// The ID of the particle effect.
    pub id: u32,
    /// The name of the particle effect.
    pub name: String,
    /// The short name of the particle effect.
    pub short_name: String,
    /// The image URL of the particle effect.
    pub image_url: String,
    /// The type of the particle effect.
    pub r#type: String,
}

/// Represents a texture.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextureAttribute {
    /// The ID of the texture.
    pub id: u32,
    /// The item defindex of the war paint (if this item originated from a war paint).
    pub item_defindex: Option<u32>,
    /// The name of the texture.
    #[serde(deserialize_with = "deserializers::map_to_enum")]
    pub rarity: Grade,
    /// The image URL of the texture.
    pub name: String,
}

/// Represents a kill eater type.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KillEaterTypeAttribute {
    /// The ID of the kill eater type.
    pub id: Option<u32>,
    /// The name of the kill eater type.
    pub name: String,
}

impl KillEaterTypeAttribute {
    /// If the ID correlates to a strange part, gets the strange part for this kill eater type.
    pub fn get_strange_part(&self) -> Option<StrangePart> {
        StrangePart::try_from(self.id?).ok()
    }
}

/// Represents a kill eater attribute which includes a score and score type. This can be used
/// to determine strange parts.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KillEaterAttribute {
    /// The score of the kill eater attribute.
    pub score: u64,
    /// The kill eater type of the kill eater attribute.
    pub kill_eater: KillEaterTypeAttribute,
}

impl KillEaterAttribute {
    /// If the score type correlates to a strange part, gets the strange part for this kill eater 
    /// attribute.
    pub fn get_strange_part(&self) -> Option<StrangePart> {
        self.kill_eater.get_strange_part()
    }
}

/// Represents a recipe input item.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RecipeInputItem {
    /// The quantity of the input item.
    #[serde(deserialize_with = "deserializers::from_number_or_string")]
    pub quantity: u32,
    /// The name of the input item.
    pub name: String,
}

/// Represents an item source.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ItemSource {
    /// The defindex of the item source.
    pub defindex: u32,
}

/// Represents a target item.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetItem {
    /// The name of the target item.
    pub item_name: String,
    /// The source of the target item.
    #[serde(rename = "_source")]
    pub source: ItemSource,
}

/// Represents a recipe attribute.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecipeAttribute {
    /// The recipe input items.
    pub input_items: Vec<RecipeInputItem>,
    /// The target item.
    #[serde(default)]
    pub target_item: Option<TargetItem>,
    /// The output item.
    #[serde(default)]
    pub output_item: Option<Box<Item>>,
}

/// Represents a spell attribute.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpellAttribute {
    /// The spell.
    #[serde(rename = "name")]
    #[serde(serialize_with = "to_display")]
    #[serde(deserialize_with = "deserializers::from_str")]
    pub spell: Spell,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn kill_eater_into_strange_part() {
        let attribute = KillEaterTypeAttribute {
            id: Some(17),
            name: "Engineers Killed".into()
        };
        
        assert_eq!(attribute.get_strange_part(), Some(StrangePart::EngineersKilled));
    }
}