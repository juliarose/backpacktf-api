use serde::{Serialize, Deserialize};
use tf2_enum::{StrangePart, Rarity};
use crate::response::deserializers::{
    from_number_or_string,
    map_to_enum,
};
use super::Item;

/// Represents a particle effect.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticleAttribute {
    pub id: u32,
    pub name: String,
    pub short_name: String,
    pub image_url: String,
    pub r#type: String,
}

/// Represents a texture.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextureAttribute {
    pub id: u32,
    pub item_defindex: Option<u32>,
    #[serde(deserialize_with = "map_to_enum")]
    pub rarity: Rarity,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KillEaterTypeAttribute {
    pub id: Option<u32>,
    pub name: String,
}

impl KillEaterTypeAttribute {
    pub fn get_strange_part(&self) -> Option<StrangePart> {
        let id = self.id.unwrap_or_default();
        
        if let Ok(strange_part) = StrangePart::try_from(id) {
            return Some(strange_part);
        }
        
        None
    }
}

/// Represents a kill eater attribute which includes a score and score type. This can be used
/// to determine strange parts.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KillEaterAttribute {
    pub score: u64,
    pub kill_eater: KillEaterTypeAttribute,
}

impl KillEaterAttribute {
    /// If the score type correlates to a strange part, gets the strange part for this kill eater 
    /// attribute.
    pub fn get_strange_part(&self) -> Option<StrangePart> {
        self.kill_eater.get_strange_part()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RecipeInputItem {
    #[serde(deserialize_with = "from_number_or_string")]
    pub quantity: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ItemSource {
    pub defindex: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetItem {
    pub item_name: String,
    #[serde(rename = "_source")]
    pub source: ItemSource,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecipeAttribute {
    pub input_items: Vec<RecipeInputItem>,
    #[serde(default)]
    pub target_item: Option<TargetItem>,
    #[serde(default)]
    pub output_item: Option<Box<Item>>,
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